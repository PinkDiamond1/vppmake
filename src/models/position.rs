use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Position {
    TopLeft { top: Px, left: Px },
    TopRight { top: Px, right: Px },
    BottomLeft { bottom: Px, left: Px },
    BottomRight { bottom: Px, right: Px },
}

#[derive(PartialEq, Deserialize, Clone, Copy)]
pub struct Px(u32);

impl Display for Px {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}px", self.0)
    }
}

impl Debug for Px {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}
