//a Imports
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{spawn, JoinHandle};

use cpu_timer::AccTimer;

use hgl_sim::prelude::component::*;

//a State and Action
//tp State
#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum State {
    #[default]
    Idle,
    Running,
    Paused,
    Stopped,
}
impl State {
    fn is_stopped(&self) -> bool {
        matches!(self, State::Stopped)
    }
}

//tp Action
#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Action {
    #[default]
    Idle,
    Resume,
    Pause,
    Stop,
}

impl Action {
    fn is_idle(&self) -> bool {
        matches!(self, Action::Idle)
    }
}

//a MCInner
//tp MCInner
#[derive(Default, Debug)]
struct MCInner {
    input_data: u64,
    result_data: u64,
    timer: AccTimer<false>,
}

//ip ThreadedModel for MCInner
impl ThreadedModel for MCInner {
    fn start(&mut self) {
        self.timer.clear();
    }
    fn pause(&mut self) {
        self.timer.stop()
    }
    fn resume(&mut self) {
        self.timer.start()
    }
    fn stop(&mut self) {}
}

//a ModelControl
//tt ThreadedModel
/// Trait for a threaded model
///
/// All methods are invoked from within a thread separate from the
/// simulation thread
trait ThreadedModel: Send + 'static {
    fn start(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn stop(&mut self) {}
}

//ti ModelControl
#[derive(Default, Debug)]
struct ModelControl {
    state: State,
    action: Action,
    thread_ready: bool,
    thread: Option<JoinHandle<()>>,
}

//tp Model
struct Model<T>
where
    T: ThreadedModel,
{
    inner: Arc<(Mutex<(ModelControl, T)>, Condvar, Condvar)>,
}

impl<T> std::clone::Clone for Model<T>
where
    T: ThreadedModel,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

//ip Model
impl<T> Model<T>
where
    T: ThreadedModel,
{
    //cp new
    fn new(inner: T) -> Self {
        let control = ModelControl::default();
        let inner = Arc::new((Mutex::new((control, inner)), Condvar::new(), Condvar::new()));
        Self { inner }
    }

    //ap get_state
    fn get_state(&self) -> State {
        self.inner.0.lock().unwrap().0.state
    }

    //ap get
    fn get<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        f(&self.inner.0.lock().unwrap().1)
    }

    //mp start
    fn start(&mut self, running: bool) {
        let s = self.clone();
        eprintln!("start inner running:{running}");
        let mut mg = self.inner.0.lock().unwrap();
        assert!(mg.0.state == State::Idle);
        mg.0.action = Action::Idle;
        mg.0.state = State::Paused;
        mg.0.thread_ready = false;
        mg.0.thread = Some(spawn(move || s.thread_run()));
        drop(mg);
        if running {
            self.update_state(Action::Resume);
        };
    }

    //ap wait_for_thread_ready
    fn wait_for_thread_ready(&self) {
        let (m, t, f) = &*self.inner;
        let mut mg = m.lock().unwrap();
        while !mg.0.thread_ready {
            let (new_mg, _f) = f.wait_timeout(mg, std::time::Duration::new(1, 0)).unwrap();
            mg = new_mg;
        }
    }

    //ap update_state
    fn update_state(&self, action: Action) {
        self.wait_for_thread_ready();
        let (m, t, f) = &*self.inner;
        let mut mg = m.lock().unwrap();
        if !mg.0.action.is_idle() {
            panic!("Action should have been cleared if thread is ready!");
        }
        match (mg.0.state, action) {
            (State::Idle, Action::Resume) => (),
            (State::Running, Action::Pause) => (),
            (State::Paused, Action::Resume) => (),
            (State::Idle, Action::Stop) => (),
            (State::Running, Action::Stop) => (),
            (State::Paused, Action::Stop) => (),
            _ => {
                return;
            }
        }
        mg.0.action = action;
        mg.0.thread_ready = false;
        t.notify_all();
    }

    //mp thread_run
    fn thread_run(&self) {
        {
            let (m, _t, f) = &*self.inner;
            let mut mg = m.lock().unwrap();
            mg.1.start();
            mg.0.thread_ready = true;
            f.notify_all();
        }
        eprintln!("thread_run: start");
        loop {
            eprintln!("thread_run: loop start");
            let mg = self.inner.0.lock().unwrap();
            if mg.0.state.is_stopped() {
                break;
            };
            drop(mg);
            self.wait_for_state_change();
        }
        eprintln!("thread_run: finished");
    }

    //ap handle_state_change
    //
    // Cannot be in Idle; this is only invoked from within the thread,
    // so it must be either Running or Paused
    fn handle_state_change(&self, mci: &mut (ModelControl, T)) {
        match (mci.0.state, mci.0.action) {
            (State::Running, Action::Pause) => {
                mci.1.pause();
                mci.0.state = State::Paused;
            }
            (State::Running, Action::Stop) => {
                mci.1.pause();
                mci.1.stop();
                mci.0.state = State::Stopped;
            }
            (State::Paused, Action::Resume) => {
                mci.1.resume();
                mci.0.state = State::Running;
            }
            (State::Paused, Action::Stop) => {
                mci.1.stop();
                mci.0.state = State::Stopped;
            }
            _ => {
                panic!(
                    "Unexpected state/action {:?}, {:?}",
                    mci.0.state, mci.0.action
                );
            }
        }
    }

    //ap wait_for_state_change
    fn wait_for_state_change(&self) -> bool {
        eprintln!("thread: wait_for_state_change");
        let (m, t, f) = &*self.inner;
        let mg = m.lock().unwrap();
        let (mut mg, t) = t.wait_timeout(mg, std::time::Duration::new(1, 0)).unwrap();
        eprintln!("thread: wait_for_state_change: condvar returned {t:?}");
        // secs, ns
        if t.timed_out() {
            false
        } else {
            if !mg.0.action.is_idle() {
                self.handle_state_change(&mut *mg);
                mg.0.action = Action::Idle;
                mg.0.thread_ready = true;
                f.notify_all();
                true
            } else {
                false
            }
        }
    }
    //zz All done
}

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[SimStateInfo] = &[
    SimStateInfo::clk("clk", 0),
    SimStateInfo::input("reset_n", 1),
    SimStateInfo::input("start", 2),
    SimStateInfo::input("stop", 3),
    SimStateInfo::input("data", 4),
    SimStateInfo::output("q", 0),
];

//tp Inputs
#[derive(Debug, Default)]
pub struct Inputs {
    #[allow(dead_code)]
    clk: (),
    pub reset_n: bool,
    pub start: bool,
    pub stop: bool,
    pub data: u64,
}

//tp Outputs
#[derive(Debug, Default)]
pub struct Outputs {
    pub q: u64,
}

//a Threaded
//tp Threaded
pub struct Threaded {
    pub model: Model<MCInner>,
    pub inputs: Inputs,
    pub outputs: Outputs,
}

//ip Threaded
impl Threaded {
    //cp new
    /// Create a new [Threaded] with a given reset value (if not the
    /// default)
    pub fn new() -> Self {
        let inner = MCInner::default();
        let model = Model::new(inner);
        let inputs = Inputs::default();
        let outputs = Outputs::default();
        Self {
            model,
            inputs,
            outputs,
        }
    }
    fn generate_outputs(&mut self) {
        self.outputs.q = self.model.get(|m| m.timer.acc_value());
    }
}

//ip Simulatable for Threaded
impl Simulatable for Threaded {
    //mp as_any
    /// Return a reference as an Any so it can be downcast
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    //mp as_mut_any
    /// Return a mutable reference as an Any so it can be downcast
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    //mp reset
    /// Reset the component
    ///
    /// The reason could be simulation restart, or something 'weaker'
    fn reset(&mut self, _reason: SimReset) {
        self.generate_outputs();
    }

    fn start(&mut self, running: bool) {
        self.model.start(running);
    }

    fn pause(&mut self) {}

    fn resume(&mut self) {}

    fn stop(&mut self) {
        self.model.update_state(Action::Stop);
    }

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: SimEdgeMask) {
        if mask.is_posedge(0) {
            if !self.inputs.reset_n {
                // self.model.
            } else {
                if self.inputs.start {
                    self.model.update_state(Action::Resume);
                } else if self.inputs.stop {
                    self.model.update_state(Action::Pause);
                    self.model.wait_for_thread_ready();
                }
            }
            self.generate_outputs();
        }
    }

    //mp propagate
    /// Propagate inputs through combinational paths and to all submodules
    ///
    /// This is not invoked for clocked-only modules, except when
    /// generating waveforms (or equivalent)
    ///
    /// For modules that declare (at config time) they have
    /// comb_path's, this will be called once for each such
    /// invocation, after any event that might change the inputs. The
    /// 'stage' indicates which set of inputs will now be valid (hence
    /// it is increased on each call, starting at 0 for the first
    /// after a clock edge)
    fn propagate(&mut self, _stage: usize) {}
    fn state_info(&self, index: SimStateIndex) -> Option<SimStateInfo> {
        STATE_INFO.get(index.as_usize()).copied()
    }
    fn try_state_data(&self, index: SimStateIndex) -> Option<SimValueRef> {
        match index.as_usize() {
            1 => Some(SimValueRef::of(&self.inputs.reset_n)),
            2 => Some(SimValueRef::of(&self.inputs.start)),
            3 => Some(SimValueRef::of(&self.inputs.stop)),
            4 => Some(SimValueRef::of(&self.inputs.data)),
            5 => Some(SimValueRef::of(&self.outputs.q)),
            _ => None,
        }
    }
    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        match index.as_usize() {
            1 => Some(SimValueRefMut::of(&mut self.inputs.reset_n)),
            2 => Some(SimValueRefMut::of(&mut self.inputs.start)),
            3 => Some(SimValueRefMut::of(&mut self.inputs.stop)),
            4 => Some(SimValueRefMut::of(&mut self.inputs.data)),
            5 => Some(SimValueRefMut::of(&mut self.outputs.q)),
            _ => None,
        }
    }
}

//ip Component for Threaded
impl Component for Threaded {
    type Config = ();
    type InputsMut<'a> = &'a mut Inputs;
    type Inputs<'a> = &'a Inputs;
    type Outputs<'a> = &'a Outputs;
    fn inputs(&self) -> &Inputs {
        &self.inputs
    }
    fn outputs(&self) -> &Outputs {
        &self.outputs
    }
    fn inputs_mut(&mut self) -> &mut Inputs {
        &mut self.inputs
    }
    fn configure<S: SimRegister>(
        &mut self,
        sim: &S,
        handle: S::Handle,
        config: (),
    ) -> Result<(), String> {
        sim.register_input_edge(handle, 0, true, false);
        sim.register_input_edge(handle, 1, false, true);
        self.generate_outputs();
        Ok(())
    }
}

//ip ComponentBuilder for Threaded
impl ComponentBuilder for Threaded {
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::new()
    }
}
