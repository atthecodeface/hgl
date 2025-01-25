//a Imports
use crate::traits::SimValue;

//a SimFormatValue
//tp SimFormatValue
/// A wrapper to permit formatting of a value as a string; this can be constructed, then printed with
///
/// SimFormatValue::value_string(&Bit::F, fmt::AS_HEX | fmt::AS_BIN)
pub struct SimFormatValue<'a, T: SimValue> {
    value: &'a T,
    style: usize,
}

//ip SimFormatValue
impl<'a, T: SimValue> SimFormatValue<'a, T> {
    pub fn new(value: &'a T, style: usize) -> Self {
        Self { value, style }
    }
    pub fn value_string(value: &T, style: usize) -> String {
        SimFormatValue { value, style }.to_string()
    }
}

//ip Display for SimFormatValue
impl<'a, T: SimValue> std::fmt::Display for SimFormatValue<'a, T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt_with(fmt, self.style)
    }
}
