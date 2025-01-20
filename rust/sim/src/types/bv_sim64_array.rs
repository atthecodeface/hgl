use crate::types::{BvData, BvN, IsBv};

impl BvData for [u64; 1] {
    fn zero(&mut self) {
        self[0] = 0;
    }
    #[track_caller]
    fn as_u8s(&self, n: usize) -> &[u8] {
        assert!(n <= 8, "[u8] for u64 must be no more than 8 bytes");
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, n) }
    }
    #[track_caller]
    fn as_u8s_mut(&mut self, n: usize) -> &mut [u8] {
        assert!(n <= 8, "[u8] for u64 must be no more than 8 bytes");
        unsafe { std::slice::from_raw_parts_mut(self as *mut u64 as *mut u8, n) }
    }
}

impl BvData for [u64; 2] {
    fn zero(&mut self) {
        self[0] = 0;
    }
    #[track_caller]
    fn as_u8s(&self, n: usize) -> &[u8] {
        assert!(n <= 16, "[u8] for u64 must be no more than 16 bytes");
        unsafe { std::slice::from_raw_parts(self as *const u64 as *const u8, n) }
    }
    #[track_caller]
    fn as_u8s_mut(&mut self, n: usize) -> &mut [u8] {
        assert!(n <= 16, "[u8] for u64 must be no more than 16 bytes");
        unsafe { std::slice::from_raw_parts_mut(self as *mut u64 as *mut u8, n) }
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
