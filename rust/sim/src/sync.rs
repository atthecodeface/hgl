//! The main thread initially owns the BarrierState
//!
//! Barrier::new();
//!
//! The main thread invokes a spawn method on a thread, which adds itself to the barrier and clones it
//!
//! <tn>.start -> { Barrier::add(); spawn <tn> }
//!
//! All clients added to the barrier
//!
//! <tn>.start -> { Barrier::add(); spawn <tn> }
//! SimulationWorkers can invoke 'wait_for_edge<F:Send +
//! FnMut(SimEdgeMask) -> SimWaitResult>'; this will wait on a
//! sync::Condvar until the simulation is stopped or the relevant edge
//! occurs with the condition set
//!
//! Maybe have the worker set its filter with Arc<Mutex<Box<dyn FnMut>>>
//!
//! The main simulation thread calculates when the next clock edge is
//! to occur, and it moves time forward to that point; it calculates
//! the SimEdgeMask for that time (more than one clock edge might
//! occur); all SimulationWorkers must at this point be waiting for an edge.
//!
//! The conditions for all the SimulationWorkers are evaluated before
//! releasing the sync::Condvar's; the main thread has a thread
//! barrier and it increments the number of workers for each
//! released. When the workers return (or drop!) the increment the
//! thread barrier count, and the main thread can continue execution
//! when *it* and the worker's are all ready
//!
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarrierWaitResult {
    Started,
    Stuff,
    Finish,
}

struct BarrierState {
    /// Total number of threads in the barrier
    ///
    /// This must only increase under control of a thread
    /// participating in the barrier
    ///
    /// This must only decrease under control of a thread
    /// participating in the barrier; if this thread would have been
    /// the last to wait (which it will no longer do) then it must
    /// behave as if it was waiting
    num_threads: usize,

    /// This is permitted to be set by the main thread, from false to
    /// true; it is captured by the last thread to wait on
    /// the barrier in 'finish'.
    finish_next_gen: bool,

    /// If asserted, the next result for *all* threads on completing
    /// the barrier is 'Finish'
    ///
    /// This is copied from Barrier::finish_next_gen when the last
    /// thread reaches the barrier
    finish: bool,

    /// Count of number of threads that are waiting at the barrier; if
    /// this equals 'num_threads' then the threads should all continue
    /// after incrementing the generation_id
    waiting: usize,

    /// Generation id for the barrier; this is read by all threads as
    /// they reach the barrier, if they are not the last to reach it;
    /// then, after waiting on a condition variable, the threads can
    /// compare this value with the post-condition value (current) -
    /// if the current generation is newer, the barrier has been
    /// reached by the *last* thread and progress can continue
    generation_id: usize,
}

impl BarrierState {
    pub const fn new() -> BarrierState {
        BarrierState {
            finish: false,
            finish_next_gen: false,
            num_threads: 1,
            waiting: 0,
            generation_id: 0,
        }
    }
}

//tp BarrierInner
/// The real barrier
///
/// This is held immutably as part of an Arc by many threads; the only
/// mutatable state that can occur when the barrier is held when the
/// Arc has been cloned must be within the Mutex
pub struct BarrierInner {
    /// State shared by all the threads at the barrier
    lock: Mutex<BarrierState>,

    /// Condition variable notified when the BarrierState changes
    cvar: Condvar,
}

//ip BarrierInner
impl BarrierInner {
    //cp new
    /// Create a new barrier
    pub const fn new() -> BarrierInner {
        BarrierInner {
            lock: Mutex::new(BarrierState::new()),
            cvar: Condvar::new(),
        }
    }

    //cp new
    /// Create a new barrier
    // Invoked only by the main thread - but from a thread spawner
    pub fn add_thread(&self) -> usize {
        let mut lock = self.lock.lock().unwrap();
        lock.num_threads += 1;
        lock.num_threads
    }

    //ap num_threads
    fn num_threads(&self) -> usize {
        self.lock.lock().unwrap().num_threads
    }

    //ap pending
    // Usually invoked only by the main thread
    fn pending(&self) -> usize {
        self.lock.lock().unwrap().waiting
    }

    //ap generation
    // Usually invoked only by the main thread
    fn generation(&self) -> usize {
        self.lock.lock().unwrap().generation_id
    }

    //mp next_generation
    /// Move on to the next generation using the lock guard
    ///
    /// Invoked by the last thread which reaches the barrier
    ///
    /// This can be at a wait() or if the last thread is dropping its
    /// use of the barrier prior to reaching wait (e.g. when it
    /// completes)
    fn next_generation(&self, lock: &mut MutexGuard<BarrierState>) {
        lock.waiting = 0;
        lock.generation_id = lock.generation_id.wrapping_add(1);
        if lock.finish_next_gen {
            lock.finish = true;
        }
        self.cvar.notify_all();
    }

    //mp drop_thread
    /// Drop a thread from the barrier
    ///
    /// Invoked by worker threads when dropped
    ///
    /// If this is the last thread then it must perform similarly to
    /// 'wait'; otherwise it *only* needs to reduce the number of
    /// threads in the barrier and notify the threads (in case it was
    /// last-but-one)
    fn drop_thread(&self) {
        let mut lock = self.lock.lock().unwrap();
        if lock.waiting == lock.num_threads {
            self.next_generation(&mut lock);
        }
        lock.num_threads -= 1;
        self.cvar.notify_all();
    }

    //mp wait
    /// Wait on the barrier for all other threads to reach the barrier
    ///
    /// Called by all threads
    ///
    /// It waits until the threads have all arrived: if the number of
    /// threads waiting (including this) is less than the total number
    /// of threads then this thread must wait for others; else it is
    /// the last thread to arrive, and it should perform the moving on
    /// of generation.
    ///
    fn wait(&self) -> BarrierWaitResult {
        let mut lock = self.lock.lock().unwrap();
        let local_gen = lock.generation_id;
        lock.waiting += 1;
        if lock.waiting < lock.num_threads {
            let new_lock = self
                .cvar
                .wait_while(lock, |state| local_gen == state.generation_id)
                .unwrap();
            lock = new_lock;
        } else {
            self.next_generation(&mut lock);
        }
        if lock.finish {
            BarrierWaitResult::Finish
        } else if local_gen == 0 {
            BarrierWaitResult::Started
        } else {
            BarrierWaitResult::Stuff
        }
    }

    // Invoked by the master thread - at this point the number of
    // threads is at a maximum
    fn start(&self) {
        let _ = self.wait();
    }

    fn finish(&self) {
        {
            let mut lock = self.lock.lock().unwrap();
            lock.finish_next_gen = true;
        }
        let r = self.wait();
        assert_eq!(
            r,
            BarrierWaitResult::Finish,
            "Must have reached Finish at finish"
        );
    }
}

//tp Barrier
/// A clonable barrier
pub struct Barrier {
    inner: Arc<BarrierInner>,
}

//ip Drop for Barrier
// Must only be dropped by the main thread once all the worker
// threads have dropped it; the main thread should never be the last
impl std::ops::Drop for Barrier {
    fn drop(&mut self) {
        self.inner.drop_thread();
    }
}

//ip Barrier
impl Barrier {
    //cp new
    /// Construct a new [Barrier]
    pub fn new() -> Barrier {
        let inner = Arc::new(BarrierInner::new());
        Self { inner }
    }

    //mp add_thread
    /// Effectively clone the barrier for a thread
    ///
    /// Invoked only by the main thread - but from a thread spawner
    pub fn add_thread(&self) -> (Self, usize) {
        let inner = self.inner.clone();
        let n = inner.add_thread();
        (Self { inner }, n)
    }

    // Usually invoked only by the main thread
    pub fn pending(&self) -> usize {
        self.inner.pending()
    }

    pub fn num_threads(&self) -> usize {
        self.inner.num_threads()
    }

    pub fn generation(&self) -> usize {
        self.inner.generation()
    }

    // Invoked only by the main thread
    pub fn start(&self) -> () {
        self.inner.start();
    }

    pub fn sync(&self) {
        let _ = self.inner.wait();
    }

    pub fn finish(&self) {
        self.inner.finish()
    }

    // Invoked by the worker threads
    pub fn wait(&self) -> BarrierWaitResult {
        self.inner.wait()
    }
}

#[test]
fn test1() {
    let n = 10;
    let barrier = Barrier::new();
    eprintln!("Entering scope");
    std::thread::scope(|s| {
        for i in 0..n {
            let (b, tn) = barrier.add_thread();
            s.spawn(move || {
                assert_eq!(i+2, tn, "Thread id should match accounting for index of 1 and main thread is first index");
                loop {
                    match b.wait() {
                        BarrierWaitResult::Started => {}
                        BarrierWaitResult::Stuff => {}
                        BarrierWaitResult::Finish => {
                            break;
                        }
                    }
                }
            });
        }
        assert_eq!(
            barrier.num_threads(),
            n + 1,
            "Must have n things spawned plus main thread"
        );
        eprintln!("Start");
        barrier.start();
        eprintln!("Started");
        for _ in 0..100_000 {
            barrier.sync();
        }
        eprintln!("Finish");
        barrier.finish();
        eprintln!("Finished");
    });
    assert!(false);
}
