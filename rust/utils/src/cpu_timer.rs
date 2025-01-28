//a Documentation
//! This library provides architecture/implementation specific CPU
//! counters for high precision timing, backed up by a std::time
//! implementation where an architecture has no explicit CPU support
//!
//! The timers are really CPU tick counters, and so are not resilient
//! to threads being descheduled or being moved between CPU cores; the
//! library is designed for precise timing of short code sections
//! where the constraints are understood. Furthermore, the timer
//! values are thus not in seconds but in other arbitrary units -
//! useful for comparing execution of different parts of code, but
//! requiring another mechanism to determine the mapping from ticks to
//! seconds
//!
//! # Precision
//!
//! For some architectures a real CPU ASM instruction is used to get
//! the tick count. For x86_64 this returns (in an unvirtualized
//! world) the real CPU tick counter, with a fine precision. For
//! Aarch64 on MacOs this is no better than using std::time, and has a
//! precision of about 40 ticks. However, the asm implementation has a
//! lower overhead on Aarch64 on MacOs, so it is still worth using.
//!
//! The library does not attempt to take into account any overheads of
//! using the timers; that is for the user. Normally the overheads
//! will be small compared to the times being measured.
//!
//! # CPU support (for non-experimental Rustc target architectures)
//!
//! For the stable Rustc-supported architectures, CPU implementations
//! are provided for:
//!
//! - [ ] x86    
//! - [x] x86_64
//! - [x] aarch64
//! - [ ] wasm32
//!
//! Nonsupported architectures resort to the [std::time::Instant]
//! 'now' method instead (which can be perfectly adequate)
//!
//! # Types
//!
//! The types in the library are all generic on *UseAsm* whether the CPU
//! architecture specific version (if provided) of the timer should be
//! used, or if std::time should be used instead. For architectures
//! without a CPU implementation, the std::time version is used
//! whatever the value of the generic.
//!
//! ## DeltaTimer
//!
//! The base type provided by this library is [Timer], which allows
//! for recording the delta in CPU ticks between the entry to a region
//! of code and the exit from it. It uses a generic *UseAsm* bool.
//!
//! ```
//! # use hgl_utils::cpu_timer::DeltaTimer;
//! let mut t = DeltaTimer::<true>::default();
//! t.start();
//! // do something! - timed using CPU ticks
//! t.stop();
//! println!("That took {} cpu 'ticks'", t.value());
//!
//! let mut t = DeltaTimer::<false>::default();
//! t.start();
//! // do something! - timed using std::time
//! t.stop();
//! println!("That took {} nanoseconds", t.value());
//! ```
//!
//! ## AccTimer
//!
//! Frequently one will want to repeatedly time a piece of code, to
//! attain an average, or to just accumulate the time taken in some
//! code whenever it is called to determine if it is a 'hotspot'. The
//! [AccTimer] accumulates the time delta between start and stop.
//!
//! ```
//! # use hgl_utils::cpu_timer::AccTimer;
//! let mut t = AccTimer::<true>::default();
//! for i in 0..100 {
//!     t.start();
//!     // do something!
//!     t.stop();
//!     println!("Iteration {i} took {} ticks", t.last_delta());
//! }
//! println!("That took an average of {} ticks", t.acc_value()/100);
//! ```
//!
//! ## AccVec
//!
//! An [AccVec] is used to accumulate timer values, storing not just
//! the times but also (optionally) the number of occurrences.
//!
//! It is used as `AccVec<A, T, C, N>`; A is a bool; T the time accumulator type; C the counter type; N the number of accumulators.
//!
//!  * A is true if the CPU-specific timer should be used, false if
//!    std::time should be used
//!
//!  * T is the type used for accumulating time deltas
//!
//! ## Trace
//!
//! The [Trace] type supports tracing the execution path through some
//! logic, getting deltas along the way
//!
//! ```
//! # use hgl_utils::cpu_timer::Trace;
//! let mut t = Trace::<true, u32, 3>::default();
//! t.start();
//!   // do something!
//! t.next();
//!   // do something else!
//! t.next();
//!   // do something else!
//! t.next();
//! println!("The three steps took {:?} ticks", t.trace());
//! ```
//!
//! The trace will have three entries, which are the delta times for
//! the three operations.
//!
//! ## AccTrace
//!
//! The [AccTrace] accumulates a number of iterations of a Trace;
//!
//! ```
//! # use hgl_utils::cpu_timer::AccTrace;
//! struct MyThing {
//!     // things ...
//!     /// For timing (perhaps only if #[cfg(debug_assertions)] )
//!     acc: AccTrace::<true, u32,4>,
//! }
//!
//! impl MyThing {
//!     fn do_something_complex(&mut self) {
//!         self.acc.start();
//!         // .. do first complex thing
//!         self.acc.next();
//!         // .. do second complex thing
//!         self.acc.next();
//!         // .. do third complex thing
//!         self.acc.next();
//!         // .. do fourth complex thing
//!         self.acc.next();
//!         self.acc.acc();
//!     }
//! }
//!
//! let mut t = MyThing { // ..
//!     acc: AccTrace::<true, u32, 4>::default()
//! };
//! for _ in 0..100 {
//!     t.do_something_complex();
//! }
//! println!("After 100 iterations the accumulated times for the four steps is {:?} ticks", t.acc.acc_trace());
//! t.acc.clear();
//! // ready to be complex all again
//! ```
//!
//! The trace will have four entries, which are the accumulated delta times for
//! the four complex things.
//!
//! # OS-specific notes
//!
//! These outputs are generated from tests/cpu_timer.rs, test_timer_values
//!
//! The tables will have a rough granularity of the precision of the
//! tick counter. Average time taken is calculated using the fastest
//! 95% of 10,000 calls, as beyond that the outliers should be ignored.
//!
//! ## MacOs aarch64 (MacBook Pro M4 Max Os15.1 rustc 1.84
//!
//! The granularity of the clock appears to be 41 or 42 ticks, and the
//! asm implementation seems to match the std time implementation for this precision.
//!
//! For asm, the average time taken for a call is 3 ticks in release, 9 ticks in debug
//!
//! For std::time, the average time taken for a call is 8 ticks in
//! release, 17 ticks in debug. So clearly there is an overhead for
//! using std::time
//!
//! | %age | arch release |   arch debug | std debug    | std release  |
//! |------|--------------|--------------|--------------|--------------|
//! | 10   |      0       |       0      |       41     |         0    |
//! | 25   |      0       |       0      |       42     |         0    |
//! | 50   |      0       |       0      |       42     |         0    |
//! | 75   |      0       |      41      |       83     |        41    |
//! | 90   |     42       |      41      |       83     |        41    |
//! | 95   |     42       |      41      |       83     |        41    |
//! | 99   |     42       |      42      |       84     |        42    |
//! | 100  |  27084       |    2498      |     2166     |      1125    |
//!
//! ### MacOs aarch64 std::time release
//!
//! Percentile distribution
//! 56, 0
//! 71, 41
//! 99, 42
//! 100, 1125
//!
//! average of up to 95 8
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
//! average of up to 95 17
//!
//! ### MacOs aarch64 debug
//!
//! Percentile distribution
//! 52, 0
//! 68, 41
//! 99, 42
//! 100, 2958
//!
//! average of up to 95 9
//!
//! ### MacOs aarch64 release
//!
//! Percentile distribution
//! 77, 0
//! 85, 41
//! 99, 42
//! 100, 1500
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
//! The average time taken for a call is 15 ticks in release, 78 (but
//! sometimes 66!) ticks in debug
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
//! average of up to 95 15
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
//! average of up to 95 78
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

//ip Delta
impl Delta {
    //cp add
    /// Accmulate another delta into this value
    #[inline(always)]
    #[must_use]
    fn add(self, other: Self) -> Self {
        self.0.wrapping_add(other.0).into()
    }

    //cp sat_add
    /// Accmulate another delta into this value
    #[inline(always)]
    #[must_use]
    fn sat_add(self, other: Self) -> Self {
        self.0.saturating_add(other.0).into()
    }
}

//a Private module to allow sealing of traits
//mi private
/// This module is private to seal the TArch trait, which must be
/// implemented here only.
mod private {
    //iu Delta
    use super::Delta;
    //tp Value
    pub(super) trait Value: std::fmt::Debug + Default + Copy {
        fn since(self, last: Self) -> Delta;
        fn since_and_update(&mut self, now: Self) -> Delta;
    }
    impl Value for u64 {
        fn since(self, last: Self) -> Delta {
            Delta(self.wrapping_sub(last))
        }
        fn since_and_update(&mut self, now: Self) -> Delta {
            let delta = now.wrapping_sub(*self);
            *self = now;
            Delta(delta)
        }
    }

    //tp ArchDesc
    pub(super) trait ArchDesc: Default {
        /// Value returned by the timer
        ///
        /// This is stored within timers but is not visible to users
        type Value: Value;

        //fp get_timer
        /// Get the current value of the timer
        fn get_timer() -> Self::Value;
    }

    //tt TraceValue
    pub(super) trait TraceValue: Default + Copy + From<Delta> + Into<Delta> {
        fn sat_add(self, other: Self) -> Self;
    }
}
use private::Value;

//a TraceCount
//tt TraceCount
/// A value that can be stored in a Trace; this is implemented for u8,
/// u16, u32, u64 and usize
pub trait TraceCount: Default + Copy {
    fn sat_inc(&mut self);
    fn as_usize(self) -> usize;
}

//ip TraceCount for ()
impl TraceCount for () {
    fn sat_inc(&mut self) {}
    fn as_usize(self) -> usize {
        0
    }
}

//ip TraceCount for u8/u16/u32/u64/u128/usize
macro_rules! trace_count {
    {$t:ty} => {
        impl TraceCount for $t {
            #[inline(always)]
            fn sat_inc(&mut self) {
                if *self != Self::MAX {*self = self.wrapping_add(1);}
            }
            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }
        }
    }
}
trace_count!(u8);
trace_count!(u16);
trace_count!(u32);
trace_count!(u64);
trace_count!(u128);
trace_count!(usize);

//a TraceValue
//tt TraceValue
/// A value that can be stored in a Trace; this is implemented for u8,
/// u16, u32, u64 and usize
// Note that the type 'Delta' is private
#[allow(private_bounds)]
pub trait TraceValue: private::TraceValue {}

//ip TraceValue for T: private::TraceValue
impl<T> TraceValue for T where T: private::TraceValue {}

//ip From<()> for Delta
impl From<()> for Delta {
    #[inline(always)]
    fn from(_t: ()) -> Self {
        Self(0)
    }
}

//ip From<Delta> for ()
impl From<Delta> for () {
    #[inline(always)]
    fn from(_v: Delta) -> Self {}
}

//ip private::TraceValue for ()
impl private::TraceValue for () {
    fn sat_add(self, _other: Self) -> Self {}
}

//ip TraceValue for u8/u16/u32/u64/u128/usize
macro_rules! trace_value {
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
                Delta(t as u64)
            }
        }
        impl private::TraceValue for $t {
            fn sat_add(self, other:Self) -> Self {
                self.saturating_add(other)
            }
        }
    }
}
trace_value!(u8);
trace_value!(u16);
trace_value!(u32);
trace_value!(u64);
trace_value!(u128);
trace_value!(usize);

//a Architecture-specific and standard get_timer functions
//tt TArch
/// Trait provided for architecture-specific timers
///
/// This is supported by a single assembler timer and a standard
/// (std::time) timer
#[allow(private_bounds)]
pub trait TArch: private::ArchDesc {}

//ip TArch for T: private::ArchDesc
impl<T> TArch for T where T: private::ArchDesc {}

//tp TDesc
/// Marker type generic on a bool, which has the 'TArch' trait
/// implemented for it for (true) an assembler architecture specific
/// timer implementation, and (false) for a std::time implementation
#[derive(Default)]
pub struct TDesc<const B: bool>();

//tp Asm
/// Marker type for which TDesc is implemented for both true and false
#[derive(Default)]
pub struct Asm(());

//ip TArch for TDesc<true>
// Assembler specific implementation of a
// timer architecture
//
// If the architecture does not have an assembler implementation then
// this will actually be the std::time implementation
impl private::ArchDesc for TDesc<true> {
    type Value = arch::Value;
    #[inline(always)]
    fn get_timer() -> Self::Value {
        arch::get_timer()
    }
}

//ip TArch for TDesc<false>
// std::time implementation of a
// timer architecture
impl private::ArchDesc for TDesc<false> {
    type Value = arch_std::Value;
    #[inline(always)]
    fn get_timer() -> Self::Value {
        arch_std::get_timer()
    }
}

//a Architecture specific and standard timer implementation modules
//mi Standard architecture implementation of a timer
mod arch_std {
    use super::Delta;
    #[derive(Debug, Clone, Copy)]
    pub struct Value(std::time::Instant);
    impl super::private::Value for Value {
        fn since(self, last: Self) -> Delta {
            (self.0 - last.0).as_nanos().into()
        }
        fn since_and_update(&mut self, now: Self) -> Delta {
            let delta = (now.0 - self.0).as_nanos().into();
            *self = now;
            delta
        }
    }
    impl std::default::Default for Value {
        fn default() -> Self {
            Self(std::time::Instant::now())
        }
    }
    #[inline(always)]
    pub fn get_timer() -> Value {
        Value(std::time::Instant::now())
    }
}

//mi get_timer for OTHER architectures
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64",)))]
use arch_std as arch;

//fi get_timer for Aarch64
/// Known to work on Apple M4 (MacbookPro 2024)
#[cfg(target_arch = "aarch64")]
mod arch {
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
            asm!(
                "ldfence
                rdtsc",
                lateout("eax") lo,
                lateout("edx") hi,
              options(nomem, nostack)
            );
        }
        hi << 32 | lo
    }
}

//a BaseTimer
//tp BaseTimer
/// A basic timer that just contains the timer value
#[derive(Default, Debug)]
pub struct BaseTimer<const S: bool>
where
    TDesc<S>: TArch,
{
    start: <TDesc<S> as private::ArchDesc>::Value,
}

//ip BaseTimer
impl<const S: bool> BaseTimer<S>
where
    TDesc<S>: TArch,
{
    //mi now
    #[inline(always)]
    fn now() -> <TDesc<S> as private::ArchDesc>::Value {
        <TDesc<S> as private::ArchDesc>::get_timer()
    }

    //mp start
    /// Record the time now
    #[inline(always)]
    pub fn start(&mut self) {
        self.start = Self::now();
    }

    //mp elapsed_delta
    /// Return the Delta between now and self.start
    #[inline(always)]
    fn elapsed_delta(&self) -> Delta {
        Self::now().since(self.start)
    }

    //mp elapsed_delta_and_update
    /// Record the delta time since the last start
    #[inline(always)]
    fn elapsed_delta_and_update(&mut self) -> Delta {
        self.start.since_and_update(Self::now())
    }

    //ap elapsed
    /// Return the time elapsed as a u64
    #[inline(always)]
    pub fn elapsed(&self) -> u64 {
        self.elapsed_delta().into()
    }

    //mp elapsed_and_update
    /// Return the time elapsed as a u64, and update the timer
    #[inline(always)]
    pub fn elapsed_and_update(&mut self) -> u64 {
        self.elapsed_delta_and_update().into()
    }
}

//a DeltaTimer
//tp DeltaTimer
/// A timer that uses the underlying CPU clock ticks to generate
/// precise timings for short-term execution
///
/// This should *not* be expected to be correct in all cases; if a
/// thread sleeps or is interrupted, for example by the kernel, for
/// any reason, then the CPU timer value may not be useful; if the
/// thread migrates to a different CPU core it may become invalid; etc
///
/// The usage model is to capture a 'start' time and a 'stop' time;
/// the *value* method can then be used to retrieve the CPU ticks
/// between the start and stop
///
/// ```
/// # use hgl_utils::cpu_timer::DeltaTimer;
/// let mut t = DeltaTimer::<true>::default();
/// t.start();
/// // do something!
/// t.stop();
/// println!("That took {} ticks", t.value());
/// ```
#[derive(Default, Debug)]
pub struct DeltaTimer<const S: bool>
where
    BaseTimer<S>: Default,
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    delta: Delta,
}

//ip DeltaTimer
impl<const S: bool> DeltaTimer<S>
where
    TDesc<S>: TArch,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp start
    /// Record the ticks at the start of the timer
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp delta
    /// Return (without updating) the delta since start
    #[inline(always)]
    pub fn delta(&mut self) -> u64 {
        self.base.elapsed_delta().into()
    }

    //mp stop
    /// Record the delta time since the last start
    #[inline(always)]
    pub fn stop(&mut self) {
        self.delta = self.base.elapsed_delta();
    }

    //mp value
    /// Return the delta time in ticks
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.delta.into()
    }
}

//a AccTimer
//tp AccTimer
/// An timer that accumulates the value for multiple timer start-stops
///
#[derive(Default, Debug)]
pub struct AccTimer<const S: bool>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    delta: Delta,
    acc: Delta,
}

//ip AccTimer
impl<const S: bool> AccTimer<S>
where
    TDesc<S>: TArch,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp stop
    /// Record the ticks on stop from a region-to-time, and update the accimulator
    #[inline(always)]
    pub fn stop(&mut self) {
        self.delta = self.base.elapsed_delta();
        self.acc = self.acc.sat_add(self.delta);
    }

    //mp last_delta
    /// Return the last ticks between start and stop
    #[inline(always)]
    pub fn last_delta(&self) -> u64 {
        self.delta.into()
    }

    //mp acc_value
    /// Read the accumulator value
    #[inline(always)]
    pub fn acc_value(&self) -> u64 {
        self.acc.into()
    }
}

//a Trace
//tp Trace
/// A [Trace] can be used to trace the execution of some code, from an
/// start point through a series of intermediate points. The delta for
/// each step can be recorded.
///
/// The 'start' method is called first; at each completed step the
/// 'next' method is called. At the end (after no more than 'N'
/// steps!) the deltas for each step of the trace can be recovered
/// with the 'trace' method.
///
/// A Trace can be generated for any N, for T in u8, u16, u32, u64, u128 and usize
#[derive(Debug)]
pub struct Trace<const S: bool, T: TraceValue, const N: usize>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    index: usize,
    trace: [T; N],
}

//ip Default for Trace
impl<const S: bool, T, const N: usize> std::default::Default for Trace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    [T; N]: Default,
{
    fn default() -> Self {
        let base = BaseTimer::default();
        let index = 0;
        let trace = <[T; N]>::default();
        Self { base, index, trace }
    }
}

//ip Trace
impl<const S: bool, T, const N: usize> Trace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and trace
    pub fn clear(&mut self) {
        unsafe { *self = std::mem::zeroed() };
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    ///
    /// Up to *N* invocations of 'next' afterwards will store
    /// individual deltas in the trace
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
        self.index = 0;
    }

    //mp next
    /// Calculate the delta since the last 'start' or 'next', and
    /// store it in the next trace slot
    ///
    /// If this is invoked more than *N* times after a start then no
    /// work is done, as there is no space to store the time in the
    /// internal trace
    #[inline(always)]
    pub fn next(&mut self) {
        if self.index < N {
            let delta = self.base.elapsed_delta_and_update();
            self.trace[self.index] = delta.into();
            self.index += 1;
        }
    }

    //mp trace
    /// Return the current trace
    pub fn trace(&self) -> &[T; N] {
        &self.trace
    }
}

//a AccVec
//tp AccVec
/// An [AccVec] can be used to accumulate the times taken to execute
/// different branches of code, from a common start point. Each branch
/// is allocated a different index into the AccVec. It can also count
/// the entries.
///
/// The 'start' method is called first; when a branch completed it
/// invokes the 'acc' method with its index, and the delta time since
/// the start is added to that start's accumulator.
///
/// Invoking the 'acc' method does not update the 'start' time, and it
/// is quite sensible to issue multiple 'acc' invocations (with
/// different index values) for a given 'start' invocation.
///
/// An AccVec can be generated for any N, for T in u8, u16, u32, u64, u128 and usize
#[derive(Debug)]
pub struct AccVec<const S: bool, T: TraceValue, C: TraceCount, const N: usize>
where
    TDesc<S>: TArch,
{
    base: BaseTimer<S>,
    accs: [T; N],
    cnts: [C; N],
}

//ip Default for AccVec
impl<const S: bool, T, C, const N: usize> std::default::Default for AccVec<S, T, C, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
    [T; N]: Default,
    [C; N]: Default,
{
    fn default() -> Self {
        let base = BaseTimer::default();
        let accs = <[T; N]>::default();
        let cnts = <[C; N]>::default();
        Self { base, accs, cnts }
    }
}

//ip AccVec
impl<const S: bool, T, C, const N: usize> AccVec<S, T, C, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
    C: TraceCount,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        unsafe { *self = std::mem::zeroed() };
    }

    //mp start
    /// Start the underlying timer
    #[inline(always)]
    pub fn start(&mut self) {
        self.base.start();
    }

    //mp acc_n
    /// Add the ticks on exit to a specific region
    #[inline(always)]
    pub fn acc_n(&mut self, index: usize) {
        if index < N {
            let delta = self.base.elapsed_delta();
            let acc = delta.add(self.accs[index].into());
            self.accs[index] = acc.into();
            self.cnts[index].sat_inc();
        }
    }

    //mp acc_n_restart
    /// Add the ticks on exit to a specific region
    #[inline(always)]
    pub fn acc_n_restart(&mut self, index: usize) {
        if index < N {
            let delta = self.base.elapsed_delta_and_update();
            let acc = delta.add(self.accs[index].into());
            self.accs[index] = acc.into();
            self.cnts[index].sat_inc();
        }
    }

    //mp accs
    /// Return the accumulated values
    pub fn accs(&self) -> &[T; N] {
        &self.accs
    }

    //mp cnts
    /// Return the accumulated counts
    pub fn cnts(&self) -> &[C; N] {
        &self.cnts
    }
}

//a AccTrace
//tp AccTrace
#[derive(Debug)]
pub struct AccTrace<const S: bool, T: TraceValue, const N: usize>
where
    TDesc<S>: TArch,
{
    trace: Trace<S, T, N>,
    acc: [T; N],
}

//ip Default for AccTrace
impl<const S: bool, T, const N: usize> std::default::Default for AccTrace<S, T, N>
where
    TDesc<S>: TArch,
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
impl<const S: bool, T, const N: usize> AccTrace<S, T, N>
where
    TDesc<S>: TArch,
    T: TraceValue,
{
    //mp clear
    /// Clear the timer and accumulated values
    pub fn clear(&mut self) {
        self.trace.clear();
        unsafe { self.acc = std::mem::zeroed() };
    }

    //mp start
    /// Record the ticks on start to a region-to-time
    #[inline(always)]
    pub fn start(&mut self) {
        self.trace.start();
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
            let v = v.add(self.trace.trace[i].into());
            self.acc[i] = v.into();
        }
    }

    //mp last_trace
    /// Return the current trace
    pub fn last_trace(&self) -> &[T; N] {
        self.trace.trace()
    }

    //mp acc_trace
    /// Return the accumulated trace
    pub fn acc_trace(&self) -> &[T; N] {
        &self.acc
    }
}
