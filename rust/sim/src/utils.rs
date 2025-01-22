//fi mask_u8_zero_none
#[allow(dead_code)]
pub fn mask_u8_zero_none(n: usize) -> u8 {
    if n >= 8 {
        255
    } else {
        (1 << n) - 1
    }
}

//fi mask_u8_zero_all
#[allow(dead_code)]
pub fn mask_u8_zero_all(n: usize) -> u8 {
    if n >= 8 || n == 0 {
        255
    } else {
        (1 << n) - 1
    }
}

//fi mask_u64_zero_none
#[allow(dead_code)]
pub fn mask_u64_zero_none(n: usize) -> u64 {
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
        if i+1 < n {
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
        if i+1 < n {
            (i, u8::MAX)
        } else {
            (i, mask_u8_zero_all(num_bits & 7))
        }
    })
}
