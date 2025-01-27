//a Documentation
//! This library provides architecture/implementation specific CPU
//! counters for high precision timing
//!
//! The timers are really CPU tick counters, and so are not resilient
//! to threads being descheduled or being moved between CPU cores; the
//! library is designed for precise timing of short code sections
//! where the constraints are understood.
//!
//! # CPU support (for non-experimental Rustc target architectures)
//!
//! - [ ] x86_64 (implemented, not tested)
//! - [ ] x86    
//! - [x] aarch64
//! - [ ] wasm32
//!
//! Nonsupported architectures resort to the [std::time::Instant]
//! 'now' method
//!
//! # Types
//!
//! ## Timer
//!
//! The base type provided by this library is [Timer], which allows
//! for recording the delta in CPU ticks between the entry to a region
//! of code and the exit from it
//!
//! ```
//! # use hgl_utils::cpu_timer::Timer;
//! let mut t = Timer::default();
//! t.entry();
//! // do something!
//! t.exit();
//! println!("That took {} ticks", t.value());
//! ```
//!
//! ## AccTimer
//!
//! Frequently one will want to repeatedly time a piece of code, to
//! attain an average, or to just accumulate the time taken in some
//! code whenever it is called to determine if it is a 'hotspot'. The
//! [AccTimer] accumulates the time delta between entry and exit
//!
//! ```
//! # use hgl_utils::cpu_timer::AccTimer;
//! let mut t = AccTimer::default();
//! for i in 0..100 {
//!     t.entry();
//!     // do something!
//!     t.exit();
//!     println!("Iteration {i} took {} ticks", t.value());
//! }
//! println!("That an average of {} ticks", t.acc()/100);
//! ```
//!
//! # Ticks takenOS-specific notes
//!
//! These outputs are generated from tests/cpu_timer.rs, test_timer_values
//!
//! The tables will have a rough granularity of the time 'taken' to fetch a timer value
//!
//! ## MacOs aarch64 (MacBook Pro M4 Max Os15.1 rustc 1.84
//!
//! The granularity of the clock appears to be 41 or 42 ticks, and the
//! asm implementation seems to match the std time implementation.
//!
//! The average time taken for a call is 3 ticks in release, 9 ticks in debug
//!
//! | %age | arch release |   arch debug | std debug    | std release  |
//! |------|--------------|--------------|--------------|--------------|
//! | 10   |      0       |      41      |       41     |         0    |
//! | 25   |      0       |      42      |       42     |         0    |
//! | 50   |     41       |      42      |       42     |         0    |
//! | 75   |     41       |      42      |       83     |        41    |
//! | 90   |     42       |      83      |       83     |        41    |
//! | 95   |     42       |      83      |       83     |        41    |
//! | 99   |     42       |      84      |       84     |        42    |
//! | 100  |  27084       |   11125      |     2166     |      1125    |
//!
//! ### MacOs aarch64 std::time release
//!
//! Percentile distribution
//! 56, 0
//! 71, 41
//! 99, 42
//! 100, 1125
//!
//! ### MacOs aarch64 std::time debug
//!
//! Percentile distribution
//! 6, 41
//! 18, 42
//! 71, 83
//! 98, 84
//! 99, 125
//! 100, 2166
//!
//! ### MacOs aarch64 debug
//!
//! Percentile distribution
//! 22, 41
//! 66, 42
//! 88, 83
//! 99, 84
//! 100, 11125
//!
//! average of up to 95 9
//!
//! ### MacOs aarch64 release
//!
//! Percentile distribution
//! 40, 0
//! 60, 41
//! 99, 42
//! 100, 27084
//!
//! average of up to 95 3
//!
//! ## MacOs x86_64
//!
//! MacBook Pro 2018 Os 15.0 rustc 1.84 2.2GHz i7
//!
//! The granularity of the clock appears to be 2 ticks, and the
//! asm implementation is better than using the std::time implementation
//!
//! | %age | arch release |   arch debug | std debug    | std release  |
//! |------|--------------|--------------|--------------|--------------|
//! | 10   |     12       |      62      |       72     |        38    |
//! | 25   |     12       |      64      |       74     |        38    |
//! | 50   |     12       |      64      |       79     |        39    |
//! | 75   |     14       |      66      |       81     |        39    |
//! | 90   |     14       |      68      |       83     |        39    |
//! | 95   |     14       |      70      |       83     |        40    |
//! | 99   |     16       |      82      |      132     |        41    |
//! | 100  |  42918       |   65262      |    17101     |     24560    |
//!
//!
//! ### MacOs x86_64 release
//!
//! Percentile distribution
//! 5, 12
//! 73, 14
//! 99, 16
//! 100, 42918
//!
//! ### MacOs x86_64 debug
//!
//! Percentile distribution
//! 4, 62
//! 22, 64
//! 55, 66
//! 81, 68
//! 92, 70
//! 96, 72
//! 98, 74
//! 99, 82
//! 100, 65262    
//!
//! ### MacOs std::time debug
//!
//! Percentile distribution
//! 1, 70
//! 4, 71
//! 9, 72
//! 15, 73
//! 22, 74
//! 28, 75
//! 34, 76
//! 40, 77
//! 45, 78
//! 50, 79
//! 56, 80
//! 66, 81
//! 79, 82
//! 90, 83
//! 96, 84
//! 98, 85
//! 99, 132
//! 100, 17101
//!
//! ### MacOs std::time release
//!
//! Percentile distribution
//! 3, 37
//! 44, 38
//! 92, 39
//! 96, 40
//! 99, 41
//! 100, 24560

//a Imports
//a Constants
//cp TICKS_PER_US_APPLE_M4
pub const TICKS_PER_US_APPLE_M4: u64 = 1_000_000_000;

//a Delta
//ti Delta
/// A private type that is returned by get_timer, and which can be
/// used for all the timer calculations
///
/// This is used to abstract the internals from the public API
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
struct Delta(u64);

//ip From<u64> for Delta
impl From<u64> for Delta {
    #[inline(always)]
    fn from(t: u64) -> Self {
        Self(t)
    }
}

//ip From<Delta> for u64
impl From<Delta> for u64 {
    #[inline(always)]
    fn from(v: Delta) -> Self {
        v.0
    }
}

//ip From<u8/u16/u32/u128/usize> for Delta and back - needs u64 elsewhere
macro_rules! from_into_value {
    {$t:ty} => {
        impl From<Delta> for $t {
            #[inline(always)]
            fn from(v: Delta) -> Self {
                v.0 as $t
            }
        }
        impl From<$t> for Delta {
            #[inline(always)]
            fn from(t: $t) -> Self {
                (t as u64).into()
            }
        }
    }
}
from_into_value!(u8);
from_into_value!(u16);
from_into_value!(u32);
from_into_value!(u128);
from_into_value!(usize);

//ip Delta
impl Delta {
    //cp add
    /// Accmulate another delta into this value
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        self.0.wrapping_add(other.0).into()
    }
}

//a Architecture-specific get_timer functions
//fi get_timer for OTHER architectures
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64",)))]
mod arch {
    use super::Delta;
    #[derive(Debug, Clone, Copy)]
    pub struct Value(std::time::Instant);
    impl std::default::Default for Value {
        fn default() -> Self {
            Self(std::time::Instant::now())
        }
    }
    #[inline(always)]
    pub fn get_timer() -> Value {
        Value(std::time::Instant::now())
    }
    #[inline(always)]
    pub fn get_delta(since: &Value) -> Delta {
        since.0.elapsed().as_nanos().into()
    }
    #[inline(always)]
    pub fn delta_and_timer(since: &mut Value) -> Delta {
        let now = std::time::Instant::now();
        let delta = now - since.0;
        *since = Value(now);
        delta.as_nanos().into()
    }
}

//fi get_timer for Aarch64
/// Known to work on Apple M4 (MacbookPro 2024)
#[cfg(target_arch = "aarch64")]
mod arch {
    use super::Delta;
    use std::arch::asm;
    pub type Value = u64;
    #[inline(always)]
    pub fn get_timer() -> u64 {
        let timer: u64;
        unsafe {
            asm!(
                "isb
                mrs {timer}, cntvct_el0",
                timer = out(reg) timer,
            );
        }
        timer
    }
    #[inline(always)]
    pub fn get_delta(since: &Value) -> Delta {
        get_timer().wrapping_sub(*since).into()
    }
    #[inline(always)]
    pub fn delta_and_timer(since: &mut Value) -> Delta {
        let now = get_timer();
        let delta = now.wrapping_sub(*since).into();
        *since = now;
        delta
    }
}

//fi get_timer for x86_64
/// Not tested yet
#[cfg(target_arch = "x86_64")]
mod arch {
    use super::Delta;
    use std::arch::asm;
    pub type Value = u64;
    #[inline(always)]
    pub fn get_timer() -> Value {
        let lo: u64;
        let hi: u64;
        unsafe {
            asm!("rdtsc", lateout("eax") lo, lateout("edx") hi,
              options(nomem, nostack)
            );
        }
        hi << 32 | lo
    }
    #[inline(always)]
    pub fn get_delta(since: &Value) -> Delta {
        get_timer().wrapping_sub(*since).into()
    }
    #[inline(always)]
    pub fn delta_and_timer(since: &mut Value) -> Delta {
        let now = get_timer();
        let delta = now.wrapping_sub(*since).into();
        *since = now;
        delta
    }
}
//a Timer
//tp Timer
/// A timer that uses the underlying CPU clock ticks to generate
/// precise timings for short-term execution
///
/// This should *not* be expected to be correct in all cases; if a
/// thread sleeps or is interrupted, for example by the kernel, for
/// any reason, then the CPU timer value may not be useful; if the
/// thread migrates to a different CPU core it may become invalid; etc
///
/// The usage model is to capture an 'entry' time and an 'exit' time;
/// the *value* method can then be used to retrieve the CPU ticks
/// between the entry and exit
///
/// ```
/// # use hgl_utils::cpu_timer::Timer;
/// let mut t = Timer::default();
/// t.entry();
/// // do something!
/// t.exit();
/// println!("That took {} ticks", t.value());
/// ```
#[derive(Default, Debug)]
pub struct Timer {
    entry: arch::Value,
    delta: Delta,
}

//ip Timer
impl Timer {
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp entry
    /// Record the ticks on entry to a region-to-time
    #[inline(always)]
    pub fn entry(&mut self) {
        self.entry = arch::get_timer();
    }

    //mp delta
    /// Return (without updating) the delta since entry
    #[inline(always)]
    pub fn delta(&mut self) -> u64 {
        arch::get_delta(&self.entry).into()
    }

    //mp exit
    /// Record the ticks on exit from a region-to-time
    #[inline(always)]
    pub fn exit(&mut self) {
        self.delta = arch::get_delta(&self.entry);
    }

    //mp value
    /// Record the ticks on exit from a region-to-time, and update the
    /// accumulator
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.delta.into()
    }

    //mi raw
    /// Return the internal value for other methods in this library
    #[inline(always)]
    fn raw(&self) -> Delta {
        self.delta
    }
}

//a AccTimer
//tp AccTimer
/// An timer that accumulates the value for multiple timer entry-exits
///
#[derive(Default, Debug)]
pub struct AccTimer {
    timer: Timer,
    acc: Delta,
}

//ip AccTimer
impl AccTimer {
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp entry
    /// Record the ticks on entry to a region-to-time
    #[inline(always)]
    pub fn entry(&mut self) {
        self.timer.entry();
    }

    //mp exit
    /// Record the ticks on exit from a region-to-time
    #[inline(always)]
    pub fn exit(&mut self) {
        self.timer.exit();
        self.acc.add(self.timer.raw());
    }

    //mp value
    /// Record the ticks on exit from a region-to-time, and update the
    /// accumulator
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.timer.value()
    }

    //mp acc
    /// Read the accumulator value
    #[inline(always)]
    pub fn acc(&self) -> u64 {
        self.acc.into()
    }
}

//a TraceValue, Trace, AccTrace
//tt TraceValue
/// A value that can be stored in a Trace; this is implemented for u8,
/// u16, u32, u64 and usize
// Note that the type 'Delta' is private
#[allow(private_bounds)]
pub trait TraceValue: Default + Copy + From<Delta> + Into<Delta> {}

//ip TraceValue for u8/u16/u32/u64/usize
impl TraceValue for u8 {}
impl TraceValue for u16 {}
impl TraceValue for u32 {}
impl TraceValue for u64 {}
impl TraceValue for usize {}

//tp Trace
#[derive(Debug)]
pub struct Trace<T: TraceValue, const N: usize> {
    last: arch::Value,
    index: usize,
    trace: [T; N],
}

//ip Default for Trace
impl<T, const N: usize> std::default::Default for Trace<T, N>
where
    T: TraceValue,
    [T; N]: Default,
{
    fn default() -> Self {
        let last = arch::Value::default();
        let index = 0;
        let trace = <[T; N]>::default();
        Self { last, index, trace }
    }
}

//ip Trace
impl<T, const N: usize> Trace<T, N>
where
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        unsafe { *self = std::mem::zeroed() };
    }

    //mp entry
    /// Record the ticks on entry to a region-to-time
    #[inline(always)]
    pub fn entry(&mut self) {
        self.last = arch::get_timer();
        self.index = 0;
    }

    //mp next
    /// Record the ticks on exit from a region-to-time
    #[inline(always)]
    pub fn next(&mut self) {
        if self.index < N {
            let delta = arch::delta_and_timer(&mut self.last);
            self.trace[self.index] = delta.into();
            self.index += 1;
        }
    }

    //mp values
    /// Return the current trace
    pub fn values(&self) -> &[T; N] {
        &self.trace
    }
}

//tp AccTrace
#[derive(Debug)]
pub struct AccTrace<T: TraceValue, const N: usize> {
    trace: Trace<T, N>,
    acc: [T; N],
}

//ip Default for AccTrace
impl<T, const N: usize> std::default::Default for AccTrace<T, N>
where
    T: TraceValue,
    [T; N]: Default,
{
    fn default() -> Self {
        let trace = Trace::default();
        let acc = <[T; N]>::default();
        Self { trace, acc }
    }
}

//ip AccTrace
impl<T, const N: usize> AccTrace<T, N>
where
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        self.trace.clear();
        unsafe { self.acc = std::mem::zeroed() };
    }

    //mp entry
    /// Record the ticks on entry to a region-to-time
    #[inline(always)]
    pub fn entry(&mut self) {
        self.trace.entry();
    }

    //mp next
    /// Record the ticks on exit from a region-to-time
    #[inline(always)]
    pub fn next(&mut self) {
        self.trace.next();
    }

    //mp acc
    /// Accumulate the current trace into the accumulated trace
    pub fn acc(&mut self) {
        for i in 0..N {
            let v: Delta = self.acc[i].into();
            v.add(self.trace.trace[i].into());
            self.acc[i] = v.into();
        }
    }

    //mp last_trace
    /// Return the current trace
    pub fn last_trace(&self) -> &[T; N] {
        self.trace.values()
    }

    //mp acc_trace
    /// Return the accumulated trace
    pub fn acc_trace(&self) -> &[T; N] {
        &self.acc
    }
}
