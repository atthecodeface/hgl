//a Port
#[derive(Debug, Clone, Copy)]
pub struct PortInfo<'a> {
    name: &'a str,
    clock: bool,
}
impl<'a> PortInfo<'a> {
    pub const fn clk(name: &'a str) -> Self {
        Self { name, clock: true }
    }
    pub const fn data(name: &'a str) -> Self {
        Self { name, clock: false }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn is_clock(&self) -> bool {
        self.clock
    }
}
pub enum Port {
    Clock(usize),
    Input(usize),
    Output(usize),
}

impl Port {
    pub fn clock(n: usize) -> Self {
        Port::Clock(n)
    }
    pub fn input(n: usize) -> Self {
        Port::Input(n)
    }
    pub fn output(n: usize) -> Self {
        Port::Output(n)
    }
}
