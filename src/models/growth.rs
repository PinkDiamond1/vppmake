use super::{level::Lv, root::Rooted};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum Growth {
    Bounds(GrowthBounds),
    Stage(GrowthStage),
}

impl Growth {
    pub fn level(&self, post_count: u32) -> Lv {
        match self {
            Growth::Bounds(ref b) => b.level(post_count),
            Growth::Stage(ref s) => s.level(),
        }
    }
}

impl Display for Rooted<'_, Growth> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(ref growth, root) = self;
        write!(f, "{}", growth.level(root.post_count()))
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
            return Lv::MAX;
        }

        // We're lying here - unhatched Pokemon are always treated
        // as having level zero, even though the math would actually
        // give them a low but non-zero level.
        if post_count < self.hatch {
            return Lv::MIN;
        }

        let distance = self.grown - self.start;
        let progress = post_count - self.start;

        Lv::distribute(progress / distance)
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
            Self::Egg => Lv::MIN,
            Self::Growing { level } => Lv::new(level),
            Self::Grown => Lv::MAX,
        }
    }
}
