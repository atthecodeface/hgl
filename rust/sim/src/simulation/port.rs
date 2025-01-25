use std::any::TypeId;

//a Port
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    ClockInput,
    ClockOutput,
    Input,
    Output,
    Internal,
}
#[derive(Debug, Clone, Copy)]
pub struct PortInfo<'a> {
    name: &'a str,
    kind_index: usize,
    state_type: StateType,
}
impl<'a> PortInfo<'a> {
    pub const fn clk(name: &'a str, kind_index: usize) -> Self {
        Self {
            name,
            kind_index,
            state_type: StateType::ClockInput,
        }
    }
    pub const fn input(name: &'a str, kind_index: usize) -> Self {
        Self {
            name,
            kind_index,
            state_type: StateType::Input,
        }
    }
    pub const fn output(name: &'a str, kind_index: usize) -> Self {
        Self {
            name,
            kind_index,
            state_type: StateType::Output,
        }
    }
    pub const fn internal(name: &'a str, kind_index: usize) -> Self {
        Self {
            name,
            kind_index,
            state_type: StateType::Internal,
        }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn state_type(&self) -> StateType {
        self.state_type
    }
    pub fn kind_index(&self) -> usize {
        self.kind_index
    }
}

pub struct Port {
    state_index: usize,
    kind_index: usize,
    state_type: StateType,
    type_id: TypeId,
}

impl Port {
    pub fn new(state_index: usize, info: &PortInfo, type_id: Option<TypeId>) -> Self {
        let type_id = type_id.unwrap_or(std::any::TypeId::of::<()>());
        Port {
            state_index,
            kind_index: info.kind_index(),
            state_type: info.state_type(),
            type_id,
        }
    }
}
