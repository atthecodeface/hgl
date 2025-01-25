//fi mask_u8_zero_none
#[allow(dead_code)]
pub const fn mask_u8_zero_none(n: usize) -> u8 {
    if n >= 8 {
        255
    } else {
        (1 << n) - 1
    }
}

//fi mask_u8_zero_all
#[allow(dead_code)]
pub const fn mask_u8_zero_all(n: usize) -> u8 {
    if n >= 8 || n == 0 {
        255
    } else {
        (1 << n) - 1
    }
}

//fi mask_u64_zero_none
#[allow(dead_code)]
pub const fn mask_u64_zero_none(n: usize) -> u64 {
    if n >= 64 {
        u64::MAX
    } else {
        (1 << n) - 1
    }
}

//fi mask_u64_zero_all
#[allow(dead_code)]
pub fn mask_u64_zero_all(n: usize) -> u64 {
    if n >= 64 || n == 0 {
        u64::MAX
    } else {
        (1 << n) - 1
    }
}

#[inline]
pub const fn mask_wrapping_u64_zero_none(n: usize) -> std::num::Wrapping<u64> {
    if n >= 64 {
        std::num::Wrapping(u64::MAX)
    } else {
        std::num::Wrapping((1 << n) - 1)
    }
}
//fi num_u8_of_bits
#[allow(dead_code)]
pub const fn num_u8_of_bits(num_bits: usize) -> usize {
    (num_bits + 7) / 8
}

//fi num_u64_of_bits
#[allow(dead_code)]
pub const fn num_u64_of_bits(num_bits: usize) -> usize {
    (num_bits + 63) / 64
}

//fi iter_u64_of_bits
pub fn iter_u64_of_bits(num_bits: usize) -> impl std::iter::Iterator<Item = (usize, u64)> {
    let n = num_u64_of_bits(num_bits);
    (0..n).map(move |i| {
        if i + 1 < n {
            (i, u64::MAX)
        } else {
            (i, mask_u64_zero_all(num_bits & 63))
        }
    })
}

//fi iter_u8_of_bits
pub fn iter_u8_of_bits(num_bits: usize) -> impl std::iter::Iterator<Item = (usize, u8)> {
    let n = num_u8_of_bits(num_bits);
    (0..n).map(move |i| {
        if i + 1 < n {
            (i, u8::MAX)
        } else {
            (i, mask_u8_zero_all(num_bits & 7))
        }
    })
}
//fi gcd
#[allow(dead_code)]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }
    loop {
        let r = a % b;
        if r > 0 {
            a = b;
            b = r;
        } else {
            break;
        }
    }
    b
}

//fi lcm
#[allow(dead_code)]
pub fn lcm(a: usize, b: usize) -> usize {
    let b = b / gcd(a, b);
    a * b
}

//fi as_u8s
pub unsafe fn as_u8s<T: Sized>(obj: &T) -> &[u8] {
    std::slice::from_raw_parts((obj as *const T).cast::<u8>(), size_of::<T>())
}

//fi as_u8s_mut
pub unsafe fn as_u8s_mut<T: Sized>(obj: &mut T) -> &mut [u8] {
    std::slice::from_raw_parts_mut((obj as *mut T).cast::<u8>(), size_of::<T>())
}

//fi
pub fn fmt_hex<T: Sized>(obj: &T, ascii: &mut [u8]) {
    let data = unsafe { as_u8s(obj) };
    let dn = data.len() * 2;
    let n = ascii.len();
    for i in (0..n).rev() {
        if i >= dn {
            continue;
        }
        let mut d = data[i / 2];
        if i & 1 != 0 {
            d >>= 4;
        } else {
            d &= 0xf;
        }
        ascii[n - 1 - i] = {
            if d > 9 {
                d + b'a' - 10
            } else {
                d + b'0'
            }
        };
    }
}

pub fn fmt_bin<T: Sized + std::fmt::Debug>(obj: &T, ascii: &mut [u8]) {
    let data = unsafe { as_u8s(obj) };
    let dn = data.len() * 8;
    let n = ascii.len();
    for i in (0..n).rev() {
        if i >= dn {
            continue;
        }
        let mut d = data[i / 8];
        d >>= i & 7;
        ascii[n - 1 - i] = b'0' + (d & 1);
    }
}
