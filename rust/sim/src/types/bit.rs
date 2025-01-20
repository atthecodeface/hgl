use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bit(bool);
impl Bit {
    pub const T: Self = Self(true);
    pub const F: Self = Self(false);

    #[inline]
    pub const fn is_true(&self) -> bool {
        self.0
    }

    #[inline]
    pub const fn is_false(&self) -> bool {
        !self.0
    }
}

impl std::ops::Not for Bit {
    type Output = Bit;
    fn not(self) -> Bit {
        Bit(!self.0)
    }
}

macro_rules! bit_op {
    ($t:ty, $tr:ty, $trb:ty, $f:ident, $op:tt) => {

        impl $t for Bit {
            type Output = Bit;
            fn $f(self, other: Bit) -> Bit {
                Bit(self.0 $op other.0)
            }
        }
        impl $tr for Bit {
            type Output = Bit;
            fn $f(self, other: &Bit) -> Bit {
                Bit(self.0 $op other.0)
            }
        }
        impl $trb for Bit {
            type Output = Bit;
            fn $f(self, other: bool) -> Bit {
                Bit(self.0 $op other)
            }
        }

        impl $t for &Bit {
            type Output = Bit;
            fn $f(self, other: Bit) -> Bit {
                Bit(self.0 $op other.0)
            }
        }

        impl $tr for &Bit {
            type Output = Bit;
            fn $f(self, other: &Bit) -> Bit {
                Bit(self.0 $op other.0)
            }
        }

        impl $trb for &Bit {
            type Output = Bit;
            fn $f(self, other: bool) -> Bit {
                Bit(self.0 $op other)
            }
        }
    }
}

macro_rules! bit_op_assign {
    ($t:ty, $tr:ty, $trb:ty, $f:ident, $op:tt) => {

        impl $t for Bit {
            fn $f(&mut self, other: Bit) {
                self.0 = self.0 $op other.0;
            }
        }
        impl $tr for Bit {
            fn $f(&mut self, other: &Bit) {
                self.0 = self.0 $op other.0;
            }
        }
        impl $trb for Bit {
            fn $f(&mut self, other: bool) {
                self.0 = self.0 $op other;
            }
        }
    }
}

bit_op! { std::ops::BitOr<Bit>, std::ops::BitOr<&Bit>, std::ops::BitOr<bool>, bitor, |}
bit_op_assign! { std::ops::BitOrAssign<Bit>, std::ops::BitOrAssign<&Bit>, std::ops::BitOrAssign<bool>, bitor_assign, |}
bit_op! { std::ops::BitAnd<Bit>, std::ops::BitAnd<&Bit>, std::ops::BitAnd<bool>, bitand, &}
bit_op_assign! { std::ops::BitAndAssign<Bit>, std::ops::BitAndAssign<&Bit>, std::ops::BitAndAssign<bool>, bitand_assign, &}
bit_op! { std::ops::BitXor<Bit>, std::ops::BitXor<&Bit>, std::ops::BitXor<bool>, bitxor, ^}
bit_op_assign! { std::ops::BitXorAssign<Bit>, std::ops::BitXorAssign<&Bit>, std::ops::BitXorAssign<bool>, bitxor_assign, ^}

impl std::fmt::Debug for Bit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(fmt)
    }
}

impl std::hash::Hash for Bit {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.hash(h)
    }
}

impl From<bool> for Bit {
    fn from(b: bool) -> Bit {
        Self(b)
    }
}

impl From<Bit> for bool {
    fn from(b: Bit) -> bool {
        b.0
    }
}

impl<'a> From<&'a Bit> for &'a bool {
    fn from(b: &'a Bit) -> &'a bool {
        &b.0
    }
}

impl<T> AsRef<T> for Bit
where
    T: ?Sized,
    <Bit as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<T> AsMut<T> for Bit
where
    <Bit as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl std::ops::Deref for Bit {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for Bit {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}
