use crate::refs::as_u8s;

//fi fmt_hex
pub fn fmt_hex<T: Sized + Copy>(obj: &T, ascii: &mut [u8]) {
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

//fi fmt_bin
pub fn fmt_bin<T: Sized + Copy + std::fmt::Debug>(obj: &T, ascii: &mut [u8]) {
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
