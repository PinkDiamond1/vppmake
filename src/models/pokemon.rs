use super::{
    growth::{Growth, Lv},
    position::Position,
    species::{Species, SpeciesSpriteKind},
};
use crate::bbcode::{write_tag, Css};
use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Pokemon {
    // Pokemon spec
    pub slug: String,
    pub name: String,
    pub species: Species,
    #[serde(default)]
    pub shiny: bool,

    // VPP spec
    pub points: u32,
    pub growth: Growth,

    // Layout spec
    pub position: Position,
    #[serde(default)]
    flipped: bool,
}

impl Pokemon {
    pub fn sprite(&self) -> PokemonSprite {
        PokemonSprite(self)
    }

    pub fn sprite_url(&self, post_count: u32) -> String {
        let kind = self.sprite_kind();
        let egg = self.growth_level(post_count).is_egg();

        self.species.sprite_url(kind, egg)
    }

    pub fn growth_level(&self, post_count: u32) -> Lv {
        self.growth.level(post_count)
    }

    fn sprite_kind(&self) -> SpeciesSpriteKind {
        if self.shiny {
            SpeciesSpriteKind::FrontShiny
        } else {
            SpeciesSpriteKind::Front
        }
    }
}

pub struct PokemonSprite<'a>(&'a Pokemon);

impl fmt::Display for PokemonSprite<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut css = Css::with_static_and_capacity("position: absolute;", 2);

        css.extend(self.0.position);

        if self.0.flipped {
            css.set("transform", "scalex(-1)");
        }

        write_tag!(f, "cimg", css)?;
        write!(f, "{}", self.0.sprite_url())?;
        write_tag!(f, end "cimg")?;

        Ok(())
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Pokemons(pub(super) Vec<Pokemon>);

impl Pokemons {
    pub fn total_points(&self) -> u32 {
        self.0.iter().map(|p| p.points).sum()
    }

    pub fn count(&self) -> u32 {
        self.0.len() as _
    }
}

impl<'a> IntoIterator for &'a Pokemons {
    type Item = &'a Pokemon;
    type IntoIter = std::slice::Iter<'a, Pokemon>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
