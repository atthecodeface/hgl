use crate::types::{BvData, BvN, IsBv};

//ip BvData for u64
#[inline]
const fn mask_u64(n: usize) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1 << n) - 1
    }
}
impl BvData for u64 {
    fn zero(&mut self) {
        *self = 0;
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
    fn add_msk<const NB: usize>(&mut self, other: &Self) {
        *self = (*self + *other) & mask_u64(NB);
    }
    fn sub_msk<const NB: usize>(&mut self, other: &Self) {
        *self = (*self + *other) & mask_u64(NB);
    }
    fn bit_or<const NB: usize>(&mut self, other: &Self) {
        *self = *self | *other;
    }
    fn bit_and<const NB: usize>(&mut self, other: &Self) {
        *self = *self & *other;
    }
    fn bit_xor<const NB: usize>(&mut self, other: &Self) {
        *self = *self ^ *other;
    }
}

//a Implement IsBv for BvN< 1 .. 64> using u64 as backing store
macro_rules! bv_int_uN {
    ($n:expr, $t:ty) => {
        impl IsBv for BvN<$n> {
            type BackingStore = $t;
            const NB: usize = $n;
            const NU8: usize = ($n + 7) >> 3;
        }
    };
}
bv_int_uN!(1, u64);
bv_int_uN!(2, u64);
bv_int_uN!(3, u64);
bv_int_uN!(4, u64);
bv_int_uN!(5, u64);
bv_int_uN!(6, u64);
bv_int_uN!(7, u64);
bv_int_uN!(8, u64);
bv_int_uN!(9, u64);

bv_int_uN!(10, u64);
bv_int_uN!(11, u64);
bv_int_uN!(12, u64);
bv_int_uN!(13, u64);
bv_int_uN!(14, u64);
bv_int_uN!(15, u64);
bv_int_uN!(16, u64);
bv_int_uN!(17, u64);
bv_int_uN!(18, u64);
bv_int_uN!(19, u64);

bv_int_uN!(20, u64);
bv_int_uN!(21, u64);
bv_int_uN!(22, u64);
bv_int_uN!(23, u64);
bv_int_uN!(24, u64);
bv_int_uN!(25, u64);
bv_int_uN!(26, u64);
bv_int_uN!(27, u64);
bv_int_uN!(28, u64);
bv_int_uN!(29, u64);

bv_int_uN!(30, u64);
bv_int_uN!(31, u64);
bv_int_uN!(32, u64);
bv_int_uN!(33, u64);
bv_int_uN!(34, u64);
bv_int_uN!(35, u64);
bv_int_uN!(36, u64);
bv_int_uN!(37, u64);
bv_int_uN!(38, u64);
bv_int_uN!(39, u64);

bv_int_uN!(40, u64);
bv_int_uN!(41, u64);
bv_int_uN!(42, u64);
bv_int_uN!(43, u64);
bv_int_uN!(44, u64);
bv_int_uN!(45, u64);
bv_int_uN!(46, u64);
bv_int_uN!(47, u64);
bv_int_uN!(48, u64);
bv_int_uN!(49, u64);

bv_int_uN!(50, u64);
bv_int_uN!(51, u64);
bv_int_uN!(52, u64);
bv_int_uN!(53, u64);
bv_int_uN!(54, u64);
bv_int_uN!(55, u64);
bv_int_uN!(56, u64);
bv_int_uN!(57, u64);
bv_int_uN!(58, u64);
bv_int_uN!(59, u64);

bv_int_uN!(60, u64);
bv_int_uN!(61, u64);
bv_int_uN!(62, u64);
bv_int_uN!(63, u64);
bv_int_uN!(64, u64);
