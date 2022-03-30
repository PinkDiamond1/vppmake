use crate::{bbcode, css, render::Buf};

use super::{Position, Species};
use std::fmt::{Display, Formatter, Result};

const EGG: &str = "https://i.imgur.com/LMyXl3m.png";
const BASE: &str =
    "https://raw.githubusercontent.com/tipsypastels/porygon2-pokemon-data/master/output/sprites";

pub(super) struct Sprite<'a> {
    url: SpriteUrl<'a>,
    flipped: bool,
    position: Position,
}

impl Sprite<'_> {
    pub fn build(flipped: bool, position: Position) -> SpriteBuilder {
        SpriteBuilder { flipped, position }
    }
}

pub(super) struct SpriteBuilder {
    flipped: bool,
    position: Position,
}

impl SpriteBuilder {
    pub fn egg(self) -> Sprite<'static> {
        Sprite {
            flipped: self.flipped,
            position: self.position,
            url: SpriteUrl::Egg,
        }
    }

    pub fn species(self, species: &Species, shiny: bool) -> Sprite {
        Sprite {
            flipped: self.flipped,
            position: self.position,
            url: SpriteUrl::Species {
                slug: species.slug(),
                back: false,
                shiny,
            },
        }
    }
}

pub(super) fn render_sprite(buf: Buf, sprite: &Sprite) {
    css! {
        sprite_css {
            position: "absolute";
            {{ sprite.position }};

            if {{ sprite.flipped }} {
                transform: "scalex(-1)";
            }
        }
    }

    bbcode! {
        in {{ buf }};

        cimg {{ sprite_css }} {
            {{ sprite.url }};
        }
    }
}

pub enum SpriteUrl<'a> {
    Egg,
    Species {
        slug: &'a str,
        back: bool,
        shiny: bool,
    },
}

impl Display for SpriteUrl<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Egg => write!(f, "{}", EGG),
            Self::Species { slug, back, shiny } => {
                let dir_1 = if *back { "back" } else { "front" };
                let dir_2 = if *shiny { "-shiny" } else { "" };

                write!(f, "{}/{}{}/{}.png", BASE, dir_1, dir_2, slug)
            }
        }
    }
}
