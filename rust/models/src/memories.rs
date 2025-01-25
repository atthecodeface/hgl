use hgl_sim::prelude::component::*;

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[PortInfo] = &[
    PortInfo::clk("clk", 0),
    PortInfo::input("read_enable", 1),
    PortInfo::input("write_enable", 2),
    PortInfo::input("address", 3),
    PortInfo::input("write_data", 4),
    PortInfo::output("read_valid", 0),
    PortInfo::output("read_data", 1),
];
#[derive(Debug, Default)]
pub struct Inputs<V, I>
where
    V: SimValue,
    I: SimBv,
{
    pub read_enable: Bit,
    pub write_enable: Bit,
    pub address: I,
    pub write_data: V,
}

#[derive(Debug, Default)]
pub struct Outputs<V>
where
    V: SimValue,
{
    pub read_valid: Bit,
    pub read_data: V,
}

#[derive(Debug, Default)]
pub struct Memory<V, I>
where
    V: SimValue,
    I: SimBv,
{
    size: usize,
    data: Vec<V>,
    inputs: Inputs<V, I>,
    outputs: Outputs<V>,
}

impl<V, I> Simulatable for Memory<V, I>
where
    V: SimValue,
    I: SimBv,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn clock(&mut self, _mask: u32) {
        let read = self.inputs.read_enable.is_true();
        let write = self.inputs.write_enable.is_true();
        self.outputs.read_valid = read.into();
        if read || write {
            let address = self.inputs.address.try_as_u64().unwrap() as usize;
            if address > self.size {
                panic!("Attempt to access memory out of range");
            }
            if address >= self.data.len() {
                self.data.resize(address + 1, V::default());
            }
            if read {
                self.outputs.read_data = self.data[address];
            }
            if write {
                self.data[address] = self.inputs.write_data;
            }
        }
    }
}
impl<V, I> Component for Memory<V, I>
where
    V: SimValue,
    I: SimBv,
{
    type Config = usize;
    type InputsMut<'a> = &'a mut Inputs<V, I>;
    type Inputs<'a> = &'a Inputs<V, I>;
    type Outputs<'a> = &'a Outputs<V>;
    fn state_info(&self, index: usize) -> Option<PortInfo> {
        STATE_INFO.get(index).copied()
    }
    fn try_state_data(&self, index: usize) -> Option<SimValueRef> {
        if index == 0 {
            Some(SimValueRef::of(&self.inputs.address))
        } else {
            None
        }
    }
    fn try_state_data_mut(&mut self, index: usize) -> Option<SimValueRefMut> {
        if index == 0 {
            Some(SimValueRefMut::of(&mut self.inputs.address))
        } else {
            None
        }
    }
    fn inputs(&self) -> &Inputs<V, I> {
        &self.inputs
    }
    fn outputs(&self) -> &Outputs<V> {
        &self.outputs
    }
    fn inputs_mut(&mut self) -> &mut Inputs<V, I> {
        &mut self.inputs
    }
    fn configure<S: SimRegister>(
        &mut self,
        sim: &S,
        handle: S::Handle,
        config: usize,
    ) -> Result<(), String> {
        if self.inputs.address.try_as_u64().is_none() {
            return Err("Address for memory must map to u64".into());
        }
        self.size = config;
        sim.register_input_edge(handle, 0, true, false);
        Ok(())
    }
}

impl<V, I> Memory<V, I>
where
    V: SimValue,
    I: SimBv,
{
    pub fn new(size: usize) -> Self {
        Memory::<V, I> {
            size,
            ..Default::default()
        }
    }
}

impl<V, I> ComponentBuilder for Memory<V, I>
where
    V: SimValue,
    I: SimBv,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Memory::default()
    }
}
