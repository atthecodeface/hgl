use crate::SimValueObject;

//a Port
#[derive(Debug)]
pub struct PortData<'a> {
    value: &'a (dyn SimValueObject),
}

impl<'a> PortData<'a> {
    pub fn of(value: &'a dyn SimValueObject) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &dyn SimValueObject {
        self.value
    }
    pub fn as_any(&self) -> &dyn std::any::Any {
        self.value.as_any()
    }
}

#[derive(Debug)]
pub struct PortDataMut<'a> {
    value: &'a mut (dyn SimValueObject),
}

impl<'a> PortDataMut<'a> {
    pub fn of(value: &'a mut dyn SimValueObject) -> Self {
        Self { value }
    }
    pub fn try_copy_from(&mut self, other: &PortData) -> bool {
        if other.as_any().type_id() != self.as_any().type_id() {
            false
        } else {
            let Some(size) = self.value.try_as_u8s_mut() else {
                return false;
            };
            let Some(osize) = other.value.try_as_u8s() else {
                return false;
            };
            assert_eq!(
                size.len(),
                osize.len(),
                "Sizes of port data to copy must match"
            );
            size.copy_from_slice(osize);
            true
        }
    }
    pub fn set_u8s(&mut self, data: &[u8]) -> bool {
        let Some(size) = self.value.try_as_u8s_mut() else {
            return false;
        };
        if data.len() != size.len() {
            dbg!(data, size);
            return false;
        }
        size.copy_from_slice(data);
        true
    }
    pub fn value(&self) -> &dyn SimValueObject {
        self.value
    }
    pub fn as_any(&self) -> &dyn std::any::Any {
        self.value.as_any()
    }
    // pub fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
    // self.value.as_any()
    // }
}

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
