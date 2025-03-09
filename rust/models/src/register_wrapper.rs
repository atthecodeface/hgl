//a Imports
use crate::Register;
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
    V: SimCopyValue,
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
    V: SimCopyValue,
{
    data: V,
}

//a RegisterWrapper
//tp RegisterWrapper
#[derive(Debug, Default)]
pub struct RegisterWrapper<V>
where
    V: SimCopyValue,
{
    inputs: Inputs<V>,
    outputs: Outputs<V>,
    register: Register<V>,
}

//ip RegisterWrapper
impl<V> RegisterWrapper<V>
where
    V: SimCopyValue,
{
    //cp new
    /// Create a new [RegisterWrapper] with a given reset value (if not the
    /// default)
    ///
    /// Should return a result
    pub fn new<S: SimRegister>(sim: &mut S, name: SimNsName) -> Self {
        let register = sim
            .instantiate::<Register<V>>("reg", || reset_value)
            .unwrap();
        Self {
            register,
            ..Default::default()
        }
    }
}

//ip Simulatable for RegisterWrapper
impl<V> Simulatable for RegisterWrapper<V>
where
    V: SimCopyValue,
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
    fn reset(&mut self, reason: SimReset) {
        self.register.reset(reason);
        self.outputs.data = self.register.outputs.data;
    }

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: SimEdgeMask) {
        if mask.is_posedge(0) {
            self.register.inputs.reset_n = self.inputs.reset_n;
            self.register.inputs.enable = self.inputs.enable;
            self.register.inputs.data = self.inputs.data;
            self.register.clock(SimEdgeMask::none().add_posedge(0));
            self.outputs.data = self.register.outputs.data;
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
    fn propagate(&mut self, _stage: usize) {
        self.register.inputs.reset_n = self.inputs.reset_n;
        self.register.inputs.enable = self.inputs.enable;
        self.register.inputs.data = self.inputs.data;
        self.register.propagate(0);
        self.outputs.data = self.register.inputs.data;
    }
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

//ip Component for RegisterWrapper
impl<V> Component for RegisterWrapper<V>
where
    V: SimCopyValue,
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
        _config: Option<V>,
    ) -> Result<(), String> {
        sim.register_input_edge(handle, 0, true, false);
        sim.register_input_edge(handle, 1, false, true);
        Ok(())
    }
}

//ip ComponentBuilder for RegisterWrapper
impl<V> ComponentBuilder for RegisterWrapper<V>
where
    V: SimCopyValue,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(sim: &mut S, name: SimNsName) -> Self {
        Self::new(sim, name)
    }
}
