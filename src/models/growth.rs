use super::Level;
use crate::render::Buf;
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Growth {
    Bounds(GrowthBounds),
    Stage(GrowthStage),
}

impl Growth {
    pub fn level(&self, post_count: u32) -> Level {
        match self {
            Self::Bounds(b) => b.level(post_count),
            Self::Stage(s) => s.level(),
        }
    }
}

pub fn render_growth_contents(buf: Buf, growth: &Growth, post_count: u32) {
    use std::fmt::Write;

    let level = growth.level(post_count);

    writeln!(buf, "• {}", level).unwrap();

    if level.is_egg() {
        if let Growth::Bounds(b) = growth {
            writeln!(buf, "• Hatch @ {}", b.hatch).unwrap();
        }
    } else if !level.is_max() {
        if let Growth::Bounds(b) = growth {
            writeln!(buf, "• Grown @ {}", b.grown).unwrap();
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
pub struct GrowthBounds {
    start: u32,
    hatch: u32,
    grown: u32,
}

impl GrowthBounds {
    fn level(&self, post_count: u32) -> Level {
        assert!(post_count >= self.start);

        if post_count > self.grown {
            return Level::MAX;
        }

        if post_count < self.hatch {
            return Level::MIN;
        }

        let distance = (self.grown - self.hatch) as f32;
        let progress = (post_count - self.hatch) as f32;

        Level::distribute(progress / distance)
    }
}

#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum GrowthStage {
    Egg,
    Growing { level: u32 },
    Grown,
}

impl GrowthStage {
    fn level(&self) -> Level {
        match *self {
            Self::Egg => Level::MIN,
            Self::Growing { level } => Level::new(level),
            Self::Grown => Level::MAX,
        }
    }
}
