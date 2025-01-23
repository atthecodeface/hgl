#[derive(Default)]
pub struct Clock {
    name: String,
    delay: usize,
    period: usize,
    negedge_offset: usize,
}

impl Clock {
    fn new(name: &str, delay: usize, period: usize, negedge_offset: usize) -> Clock {
        let name = name.into();
        Clock {
            name,
            delay,
            period,
            negedge_offset,
        }
    }
}

#[derive(Default)]
pub struct ClockArray {
    clocks: Vec<Clock>,
}
impl ClockArray {
    pub fn add_clock(&mut self, name: &str, delay: usize, period: usize, negedge_offset: usize) {
        let name = name.into();
        let clock = Clock {
            name,
            delay,
            period,
            negedge_offset,
        };
        self.clocks.push(clock);
    }
}
pub struct ClockIndex(usize);
impl std::ops::Index<ClockIndex> for ClockArray {
    type Output = Clock;
    fn index(&self, n: ClockIndex) -> &Clock {
        &self.clocks[n.0]
    }
}
impl From<usize> for ClockIndex {
    fn from(n: usize) -> ClockIndex {
        ClockIndex(n)
    }
}
