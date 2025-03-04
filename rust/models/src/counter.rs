//a Imports
use hgl_sim::prelude::component::*;

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[SimStateInfo] = &[
    SimStateInfo::clk("clk", 0),
    SimStateInfo::input("reset_n", 1),
    SimStateInfo::input("load", 2),
    SimStateInfo::input("increment", 3),
    SimStateInfo::input("decrement", 4),
    SimStateInfo::input("data", 5),
    SimStateInfo::output("q", 0),
    SimStateInfo::internal("d", 0),
];

//tp Inputs
#[derive(Debug, Default)]
pub struct Inputs<V>
where
    V: SimBv,
{
    #[allow(dead_code)]
    clk: (),
    pub reset_n: Bit,
    pub load: Bit,
    pub increment: Bit,
    pub decrement: Bit,
    pub data: V,
}

//tp State
#[derive(Debug, Default, Clone, Copy)]
pub struct State<V>
where
    V: SimBv,
{
    pub data: V,
}

//tp Outputs
#[derive(Debug, Default)]
pub struct Outputs<V>
where
    V: SimBv,
{
    pub data: V,
}

//a Counter
//tp Counter
#[derive(Debug, Default)]
pub struct Counter<V>
where
    V: SimBv,
{
    pub reset_value: Option<V>,
    pub inputs: Inputs<V>,
    pub state: State<V>,
    pub next_state: State<V>,
    pub outputs: Outputs<V>,
}

//ip Counter
impl<V> Counter<V>
where
    V: SimBv,
{
    //cp new
    /// Create a new [Counter] with a given reset value (if not the
    /// default)
    pub fn new(reset_value: Option<V>) -> Self {
        Self {
            reset_value,
            ..Default::default()
        }
    }
    fn generate_outputs(&mut self) {
        self.outputs.data = self.state.data;
    }
}

//ip Simulatable for Counter
impl<V> Simulatable for Counter<V>
where
    V: SimBv,
{
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
        self.state.data = self.reset_value.unwrap_or_default();
        self.generate_outputs();
    }

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: SimEdgeMask) {
        if mask.is_posedge(0) {
            self.next_state = self.state;
            if self.inputs.reset_n.is_false() {
                self.next_state.data = self.reset_value.unwrap_or_default();
            } else {
                if self.inputs.load.is_true() {
                    self.next_state.data = self.inputs.data;
                }
                if self.inputs.increment.is_true() {
                    self.next_state.data = self.state.data + V::of_u64(1);
                } else if self.inputs.decrement.is_true() {
                    self.next_state.data = self.state.data - V::of_u64(1);
                }
            }
            self.state = self.next_state;
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
            2 => Some(SimValueRef::of(&self.inputs.load)),
            3 => Some(SimValueRef::of(&self.inputs.increment)),
            4 => Some(SimValueRef::of(&self.inputs.decrement)),
            5 => Some(SimValueRef::of(&self.inputs.data)),
            6 => Some(SimValueRef::of(&self.outputs.data)),
            7 => Some(SimValueRef::of(&self.state.data)),
            _ => None,
        }
    }
    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        match index.as_usize() {
            1 => Some(SimValueRefMut::of(&mut self.inputs.reset_n)),
            2 => Some(SimValueRefMut::of(&mut self.inputs.load)),
            3 => Some(SimValueRefMut::of(&mut self.inputs.increment)),
            4 => Some(SimValueRefMut::of(&mut self.inputs.decrement)),
            5 => Some(SimValueRefMut::of(&mut self.inputs.data)),
            6 => Some(SimValueRefMut::of(&mut self.outputs.data)),
            7 => Some(SimValueRefMut::of(&mut self.state.data)),
            _ => None,
        }
    }
}

//ip Component for Counter
impl<V> Component for Counter<V>
where
    V: SimBv,
{
    type Config = Option<V>;
    type InputsMut<'a> = &'a mut Inputs<V>;
    type Inputs<'a> = &'a Inputs<V>;
    type Outputs<'a> = &'a Outputs<V>;
    fn inputs(&self) -> &Inputs<V> {
        &self.inputs
    }
    fn outputs(&self) -> &Outputs<V> {
        &self.outputs
    }
    fn inputs_mut(&mut self) -> &mut Inputs<V> {
        &mut self.inputs
    }
    fn configure<S: SimRegister>(
        &mut self,
        sim: &mut S,
        handle: S::Handle,
        config: Option<V>,
    ) -> Result<(), String> {
        self.reset_value = config;
        self.state.data = self.reset_value.unwrap_or_default();
        sim.register_input_edge(handle, 0, true, false);
        sim.register_input_edge(handle, 1, false, true);
        self.generate_outputs();
        Ok(())
    }
}

//ip ComponentBuilder for Counter
impl<V> ComponentBuilder for Counter<V>
where
    V: SimBv,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::default()
    }
}
