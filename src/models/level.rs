use std::fmt::{Debug, Display, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Lv(u32);

impl Lv {
    pub(super) const MIN: Self = Self(0);
    pub(super) const MAX: Self = Self(100);

    pub(super) fn new(lv: u32) -> Self {
        assert!(lv <= Self::MAX.0);

        Self(lv)
    }

    pub(super) fn distribute(ratio: u32) -> Self {
        Self(ratio * Self::MAX.0)
    }

    pub fn is_egg(&self) -> bool {
        *self == Self::MIN
    }

    #[allow(unused)]
    pub fn is_max(&self) -> bool {
        *self == Self::MAX
    }
}

impl Display for Lv {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Lv. {}", self.0)
    }
}

impl Debug for Lv {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}
