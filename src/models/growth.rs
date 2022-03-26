use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum Growth {
    Manual(GrowthStage),
    Bounded(GrowthBounds),
}

impl Growth {
    pub fn level(&self, post_count: u32) -> Lv {
        match self {
            Growth::Manual(ref m) => m.level(),
            Growth::Bounded(ref b) => b.level(post_count),
        }
    }
}

pub struct Lv(u32);

impl Lv {
    const MIN: u32 = 0;
    const MAX: u32 = 100;

    fn new(lv: u32) -> Self {
        assert!(lv <= Self::MAX);

        Self(lv)
    }

    fn min() -> Self {
        Self(Self::MIN)
    }

    fn max() -> Self {
        Self(Self::MAX)
    }

    pub fn is_egg(&self) -> bool {
        self.0 == Self::MIN
    }

    #[allow(unused)]
    pub fn is_max(&self) -> bool {
        self.0 == Self::MAX
    }
}

impl fmt::Display for Lv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lv. {}", self.0)
    }
}

impl fmt::Debug for Lv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct GrowthBounds {
    start: u32,
    hatch: u32,
    grown: u32,
}

impl GrowthBounds {
    fn level(&self, post_count: u32) -> Lv {
        assert!(post_count >= self.start);

        if post_count > self.grown {
            return Lv::max();
        }

        // NOTE: we're lying here - unhatched pokemon are always treated
        // as having level zero, even though the math below would give them
        // a low but nonzero level.
        if post_count < self.hatch {
            return Lv::min();
        }

        let distance = self.grown - self.start;
        let progress = post_count - self.start;

        Lv::new((progress / distance) * Lv::MAX)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GrowthStage {
    Egg,
    Growing { level: u32 },
    Grown,
}

impl GrowthStage {
    fn level(&self) -> Lv {
        match *self {
            Self::Egg => Lv::min(),
            Self::Growing { level } => Lv::new(level),
            Self::Grown => Lv::max(),
        }
    }
}
