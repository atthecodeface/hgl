use crate::types::U8Ops;
//tt IsBv
/// Trait that describes the storage for a bit vector of NB bits with
/// a specific backing store
pub trait IsBv {
    /// The storage type used
    type BackingStore: BvData;

    /// Number of bits in the vector
    const NB: usize;

    /// Number of u8 that are *valid* in the backing store
    ///
    /// BvData :: as_u8s will return an &[u8] that may be longer than
    /// is *valid* for the data; this value should be no more than the
    /// length of that [u8], and may well be less (for example, 7 for
    /// a 56-bit vector)
    const NU8: usize;
}

//fi mask_u8
fn mask_u8(n:usize) -> u8 {
    if n>=8 {
        255
    } else {
        (1<<n)-1
    }
}

//tt BvData
pub trait BvData : Sized + Copy + std::fmt::Debug + std::default::Default {
    fn zero(&mut self);
    fn as_u8s(&self, n:usize) -> &[u8];
    fn as_u8s_mut(&mut self, n:usize) -> &mut [u8];
    fn bit_or<const NB:usize>(&mut self, other:&Self)  {
        let mut n = NB;
        let s = self.as_u8s_mut((n+7)/8);
        let o = other.as_u8s((n+7)/8);
        for (sd,od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd | *od) & mask_u8(n);
            n -= 8;
        }
    }
    fn bit_and<const NB:usize>(&mut self, other:&Self)  {
        let mut n = NB;
        let s = self.as_u8s_mut((n+7)/8);
        let o = other.as_u8s((n+7)/8);
        for (sd,od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd & *od) & mask_u8(n);
            n -= 8;
        }
    }
    fn bit_xor<const NB:usize>(&mut self, other:&Self)  {
        let mut n = NB;
        let s = self.as_u8s_mut((n+7)/8);
        let o = other.as_u8s((n+7)/8);
        for (sd,od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd ^ *od) & mask_u8(n);
            n -= 8;
        }
    }
    fn bit_not<const NB:usize>(&mut self)  {
        let mut n = NB;
        let s = self.as_u8s_mut((NB+7)/8);
        for sd in s.iter_mut() {
            *sd = (!*sd) & mask_u8(n);
            n -= 8;
        }
    }
    fn add_msk<const NB:usize>(&mut self, other:&Self) {
        let mut n = NB;
        let s = self.as_u8s_mut((n+7)/8);
        let o = other.as_u8s((n+7)/8);
        let mut c = 0;
        for (sd,od) in s.iter_mut().zip(o.iter()) {
            let v = (*sd) as u16 + (*od) as u16 + c;
            *sd = (v as u8) & mask_u8(n);
            c = if v>=256 {1} else {0};
            n -= 8;
        }
    }
    fn sub_msk<const NB:usize>(&mut self, other:&Self) {
        let mut n = NB;
        let s = self.as_u8s_mut((n+7)/8);
        let o = other.as_u8s((n+7)/8);
        let mut c = 1;
        for (sd,od) in s.iter_mut().zip(o.iter()) {
            let v = (*sd) as u16 + (!*od) as u16 + c;
            *sd = (v as u8) & mask_u8(n);
            c = if v>=256 {1} else {0};
            n -= 8;
        }
    }
    fn bit_shl<const NB:usize>(&mut self, by:usize)  {
        let s = self.as_u8s_mut((NB+7)/8);
        if by < NB {
            for i in 0..NB-by {
                let j = NB-1-i;
                let b = s.bit::<NB>(j-by);
                s.bit_set::<NB>(j,b);
            }
        }
        for i in 0..by {
            s.bit_set::<NB>(i,false);
        }
    }
    fn bit_lshr<const NB:usize>(&mut self, by:usize)  {
        let s = self.as_u8s_mut((NB+7)/8);
        if by < NB {
            for i in 0..NB-by {
                let b = s.bit::<NB>(i+by);
                s.bit_set::<NB>(i,b);
            }
        }
        for i in 0..by {
            s.bit_set::<NB>(NB-1-i,false);
        }
    }
    fn to_bin(&self, n:usize) -> String {
        let mut s = String::with_capacity(n);
        let d = self.as_u8s((n+7)/8);
        for i in 0..n {
            let j = n-1-i;
            let bv = (d[j>>8]>>(j&7))&1;
            s.push((48+bv) as char);
        }
        s
    }
    fn to_hex(&self, n:usize) -> String {
        let nd = (n+3)/4;
        let mut s = String::with_capacity(nd);
        let d = self.as_u8s((n+7)/8);
        for i in 0..nd {
            let j = nd-1-i;
            let nv = if (j&1) != 0 { d[j>>1]>>4 } else {d[j>>1]&0xf };
            s.push(char::from_digit(nv as u32, 16).unwrap());
        }
        s
    }
}

