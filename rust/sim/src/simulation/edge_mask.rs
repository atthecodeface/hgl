#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimEdgeMask(u64);
impl SimEdgeMask {
    pub fn none() -> Self {
        Self(0)
    }
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }
    pub const fn is_posedge(&self, input: usize) -> bool {
        ((self.0 as usize) >> (input * 2)) & 1 != 0
    }
    pub const fn is_negedge(&self, input: usize) -> bool {
        ((self.0 as usize) >> (input * 2)) & 2 != 0
    }
    #[must_use]
    pub fn add_posedge(mut self, input: usize) -> Self {
        self.0 |= (1 << (2 * input)) as u64;
        self
    }
    #[must_use]
    pub fn add_negedge(mut self, input: usize) -> Self {
        self.0 |= (2 << (2 * input)) as u64;
        self
    }
    pub fn set_posedge(&mut self, input: usize) {
        self.0 |= (1 << (2 * input)) as u64;
    }
    pub fn set_negedge(&mut self, input: usize) {
        self.0 |= (2 << (2 * input)) as u64;
    }
    /// Return true if self contains all of others
    pub fn contains_all(&self, others: &SimEdgeMask) -> bool {
        (self.0 & others.0) == others.0
    }
}
