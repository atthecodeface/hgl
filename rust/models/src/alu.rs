//a Note: created by cyclicity CDL 2.0.0wip1 - do not hand edit without adding a comment line here
//a Imports
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
use hgl_sim::prelude::component::*;

//a  Types for alu
//t Inputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Inputs {
    op: Bv<2>,
    b: Bv<16>,
    a: Bv<16>,
}

//t Outputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Outputs {
    r: Bv<16>,
}

//t Locals
#[derive(Debug, Default, Clone, Copy)]
pub struct Locals {
    x: Bv<16>,
}

//t ClockEnables
#[derive(Debug, Default, Clone, Copy)]
pub struct ClockEnables {}

//tp Struct alu
#[derive(Debug, Default)]
pub struct alu {
    inputs: Inputs,
    outputs: Outputs,
    locals: Locals,

    clock_enables: ClockEnables,
}

//ip Struct alu
impl alu {
    fn generate_outputs(&mut self) {}
}

impl alu {
    fn propagate_resets(&mut self) {}
    fn propagate_to_all_locals(&mut self) {
        // r (3): alu_logic
        self.outputs.r = Bv::<16>::default();
        match (self.inputs.op).try_as_u64().unwrap() {
            0 => {
                self.outputs.r = (self.inputs.a) + (self.inputs.b);
            }
            1 => {
                self.outputs.r = (self.inputs.a) - (self.inputs.b);
            }
            2 => {
                self.outputs.r = (self.inputs.a) | (self.inputs.b);
            }
            3 => {
                self.outputs.r |= (self.inputs.a) >> (self.inputs.b.try_as_u64().unwrap() as usize);
            }
            _ => {
                assert!(false, "./regression/tests/simple/alu.cdl:17:Full switch statement did not cover all values");
            }
        }

        // x (3): alu_logic
        self.locals.x = Bv::<16>::default();
        if (!((self.inputs.op) == (Bv::<2>::default()))) {
            self.locals.x = self.outputs.r;
        }
    }
    fn eval_clock_gate_enables(&mut self) {}
    fn propagate_locals_to_submodules(&mut self) {}
    fn propagate_all(&mut self) {
        self.propagate_resets();
        self.propagate_to_all_locals();
        self.propagate_locals_to_submodules();
    }
}
//ip Simulatable for alu
impl Simulatable for alu {
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
        self.propagate_all();
        self.generate_outputs();
    }

    //mp Clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    fn clock(&mut self, mask: SimEdgeMask) {
        self.propagate_resets();
        self.propagate_to_all_locals();
        self.eval_clock_gate_enables();
        self.generate_outputs();
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
        None
    }

    fn try_state_data(&self, index: SimStateIndex) -> Option<SimValueRef> {
        None
    }

    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        None
    }
}
//ip Component for alu
impl Component for alu {
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
        sim: &mut S,
        handle: S::Handle,
        config: (),
    ) -> Result<(), String> {
        sim.register_input_edge(handle, 0, true, false);
        self.generate_outputs();
        Ok(())
    }
}
//ip ComponentBuilder for alu
impl ComponentBuilder for alu {
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::default()
    }
}
//a Modules built
// se_external_module_register( 1, "alu", alu_instance_fn, "cdl_model" );
