use hgl_sim::prelude::component::*;

#[derive(Debug, Default)]
pub struct Inputs<V>
where
    V: SimValue,
{
    clk: (),
    reset_n: Bit,
    enable: Bit,
    data: V,
}

#[derive(Debug, Default)]
pub struct Outputs<V>
where
    V: SimValue,
{
    data: V,
}

#[derive(Debug, Default)]
pub struct Register<V>
where
    V: SimValue,
{
    reset_value: Option<V>,
    inputs: Inputs<V>,
    outputs: Outputs<V>,
}

impl<V> Register<V>
where
    V: SimValue,
{
    pub fn new(reset_value: Option<V>) -> Self {
        let mut s = Self::default();
        s.reset_value = reset_value;
        s
    }
}

impl<V> Simulatable for Register<V>
where
    V: SimValue,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl<V> Component for Register<V>
where
    V: SimValue,
{
    type Config = Option<V>;
    type InputsMut<'a> = &'a mut Inputs<V>;
    type Inputs<'a> = &'a Inputs<V>;
    type Outputs<'a> = &'a Outputs<V>;
    fn inputs(&self) -> &Inputs<V> {
        &self.inputs
    }
    fn outputs(&self) -> &Outputs<V> {
        &self.outputs
    }
    fn inputs_mut(&mut self) -> &mut Inputs<V> {
        &mut self.inputs
    }
    fn configure<S: SimRegister>(
        &mut self,
        sim: &S,
        handle: S::Handle,
        config: Option<V>,
    ) -> Result<(), String> {
        self.reset_value = config;
        sim.register_input_edge(handle, 0, true, false);
        sim.register_input_edge(handle, 1, false, true);
        Ok(())
    }
}

impl<V> ComponentBuilder for Register<V>
where
    V: SimValue,
{
    type Build = Self;
    fn instantiate<S: SimRegister>(_sim: &mut S, _name: &FullName) -> Self {
        Self::default()
    }
}
