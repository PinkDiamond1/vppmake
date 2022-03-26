use super::r#type::Types;
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Species {
    pub slug: String,
    pub name: String,
    pub types: Types,
}

const SPRITE_URL_BASE: &str =
    "https://raw.githubusercontent.com/tipsypastels/porygon2-pokemon-data/master/output/sprites";

impl Species {
    pub(super) fn sprite_url(&self, sprite: SpeciesSpriteKind, egg: bool) -> String {
        let slug = if egg { "egg".to_string() } else { self.slug };
        format!("{}/{}/{}.png", SPRITE_URL_BASE, sprite.dir(), slug)
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum SpeciesSpriteKind {
    Front,
    Back,
    FrontShiny,
    BackShiny,
}

impl SpeciesSpriteKind {
    fn dir(&self) -> &'static str {
        match self {
            Self::Front => "front",
            Self::Back => "back",
            Self::FrontShiny => "front-shiny",
            Self::BackShiny => "back-shiny",
        }
    }
}
