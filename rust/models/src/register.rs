//a Imports
use hgl_sim::prelude::component::*;

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[PortInfo] = &[
    PortInfo::clk("clk", 0),
    PortInfo::input("reset_n", 1),
    PortInfo::input("enable", 2),
    PortInfo::input("data", 4),
    PortInfo::output("q", 0),
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
    fn state_info(&self, index: usize) -> Option<PortInfo> {
        STATE_INFO.get(index).copied()
    }
    fn try_state_data(&self, index: usize) -> Option<SimValueRef> {
        if index == 0 {
            Some(SimValueRef::of(&self.inputs.enable))
        } else {
            None
        }
    }
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
