//a Imports
use hgl_sim::prelude::component::*;

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[SimStateInfo] = &[
    SimStateInfo::clk("clk", 0),
    SimStateInfo::input("reset_n", 1),
    SimStateInfo::input("enable", 2),
    SimStateInfo::input("data", 4),
    SimStateInfo::output("q", 0),
];

//tp Inputs
#[derive(Debug, Default)]
pub struct Inputs<V>
where
    V: SimValue,
{
    #[allow(dead_code)]
    clk: (),
    reset_n: Bit,
    enable: Bit,
    data: V,
}

//tp Outputs
#[derive(Debug, Default)]
pub struct Outputs<V>
where
    V: SimValue,
{
    data: V,
}

//a Register
//tp Register
#[derive(Debug, Default)]
pub struct Register<V>
where
    V: SimValue,
{
    reset_value: Option<V>,
    inputs: Inputs<V>,
    outputs: Outputs<V>,
}

//ip Register
impl<V> Register<V>
where
    V: SimValue,
{
    //cp new
    /// Create a new [Register] with a given reset value (if not the
    /// default)
    pub fn new(reset_value: Option<V>) -> Self {
        Self {
            reset_value,
            ..Default::default()
        }
    }
}

//ip Simulatable for Register
impl<V> Simulatable for Register<V>
where
    V: SimValue,
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
        self.outputs.data = self.reset_value.unwrap_or_default();
    }

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: u32) {
        if (mask & 1) != 0 {
            if self.inputs.reset_n.is_false() {
                self.outputs.data = self.reset_value.unwrap_or_default();
            } else if self.inputs.enable.is_true() {
                self.outputs.data = self.inputs.data;
            }
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
            2 => Some(SimValueRef::of(&self.inputs.enable)),
            3 => Some(SimValueRef::of(&self.inputs.data)),
            4 => Some(SimValueRef::of(&self.outputs.data)),
            _ => None,
        }
    }
    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        match index.as_usize() {
            1 => Some(SimValueRefMut::of(&mut self.inputs.reset_n)),
            2 => Some(SimValueRefMut::of(&mut self.inputs.enable)),
            3 => Some(SimValueRefMut::of(&mut self.inputs.data)),
            4 => Some(SimValueRefMut::of(&mut self.outputs.data)),
            _ => None,
        }
    }
}

//ip Component for Register
impl<V> Component for Register<V>
where
    V: SimValue,
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
        sim: &S,
        handle: S::Handle,
        config: Option<V>,
    ) -> Result<(), String> {
        self.reset_value = config;
        sim.register_input_edge(handle, 0, true, false);
        sim.register_input_edge(handle, 1, false, true);
        Ok(())
    }
}

//ip ComponentBuilder for Register
impl<V> ComponentBuilder for Register<V>
where
    V: SimValue,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::default()
    }
}
