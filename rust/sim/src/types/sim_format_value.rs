use crate::SimValue;
pub struct SimFormatValue<'a, T: SimValue> {
    value: &'a T,
    style: usize,
}
impl<'a, T: SimValue> SimFormatValue<'a, T> {
    pub fn new(value: &'a T, style: usize) -> Self {
        Self { value, style }
    }
    pub fn value_string(value: &T, style: usize) -> String {
        format!("{:?}", &(SimFormatValue { value, style }))
    }
}
impl<'a, T: SimValue> std::fmt::Debug for SimFormatValue<'a, T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt_with(fmt, self.style)
    }
}
