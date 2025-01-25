//a Imports
use crate::traits::{BvData, IsBv};
use crate::utils;
use crate::value_types::BvN;

//a BvData for arrays
macro_rules! bv_data_for_std_array {
    ($n:expr, $t:ty, $bpt:expr) => {
        impl BvData for [$t; $n] {
            fn zero<const NB: usize>(&mut self) {
                for i in 0..$n {
                    self[i] = 0;
                }
            }
            fn as_u8s_unbounded(&self) -> &[u8] {
                unsafe { std::slice::from_raw_parts(self as *const $t as *const u8, $bpt * $n) }
            }
            #[track_caller]
            fn as_u8s<const NB: usize>(&self) -> &[u8] {
                unsafe { std::slice::from_raw_parts(self as *const $t as *const u8, (NB + 7) / 8) }
            }
            #[track_caller]
            fn as_u8s_mut<const NB: usize>(&mut self) -> &mut [u8] {
                unsafe { std::slice::from_raw_parts_mut(self as *mut $t as *mut u8, (NB + 7) / 8) }
            }
        }
    };
}

bv_data_for_std_array!(1, u64, 64);
bv_data_for_std_array!(2, u64, 64);
bv_data_for_std_array!(3, u64, 64);
bv_data_for_std_array!(4, u64, 64);
bv_data_for_std_array!(5, u64, 64);

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
    };
}

bv_int_uN!(65);
bv_int_uN!(75);
bv_int_uN!(85);
bv_int_uN!(95);
bv_int_uN!(105);
bv_int_uN!(115);
bv_int_uN!(125);
bv_int_uN!(135);
bv_int_uN!(145);
bv_int_uN!(155);
bv_int_uN!(165);
bv_int_uN!(175);
bv_int_uN!(185);
bv_int_uN!(195);
bv_int_uN!(205);
bv_int_uN!(215);
bv_int_uN!(235);
bv_int_uN!(245);
bv_int_uN!(255);
