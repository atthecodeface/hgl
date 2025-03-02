//a Import modules
mod counter;
mod memories;
mod register;
mod threaded;

pub mod alu;
pub mod nested_structures;

pub mod apb_target_gpio;
use hgl_sim::prelude::component::*;
#[derive(Debug, Default, Clone, Copy)]
pub struct t_apb_rom_request {
    pub enable: Bit,
    pub address: Bv<16>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_apb_processor_request {
    pub valid: Bit,
    pub address: Bv<16>,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_apb_processor_response {
    pub acknowledge: Bit,
    pub rom_busy: Bit,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_apb_response {
    pub prdata: Bv<32>,
    pub pready: Bit,
    pub perr: Bit,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct t_apb_request {
    pub paddr: Bv<32>,
    pub penable: Bit,
    pub psel: Bit,
    pub pwrite: Bit,
    pub pwdata: Bv<32>,
}

//a Export components
pub use counter::Counter;
pub use memories::Memory;
pub use register::Register;
pub use threaded::Threaded;
