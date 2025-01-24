//a Imports
use std::collections::BTreeMap;

use crate::utils;

//a Clock
//tp Clock
#[derive(Default)]
pub struct Clock {
    name: String,
    delay: usize,
    period: usize,
    negedge_offset: usize,
}

//ip Clock
impl Clock {
    //cp new
    /// Create a new clock
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

//a Schedule
#[derive(Default, Debug)]
pub struct ClockPos {
    next_edge: usize,
    next_is_posedge: bool,
    initial_delay_remaining: usize,
}
impl ClockPos {
    fn new(clock: &Clock) -> Self {
        let next_edge = clock.delay % clock.period;
        let next_is_posedge = true;
        let initial_delay_remaining = clock.delay;
        Self {
            next_edge,
            next_is_posedge,
            initial_delay_remaining,
        }
    }
    fn next_time_and_edges(&mut self, clock: &Clock, time: usize) -> (usize, bool, bool) {
        let enable_edge = self.initial_delay_remaining <= time;
        if time < self.next_edge {
            (self.next_edge, false, false)
        } else if time == self.next_edge {
            if !self.next_is_posedge {
                self.next_edge += clock.period - clock.negedge_offset;
                self.next_is_posedge = true;
                (self.next_edge, false, enable_edge)
            } else if clock.negedge_offset > 0 {
                self.next_edge += clock.negedge_offset;
                self.next_is_posedge = false;
                (self.next_edge, enable_edge, false)
            } else {
                self.next_edge += clock.period;
                (self.next_edge, enable_edge, false)
            }
        } else {
            panic!("Clock moved beyond its edge!");
        }
    }
}
#[derive(Default, Debug)]
pub struct Schedule {
    time: usize,
    next_time: usize,
    clock_pos: Vec<ClockPos>,
}
impl Schedule {
    fn new(clocks: &[Clock]) -> Self {
        let time = 0;
        let next_time = 0;
        let clock_pos: Vec<ClockPos> = clocks.iter().map(|c| ClockPos::new(c)).collect();
        Self {
            time,
            next_time,
            clock_pos,
        }
    }
    fn next_edges(&mut self, clocks: &[Clock]) -> (usize, usize) {
        let mut negedge_mask = 0;
        let mut posedge_mask = 0;
        loop {
            self.time = self.next_time;
            let mut earliest = usize::MAX;
            for i in 0..self.clock_pos.len() {
                let (time, posedge, negedge) =
                    self.clock_pos[i].next_time_and_edges(&clocks[i], self.time);
                earliest = earliest.min(time);
                if posedge {
                    posedge_mask |= 1 << i;
                }
                if negedge {
                    negedge_mask |= 1 << i;
                }
            }
            self.next_time = earliest;
            if negedge_mask != 0 || posedge_mask != 0 {
                break;
            }
        }
        (posedge_mask, negedge_mask)
    }
}

//a ClockArray
//tp ClockArray
#[derive(Default)]
pub struct ClockArray {
    clocks: Vec<Clock>,
    schedule: Schedule,
}

//ip ClockArray
impl ClockArray {
    pub fn add_clock(
        &mut self,
        name: &str,
        delay: usize,
        period: usize,
        negedge_offset: usize,
    ) -> usize {
        let name = name.into();
        let clock = Clock {
            name,
            delay,
            period,
            negedge_offset,
        };
        let n = self.clocks.len();
        self.clocks.push(clock);
        n
    }
    pub fn derive_schedule(&mut self) {
        if self.clocks.is_empty() {
            return;
        }
        self.schedule = Schedule::new(&self.clocks);
    }
    pub fn next_edges(&mut self) -> (usize, usize) {
        self.schedule.next_edges(&self.clocks)
    }
    pub fn time(&self) -> usize {
        self.schedule.time
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
