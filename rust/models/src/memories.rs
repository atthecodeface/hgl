use hgl_sim::prelude::component::*;

//a STATE_INFO, Inputs, Outputs
//ci STATE_INFO
const STATE_INFO: &[SimStateInfo] = &[
    SimStateInfo::clk("clk", 0),
    SimStateInfo::input("read_enable", 1),
    SimStateInfo::input("write_enable", 2),
    SimStateInfo::input("address", 3),
    SimStateInfo::input("write_data", 4),
    SimStateInfo::output("read_valid", 0),
    SimStateInfo::output("read_data", 1),
];
#[derive(Debug, Default)]
pub struct Inputs<V, I>
where
    V: SimCopyValue,
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
    V: SimCopyValue,
{
    pub read_valid: Bit,
    pub read_data: V,
}

#[derive(Debug, Default)]
pub struct Memory<V, I>
where
    V: SimCopyValue,
    I: SimBv,
{
    size: usize,
    data: Vec<V>,
    inputs: Inputs<V, I>,
    outputs: Outputs<V>,
}

impl<V, I> Simulatable for Memory<V, I>
where
    V: SimCopyValue,
    I: SimBv,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn state_info(&self, index: SimStateIndex) -> Option<SimStateInfo> {
        STATE_INFO.get(index.as_usize()).copied()
    }
    fn try_state_data(&self, index: SimStateIndex) -> Option<SimValueRef> {
        match index.as_usize() {
            1 => Some(SimValueRef::of(&self.inputs.read_enable)),
            2 => Some(SimValueRef::of(&self.inputs.write_enable)),
            3 => Some(SimValueRef::of(&self.inputs.address)),
            4 => Some(SimValueRef::of(&self.inputs.write_data)),
            5 => Some(SimValueRef::of(&self.outputs.read_valid)),
            6 => Some(SimValueRef::of(&self.outputs.read_data)),
            _ => None,
        }
    }
    fn try_state_data_mut(&mut self, index: SimStateIndex) -> Option<SimValueRefMut> {
        match index.as_usize() {
            1 => Some(SimValueRefMut::of(&mut self.inputs.read_enable)),
            2 => Some(SimValueRefMut::of(&mut self.inputs.write_enable)),
            3 => Some(SimValueRefMut::of(&mut self.inputs.address)),
            4 => Some(SimValueRefMut::of(&mut self.inputs.write_data)),
            5 => Some(SimValueRefMut::of(&mut self.outputs.read_valid)),
            6 => Some(SimValueRefMut::of(&mut self.outputs.read_data)),
            _ => None,
        }
    }
    fn clock(&mut self, _mask: SimEdgeMask) {
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
    V: SimCopyValue,
    I: SimBv,
{
    type Config = usize;
    type InputsMut<'a> = &'a mut Inputs<V, I>;
    type Inputs<'a> = &'a Inputs<V, I>;
    type Outputs<'a> = &'a Outputs<V>;
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
    V: SimCopyValue,
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
    V: SimCopyValue,
    I: SimBv,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: SimNsName) -> Self {
        Memory::default()
    }
}
