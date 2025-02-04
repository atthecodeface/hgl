//a Imports
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{spawn, JoinHandle};

use cpu_timer::AccTimer;

use hgl_sim::prelude::component::*;

//a MCInner, ModelControl
//tp State
#[derive(Default, Clone, Copy, PartialEq)]
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
#[derive(Default, Clone, Copy, PartialEq)]
enum Action {
    #[default]
    Idle,
    Start,
    Run,
    Pause,
    Stop,
}

impl Action {
    fn is_idle(&self) -> bool {
        matches!(self, Action::Idle)
    }
}
//tp MCInner
#[derive(Default)]
struct MCInner {
    state: State,
    action: Action,
    input_data: u64,
    result_data: u64,
    timer: AccTimer<true>,
    thread: Option<JoinHandle<()>>,
}

//tp ModelControl
#[derive(Clone)]
struct ModelControl {
    inner: Arc<(Mutex<MCInner>, Condvar)>,
}

//ip ModelControl
impl ModelControl {
    //cp new
    fn new() -> Self {
        let mc_inner = MCInner::default();
        let inner = Arc::new((Mutex::new(mc_inner), Condvar::new()));
        Self { inner }
    }

    //ap get_state
    fn get_state(&self) -> State {
        let i = self.inner.0.lock().unwrap();
        if !i.action.is_idle() {
            panic!("Should wait!");
        }
        i.state
    }

    //ap update_state
    fn update_state(&self, action: Action) {
        let mut i = self.inner.0.lock().unwrap();
        if !i.action.is_idle() {
            panic!("Should wait!");
        }
        if i.state == State::Stopped {
            return;
        }
        i.action = action;
        self.inner.1.notify_all();
    }

    //mp thread_run
    fn thread_run(&self) {
        loop {
            eprintln!("thread_run: loop start");
            let (m, c) = &*self.inner;
            let mg = m.lock().unwrap();
            if !mg.state.is_stopped() {
                break;
            };
            self.wait_for_state_change();
        }
    }

    //ap handle_state_change
    fn handle_state_change(&self, mg: &mut MutexGuard<'_, MCInner>) {}

    //ap wait_for_state_change
    fn wait_for_state_change(&self) -> bool {
        let (m, c) = &*self.inner;
        let mg = m.lock().unwrap();
        let (mut mg, t) = c.wait_timeout(mg, std::time::Duration::new(1, 0)).unwrap();
        // secs, ns
        if t.timed_out() {
            false
        } else {
            if !mg.action.is_idle() {
                self.handle_state_change(&mut mg);
                mg.action = Action::Idle;
                true
            } else {
                false
            }
        }
    }
    //zz All done
}

//ti ModelThread
struct ModelThread {
    control: ModelControl,
    timer: AccTimer<false>,
    thread: Option<JoinHandle<()>>,
}

//ii ModelThread
impl ModelThread {
    fn new() -> Self {
        let control = ModelControl::new();
        Self {
            control,
            timer: AccTimer::default(),
            thread: None,
        }
    }
    fn control(&self) -> ModelControl {
        self.control.clone()
    }
    fn update_state(&mut self) {}
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
    pub model: ModelThread,
    pub control: ModelControl,
    pub inputs: Inputs,
    pub outputs: Outputs,
}

//ip Threaded
impl Threaded {
    //cp new
    /// Create a new [Threaded] with a given reset value (if not the
    /// default)
    pub fn new() -> Self {
        let model = ModelThread::new();
        let control = model.control();
        let inputs = Inputs::default();
        let outputs = Outputs::default();
        Self {
            model,
            control,
            inputs,
            outputs,
        }
    }
    fn generate_outputs(&mut self) {}
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

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: SimEdgeMask) {
        if mask.is_posedge(0) {
            if !self.inputs.reset_n {
            } else {
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
