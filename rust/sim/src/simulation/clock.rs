//a Imports
use crate::FullNameIndex;

//a Clock
//tp Clock
#[derive(Default)]
pub struct Clock {
    /// Name of the clock
    name: FullNameIndex,

    /// Delay to first posedge
    delay: usize,

    /// Period of the clock - time betweeen posedges
    period: usize,

    /// Offset from posedge to negedge - if 0, then effectively no
    /// negedge
    negedge_offset: usize,
}

//ip Clock
impl Clock {
    //cp new
    /// Create a new clock
    fn new(name: FullNameIndex, delay: usize, period: usize, negedge_offset: usize) -> Clock {
        Clock {
            name,
            delay,
            period,
            negedge_offset,
        }
    }
}

//a ClockPos
//tp ClockPos
#[derive(Default, Debug)]
pub struct ClockPos {
    /// Time of the next potential edge
    ///
    /// If this is before the initial delay of the clock
    next_edge: usize,

    /// Asserted if the next edge is a posedge
    ///
    /// If the clock is configured with no negedge (negedge_offset of
    /// 0) then this will always be asserted
    next_is_posedge: bool,
}

//ip ClockPos
impl ClockPos {
    //cp new
    fn new(clock: &Clock) -> Self {
        let next_edge = clock.delay % clock.period;
        let next_is_posedge = true;
        Self {
            next_edge,
            next_is_posedge,
        }
    }

    //mp next_time_and_edges
    /// At the current time, determine if this clock has a posedge or
    /// a negedge, or neither
    ///
    /// Also return the time of the next edge
    fn next_time_and_edges(&mut self, clock: &Clock, time: usize) -> (usize, bool, bool) {
        let enable_edge = time >= clock.delay;
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

//a Schedule
//tp Schedule
/// A [Schedule] is the running state of the set of clocks, with the
/// time of the next edge for each clock, and whether it is a negedge
/// or not
///
/// A [Schedule] is tied to the array of [Clock] that it corresponds to
#[derive(Default, Debug)]
pub struct Schedule {
    time: usize,
    next_time: usize,
    clock_pos: Vec<ClockPos>,
}

//ip Schedule
impl Schedule {
    //fi new
    /// Create a new schedule for the given set of [Clock]s
    fn new(clocks: &[Clock]) -> Self {
        let time = 0;
        let next_time = 0;
        let clock_pos: Vec<ClockPos> = clocks.iter().map(ClockPos::new).collect();
        Self {
            time,
            next_time,
            clock_pos,
        }
    }

    //fi next_edges
    /// Generate bitmasks of the posedges and negedges for the next
    /// time at which clock edges occur, and move the time on to that
    /// point
    ///
    /// This can only return two empty masks if there are no clock
    /// edges left before the end of time
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
            if earliest == usize::MAX {
                break;
            }
            if negedge_mask != 0 || posedge_mask != 0 {
                break;
            }
        }
        (posedge_mask, negedge_mask)
    }
}

//a ClockArray, ClockIndex
//tp ClockIndex
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClockIndex(usize);

//ip Index<ClockIndex> for ClockArray
impl std::ops::Index<ClockIndex> for ClockArray {
    type Output = Clock;
    fn index(&self, n: ClockIndex) -> &Clock {
        &self.clocks[n.0]
    }
}

//ip From<usize> for ClockIndex
impl From<usize> for ClockIndex {
    fn from(n: usize) -> ClockIndex {
        ClockIndex(n)
    }
}

//tp ClockArray
#[derive(Default)]
pub struct ClockArray {
    /// Clocks in the array
    clocks: Vec<Clock>,

    /// Current running schedule of the clocks
    ///
    /// This should be rebuilt when time is reset
    schedule: Option<Schedule>,
}

//ip ClockArray
impl ClockArray {
    #[track_caller]
    pub fn add_clock(
        &mut self,
        name: FullNameIndex,
        delay: usize,
        period: usize,
        negedge_offset: usize,
    ) -> ClockIndex {
        let clock = Clock {
            name,
            delay,
            period,
            negedge_offset,
        };
        assert!(period > 0, "Period of a clock must be at least one");
        assert!(
            negedge_offset < period,
            "Negedge offset must be less than the clock period"
        );
        let n = self.clocks.len();
        self.clocks.push(clock);
        n.into()
    }
    pub fn derive_schedule(&mut self) {
        if self.clocks.is_empty() {
            return;
        }
        self.schedule = Some(Schedule::new(&self.clocks));
    }
    #[track_caller]
    pub fn next_edges(&mut self) -> (usize, usize) {
        let Some(schedule) = &mut self.schedule else {
            panic!("Schedule has not been set up - no call of derive_schedule yet");
        };
        schedule.next_edges(&self.clocks)
    }
    #[track_caller]
    pub fn time(&self) -> usize {
        let Some(schedule) = &self.schedule else {
            panic!("Schedule has not been set up - no call of derive_schedule yet");
        };
        schedule.time
    }
}
