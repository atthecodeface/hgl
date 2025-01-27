//a Imports
use std::collections::HashMap;

use hgl_indexed_vec::make_index;
use hgl_indexed_vec::{Idx, VecWithIndex};

use crate::simulation::{InstanceHandle, SimEdgeMask, SimNsName};

//a Clock
//tp Clock
#[derive(Default)]
pub struct Clock {
    /// Name of the clock
    name: SimNsName,

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
    #[track_caller]
    fn new(name: SimNsName, delay: usize, period: usize, negedge_offset: usize) -> Clock {
        assert!(period > 0, "Period of a clock must be at least one");
        assert!(
            negedge_offset < period,
            "Negedge offset must be less than the clock period"
        );
        Clock {
            name,
            delay,
            period,
            negedge_offset,
        }
    }

    //ap name
    pub fn name(&self) -> SimNsName {
        self.name
    }

    //ap period
    pub fn period(&self) -> usize {
        self.period
    }

    //ap delay
    pub fn delay(&self) -> usize {
        self.delay
    }

    //ap negedge_offset
    pub fn negedge_offset(&self) -> usize {
        self.negedge_offset
    }
}

//a ClockPosn
//tp ClockPosn
#[derive(Default, Debug)]
pub struct ClockPosn {
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

//ip ClockPosn
impl ClockPosn {
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
        use std::cmp::Ordering::*;
        match time.cmp(&self.next_edge) {
            Less => (self.next_edge, false, false),
            Equal => {
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
            }
            _ => {
                panic!("Clock moved beyond the next edge, bug in clock edge ordering code!");
            }
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
    clock_pos: Vec<ClockPosn>,
}

//ip Schedule
impl Schedule {
    //fi new
    /// Create a new schedule for the given set of [Clock]s
    fn new(clocks: &[Clock]) -> Self {
        let time = 0;
        let next_time = 0;
        let clock_pos: Vec<ClockPosn> = clocks.iter().map(ClockPosn::new).collect();
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
            for (i, _x) in clocks.iter().enumerate().take(self.clock_pos.len()) {
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
make_index!(ClockIndex, usize);

//tp ClockArray
#[derive(Default)]
struct ClockUse {
    instance: InstanceHandle,
    input: usize,
}

#[derive(Default)]
pub struct ClockArray {
    /// Clocks in the array
    clocks: VecWithIndex<SimNsName, ClockIndex, Clock>,

    /// Current running schedule of the clocks
    ///
    /// This should be rebuilt when time is reset
    schedule: Option<Schedule>,

    instance_edges: HashMap<(usize, usize), Vec<(InstanceHandle, SimEdgeMask)>>,
    clock_uses: HashMap<(ClockIndex, bool), Vec<ClockUse>>,
}

//ip ClockArray
impl ClockArray {
    //mp add_clock
    #[track_caller]
    pub fn add_clock(
        &mut self,
        name: SimNsName,
        delay: usize,
        period: usize,
        negedge_offset: usize,
    ) -> Result<ClockIndex, String> {
        let clock = Clock::new(name, delay, period, negedge_offset);
        self.clocks
            .insert(name, |_| clock)
            .map_err(|_| format!("Clock already exists"))
    }

    //mp find_clock
    pub fn find_clock(&self, name: SimNsName) -> Option<ClockIndex> {
        self.clocks.find_key(&name)
    }

    //mp derive_schedule
    pub fn derive_schedule(&mut self) {
        if self.clocks.is_empty() {
            return;
        }
        self.schedule = Some(Schedule::new(self.clocks.as_ref()));
    }

    //mp edge_used_by
    pub fn edge_used_by(
        &mut self,
        clock: ClockIndex,
        instance: InstanceHandle,
        input: usize,
        posedge: bool,
    ) {
        self.clock_uses
            .entry((clock, posedge))
            .or_insert_with(|| vec![])
            .push(ClockUse { instance, input });
    }

    //mp derive_instance_edges_of_masks
    pub fn derive_instance_edges_of_masks(&mut self, ie: &(usize, usize)) {
        if self.instance_edges.contains_key(&ie) {
            return;
        }
        let mut blah: HashMap<InstanceHandle, SimEdgeMask> = HashMap::new();
        for i in 0..self.clocks.len() {
            if (ie.0 >> i) & 1 == 1 {
                if let Some(x) = self.clock_uses.get(&(ClockIndex::from_usize(i), true)) {
                    for c in x.iter() {
                        blah.entry(c.instance).or_default().set_posedge(c.input);
                    }
                }
            }
            if (ie.1 >> i) & 1 == 1 {
                if let Some(x) = self.clock_uses.get(&(ClockIndex::from_usize(i), false)) {
                    for c in x.iter() {
                        blah.entry(c.instance).or_default().set_negedge(c.input);
                    }
                }
            }
        }
        let blah: Vec<(InstanceHandle, SimEdgeMask)> = blah.into_iter().collect();
        self.instance_edges.insert(*ie, blah);
    }

    //mp next_edges
    #[track_caller]
    pub fn next_edges(&mut self) -> (usize, usize) {
        let Some(schedule) = &mut self.schedule else {
            panic!("Schedule has not been set up - no call of derive_schedule yet");
        };
        let ie = schedule.next_edges(&self.clocks.as_ref());
        if !self.instance_edges.contains_key(&ie) {
            self.derive_instance_edges_of_masks(&ie);
        }
        ie
    }

    //mp instance_edges
    pub fn instance_edges(&self, edges: &(usize, usize)) -> &[(InstanceHandle, SimEdgeMask)] {
        let Some(ie) = self.instance_edges.get(edges) else {
            return &[];
        };
        &*ie
    }

    //ap time
    #[track_caller]
    pub fn time(&self) -> usize {
        let Some(schedule) = &self.schedule else {
            panic!("Schedule has not been set up - no call of derive_schedule yet");
        };
        schedule.time
    }

    //ap into_iter
    pub fn into_iter(&self) -> impl std::iter::Iterator<Item = &Clock> {
        self.clocks.into_iter()
    }
}
