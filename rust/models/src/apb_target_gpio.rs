//a Note: created by cyclicity CDL 2.0.0wip1 - do not hand edit without adding a comment line here
//a Imports
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
use hgl_sim::prelude::component::*;

use crate::t_apb_processor_request;
use crate::t_apb_processor_response;
use crate::t_apb_request;
use crate::t_apb_response;
use crate::t_apb_rom_request;

#[derive(Debug, Default, Clone, Copy)]
pub struct t_gpio_input {
    input_type: Bv<3>,
    sync_value: Bit,
    last_sync_value: Bit,
    value: Bit,
    event: Bit,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_gpio_output {
    value: Bit,
    enable: Bit,
}
pub type t_gpio_input_type = Bv<3>;
pub type t_apb_address = Bv<2>;

//a  Types for apb_target_gpio
//t Inputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Inputs {
    pub gpio_input: Bv<16>,
    pub apb_request: t_apb_request,
    pub reset_n: Bit,
}

//t Outputs
#[derive(Debug, Default, Clone, Copy)]
pub struct Outputs {
    pub gpio_input_event: Bit,
    pub gpio_output_enable: Bv<16>,
    pub gpio_output: Bv<16>,
    pub apb_response: t_apb_response,
}

//t State_clk
#[derive(Debug, Default, Clone, Copy)]
pub struct State_clk {
    pub outputs: [t_gpio_output; 16],
    pub inputs: [t_gpio_input; 16],
    pub access: Bv<3>,
}

//t Locals
#[derive(Debug, Default, Clone, Copy)]
pub struct Locals {}

//t ClockEnables
#[derive(Debug, Default, Clone, Copy)]
pub struct ClockEnables {}

//tp Struct apb_target_gpio
#[derive(Debug, Default)]
pub struct apb_target_gpio {
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub next_state_clk: State_clk,
    pub state_clk: State_clk,
    pub locals: Locals,

    pub clock_enables: ClockEnables,
}

//ip Struct apb_target_gpio
impl apb_target_gpio {
    fn generate_outputs(&mut self) {}
}

impl apb_target_gpio {
    fn reset_active_low_reset_n(&mut self) {
        self.state_clk.outputs = std::default::Default::default();
        self.state_clk.inputs = std::default::Default::default();
        self.state_clk.access = std::default::Default::default();
    }

    fn propagate_resets(&mut self) {
        if !self.inputs.reset_n.is_true() {
            self.reset_active_low_reset_n();
        }
    }
    fn propagate_to_all_locals(&mut self) {
        // gpio_input_event (3): input_logic
        self.outputs.gpio_input_event = Bit::F;
        if (Bit::from((self.state_clk.inputs[0].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[1].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[2].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[3].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[4].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[5].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[6].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[7].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[8].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[9].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[10].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[11].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[12].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[13].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[14].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }
        if (Bit::from((self.state_clk.inputs[15].event) != (Bit::F))).into() {
            self.outputs.gpio_input_event = Bit::T;
        }

        // gpio_output_enable (3): output_logic
        self.outputs
            .gpio_output_enable
            .bit_set(0, self.state_clk.outputs[0].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(1, self.state_clk.outputs[1].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(2, self.state_clk.outputs[2].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(3, self.state_clk.outputs[3].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(4, self.state_clk.outputs[4].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(5, self.state_clk.outputs[5].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(6, self.state_clk.outputs[6].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(7, self.state_clk.outputs[7].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(8, self.state_clk.outputs[8].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(9, self.state_clk.outputs[9].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(10, self.state_clk.outputs[10].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(11, self.state_clk.outputs[11].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(12, self.state_clk.outputs[12].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(13, self.state_clk.outputs[13].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(14, self.state_clk.outputs[14].enable);
        self.outputs
            .gpio_output_enable
            .bit_set(15, self.state_clk.outputs[15].enable);

        // gpio_output (3): output_logic
        self.outputs
            .gpio_output
            .bit_set(0, self.state_clk.outputs[0].value);
        self.outputs
            .gpio_output
            .bit_set(1, self.state_clk.outputs[1].value);
        self.outputs
            .gpio_output
            .bit_set(2, self.state_clk.outputs[2].value);
        self.outputs
            .gpio_output
            .bit_set(3, self.state_clk.outputs[3].value);
        self.outputs
            .gpio_output
            .bit_set(4, self.state_clk.outputs[4].value);
        self.outputs
            .gpio_output
            .bit_set(5, self.state_clk.outputs[5].value);
        self.outputs
            .gpio_output
            .bit_set(6, self.state_clk.outputs[6].value);
        self.outputs
            .gpio_output
            .bit_set(7, self.state_clk.outputs[7].value);
        self.outputs
            .gpio_output
            .bit_set(8, self.state_clk.outputs[8].value);
        self.outputs
            .gpio_output
            .bit_set(9, self.state_clk.outputs[9].value);
        self.outputs
            .gpio_output
            .bit_set(10, self.state_clk.outputs[10].value);
        self.outputs
            .gpio_output
            .bit_set(11, self.state_clk.outputs[11].value);
        self.outputs
            .gpio_output
            .bit_set(12, self.state_clk.outputs[12].value);
        self.outputs
            .gpio_output
            .bit_set(13, self.state_clk.outputs[13].value);
        self.outputs
            .gpio_output
            .bit_set(14, self.state_clk.outputs[14].value);
        self.outputs
            .gpio_output
            .bit_set(15, self.state_clk.outputs[15].value);

        // apb_response.prdata (3): apb_interface_logic
        self.outputs.apb_response.prdata = Bv::<32>::default();
        match (self.state_clk.access).try_as_u64().unwrap() {
            6 => {
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(16, self.state_clk.inputs[0].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(0, self.state_clk.inputs[0].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(17, self.state_clk.inputs[1].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(1, self.state_clk.inputs[1].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(18, self.state_clk.inputs[2].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(2, self.state_clk.inputs[2].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(19, self.state_clk.inputs[3].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(3, self.state_clk.inputs[3].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(20, self.state_clk.inputs[4].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(4, self.state_clk.inputs[4].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(21, self.state_clk.inputs[5].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(5, self.state_clk.inputs[5].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(22, self.state_clk.inputs[6].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(6, self.state_clk.inputs[6].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(23, self.state_clk.inputs[7].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(7, self.state_clk.inputs[7].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(24, self.state_clk.inputs[8].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(8, self.state_clk.inputs[8].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(25, self.state_clk.inputs[9].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(9, self.state_clk.inputs[9].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(26, self.state_clk.inputs[10].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(10, self.state_clk.inputs[10].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(27, self.state_clk.inputs[11].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(11, self.state_clk.inputs[11].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(28, self.state_clk.inputs[12].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(12, self.state_clk.inputs[12].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(29, self.state_clk.inputs[13].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(13, self.state_clk.inputs[13].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(30, self.state_clk.inputs[14].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(14, self.state_clk.inputs[14].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(31, self.state_clk.inputs[15].event);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(15, self.state_clk.inputs[15].value);
            }
            4 => {
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(0, 32)
                    .set::<32>((self.state_clk.inputs[0].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(4, 32)
                    .set::<32>((self.state_clk.inputs[1].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(8, 32)
                    .set::<32>((self.state_clk.inputs[2].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(12, 32)
                    .set::<32>((self.state_clk.inputs[3].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(16, 32)
                    .set::<32>((self.state_clk.inputs[4].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(20, 32)
                    .set::<32>((self.state_clk.inputs[5].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(24, 32)
                    .set::<32>((self.state_clk.inputs[6].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(28, 32)
                    .set::<32>((self.state_clk.inputs[7].input_type).bit_range(0, 32));
            }
            5 => {
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(0, 32)
                    .set::<32>((self.state_clk.inputs[8].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(4, 32)
                    .set::<32>((self.state_clk.inputs[9].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(8, 32)
                    .set::<32>((self.state_clk.inputs[10].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(12, 32)
                    .set::<32>((self.state_clk.inputs[11].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(16, 32)
                    .set::<32>((self.state_clk.inputs[12].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(20, 32)
                    .set::<32>((self.state_clk.inputs[13].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(24, 32)
                    .set::<32>((self.state_clk.inputs[14].input_type).bit_range(0, 32));
                self.outputs
                    .apb_response
                    .prdata
                    .bit_range_mut(28, 32)
                    .set::<32>((self.state_clk.inputs[15].input_type).bit_range(0, 32));
            }
            3 => {
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(1, self.state_clk.outputs[0].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(0, self.state_clk.outputs[0].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(3, self.state_clk.outputs[1].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(2, self.state_clk.outputs[1].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(5, self.state_clk.outputs[2].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(4, self.state_clk.outputs[2].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(7, self.state_clk.outputs[3].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(6, self.state_clk.outputs[3].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(9, self.state_clk.outputs[4].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(8, self.state_clk.outputs[4].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(11, self.state_clk.outputs[5].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(10, self.state_clk.outputs[5].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(13, self.state_clk.outputs[6].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(12, self.state_clk.outputs[6].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(15, self.state_clk.outputs[7].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(14, self.state_clk.outputs[7].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(17, self.state_clk.outputs[8].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(16, self.state_clk.outputs[8].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(19, self.state_clk.outputs[9].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(18, self.state_clk.outputs[9].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(21, self.state_clk.outputs[10].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(20, self.state_clk.outputs[10].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(23, self.state_clk.outputs[11].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(22, self.state_clk.outputs[11].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(25, self.state_clk.outputs[12].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(24, self.state_clk.outputs[12].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(27, self.state_clk.outputs[13].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(26, self.state_clk.outputs[13].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(29, self.state_clk.outputs[14].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(28, self.state_clk.outputs[14].value);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(31, self.state_clk.outputs[15].enable);
                self.outputs
                    .apb_response
                    .prdata
                    .bit_set(30, self.state_clk.outputs[15].value);
            }
            _ => {}
        }

        // apb_response.pready (3): apb_interface_logic
        self.outputs.apb_response.pready = Bit::F;
        self.outputs.apb_response.pready = Bit::T;

        // apb_response.perr (3): apb_interface_logic
        self.outputs.apb_response.perr = Bit::F;
    }
    fn eval_clock_gate_enables(&mut self) {}
    fn eval_next_state_clk(&mut self) {
        self.next_state_clk = self.state_clk;
        // apb_interface_logic
        self.next_state_clk.access = Bv::<3>::default();
        match (self.inputs.apb_request.paddr.bit_range_to_bv::<2>(0, 2))
            .try_as_u64()
            .unwrap()
        {
            0 => {
                self.next_state_clk.access = {
                    if Bit::from((self.inputs.apb_request.pwrite) != (Bit::F)).into() {
                        Bv::<3>::of_u64(0x1_u64)
                    } else {
                        Bv::<3>::of_u64(0x3_u64)
                    }
                };
            }
            2 => {
                self.next_state_clk.access = {
                    if Bit::from((self.inputs.apb_request.pwrite) != (Bit::F)).into() {
                        Bv::<3>::of_u64(0x2_u64)
                    } else {
                        Bv::<3>::of_u64(0x4_u64)
                    }
                };
            }
            3 => {
                self.next_state_clk.access = {
                    if Bit::from((self.inputs.apb_request.pwrite) != (Bit::F)).into() {
                        Bv::<3>::of_u64(0x2_u64)
                    } else {
                        Bv::<3>::of_u64(0x5_u64)
                    }
                };
            }
            1 => {
                self.next_state_clk.access = Bv::<3>::of_u64(0x6_u64);
            }
            _ => {}
        }
        if ((!(Bit::from((self.inputs.apb_request.psel) != (Bit::F))))
            | (Bit::from((self.inputs.apb_request.penable) != (Bit::F))))
        .into()
        {
            self.next_state_clk.access = Bv::<3>::default();
        }

        // output_logic
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[0].enable = self.inputs.apb_request.pwdata.bit_as_bit(1);
            self.next_state_clk.outputs[0].value = self.inputs.apb_request.pwdata.bit_as_bit(0);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[1].enable = self.inputs.apb_request.pwdata.bit_as_bit(3);
            self.next_state_clk.outputs[1].value = self.inputs.apb_request.pwdata.bit_as_bit(2);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[2].enable = self.inputs.apb_request.pwdata.bit_as_bit(5);
            self.next_state_clk.outputs[2].value = self.inputs.apb_request.pwdata.bit_as_bit(4);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[3].enable = self.inputs.apb_request.pwdata.bit_as_bit(7);
            self.next_state_clk.outputs[3].value = self.inputs.apb_request.pwdata.bit_as_bit(6);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[4].enable = self.inputs.apb_request.pwdata.bit_as_bit(9);
            self.next_state_clk.outputs[4].value = self.inputs.apb_request.pwdata.bit_as_bit(8);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[5].enable = self.inputs.apb_request.pwdata.bit_as_bit(11);
            self.next_state_clk.outputs[5].value = self.inputs.apb_request.pwdata.bit_as_bit(10);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[6].enable = self.inputs.apb_request.pwdata.bit_as_bit(13);
            self.next_state_clk.outputs[6].value = self.inputs.apb_request.pwdata.bit_as_bit(12);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[7].enable = self.inputs.apb_request.pwdata.bit_as_bit(15);
            self.next_state_clk.outputs[7].value = self.inputs.apb_request.pwdata.bit_as_bit(14);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[8].enable = self.inputs.apb_request.pwdata.bit_as_bit(17);
            self.next_state_clk.outputs[8].value = self.inputs.apb_request.pwdata.bit_as_bit(16);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[9].enable = self.inputs.apb_request.pwdata.bit_as_bit(19);
            self.next_state_clk.outputs[9].value = self.inputs.apb_request.pwdata.bit_as_bit(18);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[10].enable = self.inputs.apb_request.pwdata.bit_as_bit(21);
            self.next_state_clk.outputs[10].value = self.inputs.apb_request.pwdata.bit_as_bit(20);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[11].enable = self.inputs.apb_request.pwdata.bit_as_bit(23);
            self.next_state_clk.outputs[11].value = self.inputs.apb_request.pwdata.bit_as_bit(22);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[12].enable = self.inputs.apb_request.pwdata.bit_as_bit(25);
            self.next_state_clk.outputs[12].value = self.inputs.apb_request.pwdata.bit_as_bit(24);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[13].enable = self.inputs.apb_request.pwdata.bit_as_bit(27);
            self.next_state_clk.outputs[13].value = self.inputs.apb_request.pwdata.bit_as_bit(26);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[14].enable = self.inputs.apb_request.pwdata.bit_as_bit(29);
            self.next_state_clk.outputs[14].value = self.inputs.apb_request.pwdata.bit_as_bit(28);
        }
        if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x1_u64)))).into() {
            self.next_state_clk.outputs[15].enable = self.inputs.apb_request.pwdata.bit_as_bit(31);
            self.next_state_clk.outputs[15].value = self.inputs.apb_request.pwdata.bit_as_bit(30);
        }

        // input_logic
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4)) == (Bv::<4>::default()),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[0].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[0].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x1_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[1].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[1].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x2_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[2].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[2].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x3_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[3].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[3].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x4_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[4].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[4].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x5_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[5].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[5].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x6_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[6].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[6].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x7_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[7].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[7].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x8_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[8].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[8].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0x9_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[9].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[9].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xa_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[10].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[10].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xb_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[11].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[11].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xc_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[12].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[12].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xd_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[13].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[13].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xe_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[14].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[14].event = Bit::F;
                }
            }
        }
        if (Bit::from(
            (self.inputs.apb_request.pwdata.bit_range_to_bv::<4>(0, 4))
                == (Bv::<4>::of_u64(0xf_u64)),
        ))
        .into()
        {
            if (Bit::from((self.state_clk.access) == (Bv::<3>::of_u64(0x2_u64)))).into() {
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(8)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[15].input_type =
                        self.inputs.apb_request.pwdata.bit_range_to_bv::<3>(12, 3);
                }
                if (Bit::from((self.inputs.apb_request.pwdata.bit_as_bit(9)) != (Bit::F))).into() {
                    self.next_state_clk.inputs[15].event = Bit::F;
                }
            }
        }
        self.next_state_clk.inputs[0].sync_value = self.inputs.gpio_input.bit_as_bit(0);
        self.next_state_clk.inputs[0].last_sync_value = self.state_clk.inputs[0].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[0].value = self.state_clk.inputs[0].last_sync_value;
            match (self.state_clk.inputs[0].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[0].event =
                        !(Bit::from((self.state_clk.inputs[0].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[0].event = self.state_clk.inputs[0].value;
                }
                3 => {
                    self.next_state_clk.inputs[0].event = (self.state_clk.inputs[0].event)
                        | ((Bit::from((self.state_clk.inputs[0].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[0].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[0].event = (self.state_clk.inputs[0].event)
                        | ((!(Bit::from((self.state_clk.inputs[0].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[0].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[0].event = (self.state_clk.inputs[0].event)
                        | ((self.state_clk.inputs[0].value)
                            ^ (self.state_clk.inputs[0].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[1].sync_value = self.inputs.gpio_input.bit_as_bit(1);
        self.next_state_clk.inputs[1].last_sync_value = self.state_clk.inputs[1].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[1].value = self.state_clk.inputs[1].last_sync_value;
            match (self.state_clk.inputs[1].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[1].event =
                        !(Bit::from((self.state_clk.inputs[1].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[1].event = self.state_clk.inputs[1].value;
                }
                3 => {
                    self.next_state_clk.inputs[1].event = (self.state_clk.inputs[1].event)
                        | ((Bit::from((self.state_clk.inputs[1].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[1].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[1].event = (self.state_clk.inputs[1].event)
                        | ((!(Bit::from((self.state_clk.inputs[1].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[1].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[1].event = (self.state_clk.inputs[1].event)
                        | ((self.state_clk.inputs[1].value)
                            ^ (self.state_clk.inputs[1].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[2].sync_value = self.inputs.gpio_input.bit_as_bit(2);
        self.next_state_clk.inputs[2].last_sync_value = self.state_clk.inputs[2].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[2].value = self.state_clk.inputs[2].last_sync_value;
            match (self.state_clk.inputs[2].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[2].event =
                        !(Bit::from((self.state_clk.inputs[2].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[2].event = self.state_clk.inputs[2].value;
                }
                3 => {
                    self.next_state_clk.inputs[2].event = (self.state_clk.inputs[2].event)
                        | ((Bit::from((self.state_clk.inputs[2].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[2].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[2].event = (self.state_clk.inputs[2].event)
                        | ((!(Bit::from((self.state_clk.inputs[2].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[2].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[2].event = (self.state_clk.inputs[2].event)
                        | ((self.state_clk.inputs[2].value)
                            ^ (self.state_clk.inputs[2].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[3].sync_value = self.inputs.gpio_input.bit_as_bit(3);
        self.next_state_clk.inputs[3].last_sync_value = self.state_clk.inputs[3].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[3].value = self.state_clk.inputs[3].last_sync_value;
            match (self.state_clk.inputs[3].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[3].event =
                        !(Bit::from((self.state_clk.inputs[3].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[3].event = self.state_clk.inputs[3].value;
                }
                3 => {
                    self.next_state_clk.inputs[3].event = (self.state_clk.inputs[3].event)
                        | ((Bit::from((self.state_clk.inputs[3].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[3].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[3].event = (self.state_clk.inputs[3].event)
                        | ((!(Bit::from((self.state_clk.inputs[3].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[3].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[3].event = (self.state_clk.inputs[3].event)
                        | ((self.state_clk.inputs[3].value)
                            ^ (self.state_clk.inputs[3].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[4].sync_value = self.inputs.gpio_input.bit_as_bit(4);
        self.next_state_clk.inputs[4].last_sync_value = self.state_clk.inputs[4].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[4].value = self.state_clk.inputs[4].last_sync_value;
            match (self.state_clk.inputs[4].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[4].event =
                        !(Bit::from((self.state_clk.inputs[4].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[4].event = self.state_clk.inputs[4].value;
                }
                3 => {
                    self.next_state_clk.inputs[4].event = (self.state_clk.inputs[4].event)
                        | ((Bit::from((self.state_clk.inputs[4].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[4].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[4].event = (self.state_clk.inputs[4].event)
                        | ((!(Bit::from((self.state_clk.inputs[4].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[4].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[4].event = (self.state_clk.inputs[4].event)
                        | ((self.state_clk.inputs[4].value)
                            ^ (self.state_clk.inputs[4].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[5].sync_value = self.inputs.gpio_input.bit_as_bit(5);
        self.next_state_clk.inputs[5].last_sync_value = self.state_clk.inputs[5].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[5].value = self.state_clk.inputs[5].last_sync_value;
            match (self.state_clk.inputs[5].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[5].event =
                        !(Bit::from((self.state_clk.inputs[5].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[5].event = self.state_clk.inputs[5].value;
                }
                3 => {
                    self.next_state_clk.inputs[5].event = (self.state_clk.inputs[5].event)
                        | ((Bit::from((self.state_clk.inputs[5].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[5].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[5].event = (self.state_clk.inputs[5].event)
                        | ((!(Bit::from((self.state_clk.inputs[5].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[5].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[5].event = (self.state_clk.inputs[5].event)
                        | ((self.state_clk.inputs[5].value)
                            ^ (self.state_clk.inputs[5].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[6].sync_value = self.inputs.gpio_input.bit_as_bit(6);
        self.next_state_clk.inputs[6].last_sync_value = self.state_clk.inputs[6].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[6].value = self.state_clk.inputs[6].last_sync_value;
            match (self.state_clk.inputs[6].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[6].event =
                        !(Bit::from((self.state_clk.inputs[6].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[6].event = self.state_clk.inputs[6].value;
                }
                3 => {
                    self.next_state_clk.inputs[6].event = (self.state_clk.inputs[6].event)
                        | ((Bit::from((self.state_clk.inputs[6].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[6].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[6].event = (self.state_clk.inputs[6].event)
                        | ((!(Bit::from((self.state_clk.inputs[6].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[6].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[6].event = (self.state_clk.inputs[6].event)
                        | ((self.state_clk.inputs[6].value)
                            ^ (self.state_clk.inputs[6].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[7].sync_value = self.inputs.gpio_input.bit_as_bit(7);
        self.next_state_clk.inputs[7].last_sync_value = self.state_clk.inputs[7].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[7].value = self.state_clk.inputs[7].last_sync_value;
            match (self.state_clk.inputs[7].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[7].event =
                        !(Bit::from((self.state_clk.inputs[7].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[7].event = self.state_clk.inputs[7].value;
                }
                3 => {
                    self.next_state_clk.inputs[7].event = (self.state_clk.inputs[7].event)
                        | ((Bit::from((self.state_clk.inputs[7].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[7].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[7].event = (self.state_clk.inputs[7].event)
                        | ((!(Bit::from((self.state_clk.inputs[7].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[7].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[7].event = (self.state_clk.inputs[7].event)
                        | ((self.state_clk.inputs[7].value)
                            ^ (self.state_clk.inputs[7].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[8].sync_value = self.inputs.gpio_input.bit_as_bit(8);
        self.next_state_clk.inputs[8].last_sync_value = self.state_clk.inputs[8].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[8].value = self.state_clk.inputs[8].last_sync_value;
            match (self.state_clk.inputs[8].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[8].event =
                        !(Bit::from((self.state_clk.inputs[8].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[8].event = self.state_clk.inputs[8].value;
                }
                3 => {
                    self.next_state_clk.inputs[8].event = (self.state_clk.inputs[8].event)
                        | ((Bit::from((self.state_clk.inputs[8].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[8].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[8].event = (self.state_clk.inputs[8].event)
                        | ((!(Bit::from((self.state_clk.inputs[8].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[8].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[8].event = (self.state_clk.inputs[8].event)
                        | ((self.state_clk.inputs[8].value)
                            ^ (self.state_clk.inputs[8].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[9].sync_value = self.inputs.gpio_input.bit_as_bit(9);
        self.next_state_clk.inputs[9].last_sync_value = self.state_clk.inputs[9].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[9].value = self.state_clk.inputs[9].last_sync_value;
            match (self.state_clk.inputs[9].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[9].event =
                        !(Bit::from((self.state_clk.inputs[9].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[9].event = self.state_clk.inputs[9].value;
                }
                3 => {
                    self.next_state_clk.inputs[9].event = (self.state_clk.inputs[9].event)
                        | ((Bit::from((self.state_clk.inputs[9].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[9].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[9].event = (self.state_clk.inputs[9].event)
                        | ((!(Bit::from((self.state_clk.inputs[9].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[9].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[9].event = (self.state_clk.inputs[9].event)
                        | ((self.state_clk.inputs[9].value)
                            ^ (self.state_clk.inputs[9].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[10].sync_value = self.inputs.gpio_input.bit_as_bit(10);
        self.next_state_clk.inputs[10].last_sync_value = self.state_clk.inputs[10].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[10].value = self.state_clk.inputs[10].last_sync_value;
            match (self.state_clk.inputs[10].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[10].event =
                        !(Bit::from((self.state_clk.inputs[10].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[10].event = self.state_clk.inputs[10].value;
                }
                3 => {
                    self.next_state_clk.inputs[10].event = (self.state_clk.inputs[10].event)
                        | ((Bit::from((self.state_clk.inputs[10].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[10].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[10].event = (self.state_clk.inputs[10].event)
                        | ((!(Bit::from((self.state_clk.inputs[10].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[10].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[10].event = (self.state_clk.inputs[10].event)
                        | ((self.state_clk.inputs[10].value)
                            ^ (self.state_clk.inputs[10].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[11].sync_value = self.inputs.gpio_input.bit_as_bit(11);
        self.next_state_clk.inputs[11].last_sync_value = self.state_clk.inputs[11].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[11].value = self.state_clk.inputs[11].last_sync_value;
            match (self.state_clk.inputs[11].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[11].event =
                        !(Bit::from((self.state_clk.inputs[11].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[11].event = self.state_clk.inputs[11].value;
                }
                3 => {
                    self.next_state_clk.inputs[11].event = (self.state_clk.inputs[11].event)
                        | ((Bit::from((self.state_clk.inputs[11].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[11].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[11].event = (self.state_clk.inputs[11].event)
                        | ((!(Bit::from((self.state_clk.inputs[11].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[11].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[11].event = (self.state_clk.inputs[11].event)
                        | ((self.state_clk.inputs[11].value)
                            ^ (self.state_clk.inputs[11].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[12].sync_value = self.inputs.gpio_input.bit_as_bit(12);
        self.next_state_clk.inputs[12].last_sync_value = self.state_clk.inputs[12].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[12].value = self.state_clk.inputs[12].last_sync_value;
            match (self.state_clk.inputs[12].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[12].event =
                        !(Bit::from((self.state_clk.inputs[12].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[12].event = self.state_clk.inputs[12].value;
                }
                3 => {
                    self.next_state_clk.inputs[12].event = (self.state_clk.inputs[12].event)
                        | ((Bit::from((self.state_clk.inputs[12].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[12].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[12].event = (self.state_clk.inputs[12].event)
                        | ((!(Bit::from((self.state_clk.inputs[12].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[12].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[12].event = (self.state_clk.inputs[12].event)
                        | ((self.state_clk.inputs[12].value)
                            ^ (self.state_clk.inputs[12].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[13].sync_value = self.inputs.gpio_input.bit_as_bit(13);
        self.next_state_clk.inputs[13].last_sync_value = self.state_clk.inputs[13].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[13].value = self.state_clk.inputs[13].last_sync_value;
            match (self.state_clk.inputs[13].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[13].event =
                        !(Bit::from((self.state_clk.inputs[13].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[13].event = self.state_clk.inputs[13].value;
                }
                3 => {
                    self.next_state_clk.inputs[13].event = (self.state_clk.inputs[13].event)
                        | ((Bit::from((self.state_clk.inputs[13].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[13].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[13].event = (self.state_clk.inputs[13].event)
                        | ((!(Bit::from((self.state_clk.inputs[13].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[13].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[13].event = (self.state_clk.inputs[13].event)
                        | ((self.state_clk.inputs[13].value)
                            ^ (self.state_clk.inputs[13].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[14].sync_value = self.inputs.gpio_input.bit_as_bit(14);
        self.next_state_clk.inputs[14].last_sync_value = self.state_clk.inputs[14].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[14].value = self.state_clk.inputs[14].last_sync_value;
            match (self.state_clk.inputs[14].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[14].event =
                        !(Bit::from((self.state_clk.inputs[14].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[14].event = self.state_clk.inputs[14].value;
                }
                3 => {
                    self.next_state_clk.inputs[14].event = (self.state_clk.inputs[14].event)
                        | ((Bit::from((self.state_clk.inputs[14].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[14].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[14].event = (self.state_clk.inputs[14].event)
                        | ((!(Bit::from((self.state_clk.inputs[14].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[14].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[14].event = (self.state_clk.inputs[14].event)
                        | ((self.state_clk.inputs[14].value)
                            ^ (self.state_clk.inputs[14].last_sync_value));
                }
                _ => {}
            }
        }
        self.next_state_clk.inputs[15].sync_value = self.inputs.gpio_input.bit_as_bit(15);
        self.next_state_clk.inputs[15].last_sync_value = self.state_clk.inputs[15].sync_value;
        if (Bit::from((self.state_clk.access) == (Bv::<3>::default()))).into() {
            self.next_state_clk.inputs[15].value = self.state_clk.inputs[15].last_sync_value;
            match (self.state_clk.inputs[15].input_type).try_as_u64().unwrap() {
                1 => {
                    self.next_state_clk.inputs[15].event =
                        !(Bit::from((self.state_clk.inputs[15].value) != (Bit::F)));
                }
                2 => {
                    self.next_state_clk.inputs[15].event = self.state_clk.inputs[15].value;
                }
                3 => {
                    self.next_state_clk.inputs[15].event = (self.state_clk.inputs[15].event)
                        | ((Bit::from((self.state_clk.inputs[15].value) != (Bit::F)))
                            & (!(Bit::from(
                                (self.state_clk.inputs[15].last_sync_value) != (Bit::F),
                            ))));
                }
                4 => {
                    self.next_state_clk.inputs[15].event = (self.state_clk.inputs[15].event)
                        | ((!(Bit::from((self.state_clk.inputs[15].value) != (Bit::F))))
                            & (Bit::from((self.state_clk.inputs[15].last_sync_value) != (Bit::F))));
                }
                5 => {
                    self.next_state_clk.inputs[15].event = (self.state_clk.inputs[15].event)
                        | ((self.state_clk.inputs[15].value)
                            ^ (self.state_clk.inputs[15].last_sync_value));
                }
                _ => {}
            }
        }
    }

    fn propagate_locals_to_submodules(&mut self) {}
    fn propagate_all(&mut self) {
        self.propagate_resets();
        self.propagate_to_all_locals();
        self.propagate_locals_to_submodules();
    }
}
//ip Simulatable for apb_target_gpio
impl Simulatable for apb_target_gpio {
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
        self.reset_active_low_reset_n();
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
            self.eval_next_state_clk();
        }
        if mask.is_posedge(0) {
            self.state_clk = self.next_state_clk;
            if self.state_clk.access.try_as_u64().unwrap() != 0 {
                eprintln!(
                    "Access {:?} {:?}",
                    self.state_clk.access, self.inputs.apb_request.pwdata
                );
            }
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
        match index.as_usize() {
            0 => Some(SimStateInfo::clk("clk", 0)),
            _ => None,
        }
    }

    fn try_state_data(&self, index: SimStateIndex) -> Option<SimValueRef> {
        None
    }

    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        None
    }
}
//ip Component for apb_target_gpio
impl Component for apb_target_gpio {
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
//ip ComponentBuilder for apb_target_gpio
impl ComponentBuilder for apb_target_gpio {
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Self::default()
    }
}
//a Modules built
// se_external_module_register( 1, "apb_target_gpio", apb_target_gpio_instance_fn, "cdl_model" );
