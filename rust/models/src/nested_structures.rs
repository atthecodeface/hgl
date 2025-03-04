//a Note: created by cyclicity CDL 2.0.0wip1 - do not hand edit without adding a comment line here
//a Imports
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
use hgl_sim::prelude::component::*;

//a  Types for nested_structures
#[derive(Debug, Default, Clone, Copy)]
pub struct t_cline_lg {
    p_a: t_point,
    p_b: t_point,
    c: t_color_lg,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_cline {
    p_a: t_point,
    p_b: t_point,
    c: t_color,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_color_lg {
    r: Bv<8>,
    g: Bv<8>,
    b: Bv<8>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_color {
    r: Bv<3>,
    g: Bv<3>,
    b: Bv<3>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_point {
    x: Bv<4>,
    y: Bv<4>,
}

//t Inputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Inputs {
    vector_input_1: Bv<8>,
    vector_input_0: Bv<8>,
    io_reset: Bit,
}

//t Outputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Outputs {
    vector_output_1: Bv<8>,
    vector_output_0: Bv<8>,
}

//t State_io_clock
#[derive(Debug, Default, Clone, Copy)]
pub struct State_io_clock {
    cline_array: [t_cline; 4],
    cline_2: t_cline,
    cline: t_cline,
}

//t Locals
#[derive(Debug, Default, Clone, Copy)]
pub struct Locals {}

//t ClockEnables
#[derive(Debug, Default, Clone, Copy)]
pub struct ClockEnables {}

//tp Struct nested_structures
#[derive(Debug, Default)]
pub struct nested_structures {
    inputs: Inputs,
    outputs: Outputs,
    next_state_io_clock: State_io_clock,
    state_io_clock: State_io_clock,
    locals: Locals,

    clock_enables: ClockEnables,
}

//ip Struct nested_structures
impl nested_structures {
    fn generate_outputs(&mut self) {}
}

impl nested_structures {
    fn reset_active_high_io_reset(&mut self) {
        self.state_io_clock.cline_array = std::default::Default::default();
        self.state_io_clock.cline_2 = std::default::Default::default();
        self.state_io_clock.cline = std::default::Default::default();
    }

    fn propagate_resets(&mut self) {
        if self.inputs.io_reset.is_true() {
            self.reset_active_high_io_reset();
        }
    }
    fn propagate_to_all_locals(&mut self) {
        // vector_output_1 (3): read_cline
        self.outputs.vector_output_1 = Bv::<8>::default();
        if ((self.inputs.vector_input_0.bit_as_bit(7)) != (Bit::F)) {
            self.outputs
                .vector_output_1
                .bit_range_mut(0, 8)
                .set::<8>((self.state_io_clock.cline.c.r).bit_range(0, 8));
            self.outputs
                .vector_output_1
                .bit_range_mut(3, 8)
                .set::<8>((self.state_io_clock.cline.c.g).bit_range(0, 8));
        } else {
            self.outputs
                .vector_output_1
                .bit_range_mut(0, 8)
                .set::<8>((self.state_io_clock.cline.c.b).bit_range(0, 8));
        }

        // vector_output_0 (3): read_cline
        self.outputs.vector_output_0 = Bv::<8>::default();
        self.outputs
            .vector_output_0
            .bit_range_mut(0, 8)
            .set::<8>((Bv::<2>::default()).bit_range(0, 8));
        self.outputs
            .vector_output_0
            .bit_range_mut(0, 8)
            .set::<8>((Bv::<2>::of_u64(0x1_u64)).bit_range(0, 8));
        self.outputs
            .vector_output_0
            .bit_range_mut(0, 8)
            .set::<8>((Bv::<2>::of_u64(0x2_u64)).bit_range(0, 8));
        self.outputs
            .vector_output_0
            .bit_range_mut(0, 8)
            .set::<8>((Bv::<2>::of_u64(0x3_u64)).bit_range(0, 8));
        if ((self.inputs.vector_input_0.bit_as_bit(6)) != (Bit::F)) {
            self.outputs
                .vector_output_0
                .bit_range_mut(0, 8)
                .set::<8>((self.state_io_clock.cline.p_a.x).bit_range(0, 8));
            self.outputs
                .vector_output_0
                .bit_range_mut(4, 8)
                .set::<8>((self.state_io_clock.cline.p_a.y).bit_range(0, 8));
        } else {
            self.outputs
                .vector_output_0
                .bit_range_mut(0, 8)
                .set::<8>((self.state_io_clock.cline.p_b.x).bit_range(0, 8));
            self.outputs
                .vector_output_0
                .bit_range_mut(4, 8)
                .set::<8>((self.state_io_clock.cline.p_b.y).bit_range(0, 8));
        }
    }
    fn eval_clock_gate_enables(&mut self) {}
    fn eval_next_state_io_clock(&mut self) {
        self.next_state_io_clock = self.state_io_clock;
        // write_cline
        self.next_state_io_clock.cline_2.p_a.x = self.state_io_clock.cline.p_a.x;
        self.next_state_io_clock.cline_2.p_a.y = self.state_io_clock.cline.p_a.y;
        self.next_state_io_clock.cline_2.p_b.x = self.state_io_clock.cline.p_b.x;
        self.next_state_io_clock.cline_2.p_b.y = self.state_io_clock.cline.p_b.y;
        self.next_state_io_clock.cline_2.c.r = self.state_io_clock.cline.c.r;
        self.next_state_io_clock.cline_2.c.g = self.state_io_clock.cline.c.g;
        self.next_state_io_clock.cline_2.c.b = self.state_io_clock.cline.c.b;
        self.next_state_io_clock.cline.p_a.x = Bv::<4>::default();
        self.next_state_io_clock.cline.p_a.y = Bv::<4>::default();
        self.next_state_io_clock.cline.p_b.x = Bv::<4>::default();
        self.next_state_io_clock.cline.p_b.y = Bv::<4>::default();
        self.next_state_io_clock.cline.c.r = Bv::<3>::default();
        self.next_state_io_clock.cline.c.g = Bv::<3>::default();
        self.next_state_io_clock.cline.c.b = Bv::<3>::default();
        self.next_state_io_clock.cline.c.r = {
            let mut bv = Bv::<3>::default();
            bv.bit_set(2, Bit::F);
            bv.bit_set(1, Bit::F);
            bv.bit_set(0, self.outputs.vector_output_1.bit_as_bit(7));
            bv
        };
        self.next_state_io_clock.cline_array[0].p_a.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[0].p_a.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[0].p_b.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[0].p_b.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[0].c.r = Bv::<3>::default();
        self.next_state_io_clock.cline_array[0].c.g = Bv::<3>::default();
        self.next_state_io_clock.cline_array[0].c.b = Bv::<3>::default();
        self.next_state_io_clock.cline_array[2].p_a.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[2].p_a.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[2].p_b.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[2].p_b.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[2].c.r = Bv::<3>::default();
        self.next_state_io_clock.cline_array[2].c.g = Bv::<3>::default();
        self.next_state_io_clock.cline_array[2].c.b = Bv::<3>::default();
        self.next_state_io_clock.cline_array[3].p_a.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[3].p_a.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[3].p_b.x = Bv::<4>::default();
        self.next_state_io_clock.cline_array[3].p_b.y = Bv::<4>::default();
        self.next_state_io_clock.cline_array[3].c.r = Bv::<3>::default();
        self.next_state_io_clock.cline_array[3].c.g = Bv::<3>::default();
        self.next_state_io_clock.cline_array[3].c.b = Bv::<3>::default();
        if ((self.inputs.vector_input_0.bit_as_bit(0)) != (Bit::F)) {
            self.next_state_io_clock.cline.c.r = self.inputs.vector_input_1.bit_range_to_bv(0, 3);
        }
        if ((self.inputs.vector_input_0.bit_as_bit(1)) != (Bit::F)) {
            self.next_state_io_clock.cline.c.g = self.inputs.vector_input_1.bit_range_to_bv(0, 3);
        }
        if ((self.inputs.vector_input_0.bit_as_bit(2)) != (Bit::F)) {
            self.next_state_io_clock.cline.c.b = self.inputs.vector_input_1.bit_range_to_bv(0, 3);
        }
        if ((self.inputs.vector_input_0.bit_as_bit(4)) != (Bit::F)) {
            self.next_state_io_clock.cline.p_a.x = self.inputs.vector_input_1.bit_range_to_bv(0, 4);
            self.next_state_io_clock.cline.p_a.y = self.inputs.vector_input_1.bit_range_to_bv(4, 4);
        }
        if ((self.inputs.vector_input_0.bit_as_bit(5)) != (Bit::F)) {
            self.next_state_io_clock.cline.p_b.x = self.inputs.vector_input_1.bit_range_to_bv(0, 4);
            self.next_state_io_clock.cline.p_b.y = self.inputs.vector_input_1.bit_range_to_bv(4, 4);
        }
    }

    fn propagate_locals_to_submodules(&mut self) {}
    fn propagate_all(&mut self) {
        self.propagate_resets();
        self.propagate_to_all_locals();
        self.propagate_locals_to_submodules();
    }
}
//ip Simulatable for nested_structures
impl Simulatable for nested_structures {
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
        self.reset_active_high_io_reset();
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
        if mask.is_posedge(0) {
            self.eval_next_state_io_clock();
        }
        if mask.is_posedge(0) {
            self.state_io_clock = self.next_state_io_clock;
        }
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
//ip Component for nested_structures
impl Component for nested_structures {
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
//ip ComponentBuilder for nested_structures
impl ComponentBuilder for nested_structures {
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::default()
    }
}
//a Modules built
// se_external_module_register( 1, "nested_structures", nested_structures_instance_fn, "cdl_model" );
