//a Documentation
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

//a Imports
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use crate::simulation::SimEdgeMask;

//a Types
//a BitSet
//tp BitSet
/// A bit set of bits
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BitSet<const NBYTES: usize> {
    data: [u8; NBYTES],
}

//ip BitSet
impl<const NBYTES: usize> BitSet<NBYTES> {
    //cp new
    /// Create a new BitSet
    pub fn new() -> Self {
        Self { data: [0; NBYTES] }
    }

    //fi as_u8s
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s(&self) -> &[u8] {
        &self.data
    }

    //fi as_u8s_mut
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    //mp set
    /// Set a bit value
    #[track_caller]
    pub fn set<I: Into<bool>>(&mut self, n: usize, v: I) {
        let v = if v.into() { 0xff } else { 0 };
        let b = 1 << (n % 8);
        self.data[n / 8] = (self.data[n / 8] & !b) | (b & v);
    }

    //mp is_set
    /// Return true if a bit is set
    #[track_caller]
    pub fn is_set(&mut self, n: usize) -> bool {
        ((self.data[n / 8] >> (n % 8)) & 1) != 0
    }
}

//tp SimWaitResult
/// Return value from a poll function
pub enum SimWaitResult {
    Ready,
    Other,
}

//tt SimBlah
/// Trait provided by a polling
pub trait SimBlah: Send + 'static {
    fn poll(&mut self, edges: &SimEdgeMask) -> SimWaitResult {
        SimWaitResult::Ready
    }
}

//ip SimBlah for () - enable a null poll
impl SimBlah for () {}

//tp BarrierWaitResult
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarrierWaitResult {
    Started,
    Stuff,
    Finish,
}

//a WorkerBarrier
//ti WorkerBarrierState
struct WorkerBarrierState {
    /// If asserted, the worker must receive a 'BarrierWaitResult::Finish'
    /// after return from its next wait (which it is notified for)
    ///
    /// It must then drop the Worker
    finish: bool,

    /// If asserted, the worker will receive a 'BarrierWaitResult::Stuff'
    /// after return from its next wait (which it is notified for)
    run: bool,

    /// If asserted, the worker has completed and then invoked wait
    ///
    /// It must never be waited for again
    completed: bool,

    /// If asserted, the worker has been added
    ///
    /// It must never be waited for again
    added: bool,

    /// Function to invoke at the next sim clock firing, given a set
    /// of SimEdges
    poll: Box<dyn SimBlah>,
}

//ip WorkerBarrierState
impl WorkerBarrierState {
    //cp new
    /// Create a new [WorkerBarrierState]
    pub fn new() -> WorkerBarrierState {
        WorkerBarrierState {
            finish: false,
            run: false,
            added: false,
            completed: false,
            poll: Box::new(()),
        }
    }

    /// Invoked by the main thread
    fn is_ready(&mut self, edges: &SimEdgeMask) -> bool {
        match self.poll.poll(edges) {
            SimWaitResult::Ready => true,
            _ => false,
        }
    }
}

//tp WorkerBarrierInner
pub struct WorkerBarrierInner {
    /// State shared by all the threads at the barrier
    lock: Mutex<WorkerBarrierState>,

    /// Condition variable notified when the BarrierState changes
    cvar: Condvar,
    worker: usize,
}

//ip WorkerBarrierInner
impl WorkerBarrierInner {
    //cp new
    fn new(worker: usize) -> Self {
        Self {
            lock: Mutex::new(WorkerBarrierState::new()),
            cvar: Condvar::new(),
            worker,
        }
    }

    //mp poll
    fn poll(&self, edges: &SimEdgeMask) -> (bool, bool) {
        let mut lock = self.lock.lock().unwrap();
        if lock.completed || !lock.added {
            (false, true)
        } else {
            (lock.is_ready(edges), false)
        }
    }

    //mp wait
    /// Invoked by the worker thread
    fn wait<T: SimBlah>(&self, tock_barrier: &TockBarrier, t: T) -> BarrierWaitResult {
        let mut lock = self.lock.lock().unwrap();
        lock.poll = Box::new(t);
        lock.run = false;
        tock_barrier.worker_waiting(self.worker);
        if !lock.run && !lock.finish {
            let new_lock = self
                .cvar
                .wait_while(lock, |wbs| !wbs.run && !wbs.finish)
                .unwrap();
            lock = new_lock;
        }
        if lock.finish {
            BarrierWaitResult::Finish
        } else {
            BarrierWaitResult::Stuff
        }
    }

    //mp complete
    /// Invoked by a worker thread
    fn complete(&self, tock_barrier: &TockBarrier) {
        let mut lock = self.lock.lock().unwrap();
        // assert!(lock.run, "Can only complete a thread if it is running");
        lock.completed = true;
        tock_barrier.worker_completed(self.worker, lock.run);
    }

    //mp run
    /// Invoked by the main thread
    fn run(&self) {
        let mut lock = self.lock.lock().unwrap();
        lock.run = true;
        self.cvar.notify_all();
    }

    //mp finish
    /// Invoked by the main thread
    fn finish(&self) {
        let mut lock = self.lock.lock().unwrap();
        lock.finish = true;
        self.cvar.notify_all();
    }
}

//a TockBarrier
//ti TockBarrierState
struct TockBarrierState {
    /// Total number of workers in the barrier
    ///
    /// This must only increase when the barrier is created, before
    /// the use of the barrier in earnest. This equals the length of
    /// the poll instances in the outer [BarrierInner]
    num_running: usize,

    /// Count of number of worker threads that have are waiting after having completed their 'tick' execution
    waiting: usize,

    /// If asserted, workers must finish
    finish: bool,

    /// Number of workers added
    added_workers: usize,

    /// Maximum number of workers
    max_workers: usize,
}

//ip TockBarrierState
impl TockBarrierState {
    pub const fn new(max_workers: usize) -> TockBarrierState {
        TockBarrierState {
            num_running: 0,
            waiting: 0,
            finish: false,
            added_workers: 0,
            max_workers,
        }
    }
}

//tp TockBarrier
/// The real barrier that the main thread waits on
///
/// This is the 'tock', with the 'tick' being the kick for the workers
/// and the 'tock' for the continuation of the main thread once the
/// workers have are waiting after their 'tick' execution completed
pub struct TockBarrier {
    /// State shared by all the threads at the barrier
    lock: Mutex<TockBarrierState>,

    /// Condition variable notified when the [TockBarrierState] changes
    cvar: Condvar,
}

//ip TockBarrier
impl TockBarrier {
    //cp new
    /// Create a new barrier
    pub const fn new(max_workers: usize) -> TockBarrier {
        TockBarrier {
            lock: Mutex::new(TockBarrierState::new(max_workers)),
            cvar: Condvar::new(),
        }
    }

    //mp set_workers_running
    pub fn set_workers_running(&self, num_running: usize) {
        let mut lock = self.lock.lock().unwrap();
        lock.num_running = num_running;
        lock.waiting = 0;
    }

    //mp set_finish
    pub fn set_finish(&self) {
        let mut lock = self.lock.lock().unwrap();
        lock.finish = true;
    }

    //ap num_running
    fn num_running(&self) -> usize {
        self.lock.lock().unwrap().num_running
    }

    //ap finish
    fn finish(&self) -> bool {
        self.lock.lock().unwrap().finish
    }

    //mp wait_for_all_workers
    pub fn wait_for_all_workers(&self) {
        let mut lock = self.lock.lock().unwrap();
        if lock.num_running > lock.waiting {
            let new_lock = self
                .cvar
                .wait_while(lock, |tbs| tbs.num_running > tbs.waiting)
                .unwrap();
            lock = new_lock;
        }
    }

    //mp worker_waiting
    pub fn worker_waiting(&self, _worker: usize) {
        let mut lock = self.lock.lock().unwrap();
        lock.waiting += 1;
        self.cvar.notify_all();
    }

    //mp worker_completed
    /// Invoked by a worker when it completes
    ///
    /// The worker is locked at this point; if it was running then it
    /// was being waited for by the main thread, so the main thread should be told
    pub fn worker_completed(&self, _worker: usize, was_running: bool) {
        let mut lock = self.lock.lock().unwrap();
        if was_running {
            lock.waiting += 1;
        }
        self.cvar.notify_all();
    }

    //zz All done
}

//a BarrierInner, Barrier
//tp BarrierInner
/// The real barrier
///
/// This is held immutably as part of an Arc by many threads; the only
/// mutatable state that can occur when the barrier is held when the
/// Arc has been cloned must be within the Mutex
pub struct BarrierInner {
    /// Barrier that the main thread waits on while workers are running
    tock_barrier: TockBarrier,

    /// One thing per worker
    ///
    /// Put this in a RwLock?
    workers: Vec<WorkerBarrierInner>,
}

//ip BarrierInner
impl BarrierInner {
    //cp new
    /// Create a new barrier
    pub fn new(max_workers: usize) -> BarrierInner {
        let mut workers = vec![];
        for i in 0..max_workers {
            workers.push(WorkerBarrierInner::new(i));
        }
        BarrierInner {
            tock_barrier: TockBarrier::new(max_workers),
            workers,
        }
    }

    //mp add_worker
    /// Add another worker thread to the barrier
    ///
    /// Invoked by the main thread
    ///
    /// The worker should then issue a wait_poll or wait; at some point
    ///
    /// Hence the worker thread state must be
    #[track_caller]
    pub fn add_worker(&self) -> usize {
        let max = self.tock_barrier.lock.lock().unwrap().max_workers;
        let n = self.tock_barrier.lock.lock().unwrap().added_workers;
        if n >= max {
            panic!("Too many worker threads added");
        }
        self.workers[n].lock.lock().unwrap().run = true;
        self.workers[n].lock.lock().unwrap().added = true;
        self.tock_barrier.lock.lock().unwrap().num_running += 1;
        self.tock_barrier.lock.lock().unwrap().added_workers += 1;
        n
    }

    //ap added_workers
    fn added_workers(&self) -> usize {
        self.tock_barrier.lock.lock().unwrap().added_workers
    }

    //ap running_workers
    fn running_workers(&self) -> usize {
        self.tock_barrier.num_running()
    }

    //mp run_workers
    pub fn run_workers<const NBYTES: usize>(&self, edges: &SimEdgeMask) -> usize {
        let mut bits = BitSet::<{ NBYTES }>::new();
        let mut running = 0;
        let finish = self.tock_barrier.finish();
        for (i, w) in self.workers.iter().enumerate() {
            let (ready, completed) = w.poll(edges);
            if !completed && (finish || ready) {
                bits.set(i, true);
                running += 1;
            }
        }
        self.tock_barrier.set_workers_running(running);
        for (i, w) in self.workers.iter().enumerate() {
            if finish {
                w.finish();
            } else if bits.is_set(i) {
                w.run();
            }
        }
        running
    }

    //mp wait_for_all_workers
    pub fn wait_for_all_workers(&self) {
        self.tock_barrier.wait_for_all_workers();
    }

    //mp wait_poll
    /// Wait until a simulation edge hits where a poll condition is met
    ///
    /// Called by all worker threads
    ///
    /// It updates the worker's barrier state; it prods the general
    /// barrier, and then sleeps the worker on its worker barrier.
    ///
    /// The general barrier is waiting for all workers that
    /// threads waiting (including this) is less than the total number
    /// of threads then this thread must wait for others; else it is
    /// the last thread to arrive, and it should perform the moving on
    /// of generation.
    ///
    fn worker_wait_poll<T: SimBlah>(&self, worker: usize, t: T) -> BarrierWaitResult {
        self.workers[worker].wait(&self.tock_barrier, t)
    }

    //mp wait
    /// Wait on the barrier for all other workers to reach the barrier
    ///
    /// Called by all threads
    ///
    /// It waits until the threads have all arrived: if the number of
    /// threads waiting (including this) is less than the total number
    /// of threads then this thread must wait for others; else it is
    /// the last thread to arrive, and it should perform the moving on
    /// of generation.
    ///
    fn worker_wait(&self, worker: usize) -> BarrierWaitResult {
        self.worker_wait_poll(worker, ())
    }

    //mp worker_completed
    /// Invoked by a worker
    ///
    ///
    ///
    /// Worker has completed
    fn worker_completed(&self, worker: usize) {
        self.workers[worker].complete(&self.tock_barrier);
    }

    //mp finish
    fn finish(&self) {
        let n = self.workers.len();
        self.tock_barrier.set_finish();
        self.tock_barrier.set_workers_running(n);
        for w in self.workers.iter() {
            w.finish();
        }
    }
}

//tp Barrier
/// A clonable barrier
pub struct Barrier {
    inner: Arc<BarrierInner>,
}

//ip Barrier
impl Barrier {
    //cp new
    /// Construct a new [Barrier]
    pub fn new(max_workers: usize) -> Barrier {
        let inner = Arc::new(BarrierInner::new(max_workers));
        Self { inner }
    }

    //mp add_worker
    /// Effectively clone the barrier for a worker thread
    ///
    /// Invoked only from the main thread - but from a worker spawner
    ///
    /// The worker
    #[track_caller]
    pub fn add_worker(&self) -> Worker {
        let barrier = self.inner.clone();
        let worker = barrier.add_worker();
        Worker {
            barrier: Barrier { inner: barrier },
            worker,
        }
    }

    //ap added_rworkers
    fn added_workers(&self) -> usize {
        self.inner.added_workers()
    }

    //mp start
    // Invoked only by the main thread
    pub fn start(&self) -> usize {
        self.inner.added_workers()
    }

    //mp sync
    pub fn sync(&self) {
        let _ = self.inner.wait_for_all_workers();
    }

    //mp run_workers
    pub fn run_workers<const NBYTES: usize>(&self, edges: &SimEdgeMask) -> usize {
        self.inner.run_workers::<NBYTES>(edges)
    }

    pub fn finish(&self) {
        self.inner.finish()
    }
}

//a Worker
//tp Worker
/// A worker that is part of a clonable barrier
pub struct Worker {
    barrier: Barrier,
    worker: usize,
}

//ip Worker
impl Worker {
    fn worker(&self) -> usize {
        self.worker
    }

    fn wait(&self) -> BarrierWaitResult {
        self.barrier.inner.worker_wait(self.worker)
    }
    fn wait_poll<T: SimBlah>(&self, t: T) -> BarrierWaitResult {
        self.barrier.inner.worker_wait_poll(self.worker, t)
    }
}

//ip Drop for Worker
// Workers can drop if they are supposed to be waiting or after they completed
impl std::ops::Drop for Worker {
    fn drop(&mut self) {
        self.barrier.inner.worker_completed(self.worker)
    }
}

//a Tests
//tf test1
#[test]
fn test1() {
    let n = 10;
    let barrier = Barrier::new(32);
    eprintln!("Entering scope");
    std::thread::scope(|s| {
        for i in 0..n {
            let wt = barrier.add_worker();
            s.spawn(move || {
                //                 assert_eq!(i+2, tn, "Thread id should match accounting for index of 1 and main thread is first index");
                loop {
                    // eprintln!("Loop {}", wt.worker());
                    match wt.wait() {
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
            barrier.added_workers(),
            n,
            "Must have n things spawned plus main thread"
        );
        eprintln!("Start");
        barrier.start();
        eprintln!("Started");
        let edges = SimEdgeMask::default();
        for _ in 0..100_000 {
            // eprintln!("Sync");
            barrier.sync();
            // eprintln!("Synced, runing workers");
            barrier.run_workers::<8>(&edges);
        }
        eprintln!("Finish");
        barrier.finish();
        eprintln!("Finished");
    });
}
