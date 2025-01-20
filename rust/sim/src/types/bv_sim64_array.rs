use crate::types::{BvData, BvN, IsBv};

impl BvData for [u64; 1] {
    fn zero<const NB:usize>(&mut self) {
        self[0] = 0;
    }
    fn as_u8s_unbounded(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, 8) }
    }
    #[track_caller]
    fn as_u8s<const NB:usize>(&self) -> &[u8] {
        assert!(NB <= 64, "[u8] for u64 must be no more than 8 bytes");
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, (NB+7)/8) }
    }
    #[track_caller]
    fn as_u8s_mut<const NB:usize>(&mut self) -> &mut [u8] {
        assert!(NB <= 64, "[u8] for u64 must be no more than 8 bytes");
        unsafe { std::slice::from_raw_parts_mut(self as *mut u64 as *mut u8, (NB+7)/8) }
    }
}

impl BvData for [u64; 2] {
    fn zero<const NB:usize>(&mut self) {
        self[0] = 0;
        self[1] = 0;
    }
    fn as_u8s_unbounded(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, 16) }
    }
    #[track_caller]
    fn as_u8s<const NB:usize>(&self) -> &[u8] {
        assert!(NB <= 128, "[u8] for u64 must be no more than 16 bytes");
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, (NB+7)/8) }
    }
    #[track_caller]
    fn as_u8s_mut<const NB:usize>(&mut self) -> &mut [u8] {
        assert!(NB <= 128, "[u8] for u64 must be no more than 16 bytes");
        unsafe { std::slice::from_raw_parts_mut(self as *mut u64 as *mut u8, (NB+7)/8) }
    }
}

macro_rules! bv_int_uN {
    ($n:expr, $m:expr) => {
        impl IsBv for BvN<{ $n + $m }> {
            type BackingStore = [u64; ($n + $m + 63) >> 6];
            const NB: usize = $n + $m;
            const NU8: usize = ($n + $m + 7) >> 3;
        }
    };
    ($n:expr) => {
        bv_int_uN!($n, 0);
        bv_int_uN!($n, 1);
        bv_int_uN!($n, 2);
        bv_int_uN!($n, 3);
        bv_int_uN!($n, 4);
        bv_int_uN!($n, 5);
        bv_int_uN!($n, 6);
        bv_int_uN!($n, 7);
        bv_int_uN!($n, 8);
        bv_int_uN!($n, 9);
        bv_int_uN!($n, 10);
        bv_int_uN!($n, 11);
        bv_int_uN!($n, 12);
        bv_int_uN!($n, 13);
        bv_int_uN!($n, 14);
        bv_int_uN!($n, 15);
    };
}

bv_int_uN!(65);
bv_int_uN!(81);
bv_int_uN!(97);
bv_int_uN!(113);
