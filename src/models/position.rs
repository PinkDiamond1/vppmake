use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Position {
    TopLeft { top: Px, left: Px },
    TopRight { top: Px, right: Px },
    BottomLeft { bottom: Px, left: Px },
    BottomRight { bottom: Px, right: Px },
}

use Position::*;
type Pair = (&'static str, Px);

pub struct Iter(Option<Pair>, Option<Pair>);

impl Iterator for Iter {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().or_else(|| self.1.take())
    }
}

impl IntoIterator for Position {
    type Item = Pair;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let pairs: (Pair, Pair) = match self {
            TopLeft { top, left } => (("top", top), ("left", left)),
            TopRight { top, right } => (("top", top), ("right", right)),
            BottomLeft { bottom, left } => (("bottom", bottom), ("left", left)),
            BottomRight { bottom, right } => (("bottom", bottom), ("right", right)),
        };

        Iter(Some(pairs.0), Some(pairs.1))
    }
}

#[derive(PartialEq, Deserialize, Clone, Copy)]
pub struct Px(u32);

impl fmt::Display for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

impl fmt::Debug for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
