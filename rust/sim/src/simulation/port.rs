//a Port
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
