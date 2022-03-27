use crate::{bbcode::Css, write_tag};

use super::position::Position;
use smartstring::alias::String;
use std::fmt::{Display, Formatter, Result};

const EGG: &str = "https://i.imgur.com/LMyXl3m.png";
const BASE: &str =
    "https://raw.githubusercontent.com/tipsypastels/porygon2-pokemon-data/master/output/sprites";

#[derive(PartialEq, Debug)]
pub(super) struct Sprite<'a> {
    pub url: SpriteUrl<'a>,
    pub flipped: bool,
    pub position: Position,
}

impl Display for Sprite<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut css = Css::with_static_and_capacity("position: absolute;", 2);

        css.positioned(self.position);

        if self.flipped {
            css.set("transform", "scalex(-1)");
        }

        write_tag!(f, "cimg", css)?;
        write!(f, "{}", self.url)?;
        write_tag!(f, end "cimg")
    }
}

#[derive(PartialEq, Debug)]
pub(super) enum SpriteUrl<'a> {
    Egg,
    Pokemon {
        slug: &'a str,
        back: bool,
        shiny: bool,
    },
}

impl SpriteUrl<'_> {
    fn url(&self) -> String {
        match self {
            SpriteUrl::Egg => EGG.into(),
            SpriteUrl::Pokemon { slug, back, shiny } => {
                let dir_1 = if *back { "back" } else { "front" };
                let dir_2 = if *shiny { "-shiny" } else { "" };

                format!("{}/{}{}/{}.png", BASE, dir_1, dir_2, slug).into()
            }
        }
    }
}

impl Display for SpriteUrl<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SpriteUrl::Egg => write!(f, "{}", EGG),
            SpriteUrl::Pokemon { slug, back, shiny } => {
                let dir_1 = if *back { "back" } else { "front" };
                let dir_2 = if *shiny { "-shiny" } else { "" };

                write!(f, "{}/{}{}/{}.png", BASE, dir_1, dir_2, slug)
            }
        }
    }
}
