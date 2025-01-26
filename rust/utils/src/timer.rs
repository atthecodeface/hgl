#[derive(Default, Debug)]
pub struct Timer {
    entry_clks: std::num::Wrapping<u64>,
    accum_clks: std::num::Wrapping<u64>,
    last_accum_clks: std::num::Wrapping<u64>,
}

pub const TICKS_PER_US_APPLE_M4: u64 = 1_000_000_000;

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn get_timer() -> std::num::Wrapping<u64> {
    use std::arch::asm;
    let timer: u64;
    unsafe {
        asm!(
            "isb
            mrs {timer}, cntvct_el0",
            timer = out(reg) timer,
        );
    }
    std::num::Wrapping(timer)
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn get_timer() -> std::num::Wrapping<u64> {
    use std::arch::asm;
    let lo: u32;
    let hi: u32;
    unsafe {
        asm!(
            "rdtsc",
            lo = out(a) lo,
            hi = out(d) hi,
        );
    }
    std::num::Wrapping(((hi as u64) << 32) | (lo as u64))
}

impl Timer {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    #[inline(always)]
    pub fn entry(&mut self) {
        self.entry_clks = get_timer();
    }
    #[inline(always)]
    pub fn exit(&mut self) {
        let now = get_timer();
        self.accum_clks += now - self.entry_clks;
    }
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.accum_clks.0
    }
    pub fn delta_value(&mut self) -> u64 {
        let r = self.accum_clks - self.last_accum_clks;
        self.last_accum_clks = self.accum_clks;
        r.0
    }
}
