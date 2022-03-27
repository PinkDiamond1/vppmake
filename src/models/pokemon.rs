use crate::{models::menu::Menus, write_tag};

use super::{
    growth::Growth,
    level::Lv,
    position::Position,
    root::Rooted,
    species::Species,
    sprite::{Sprite, SpriteUrl},
};
use serde::Deserialize;
use smartstring::alias::String;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Pokemon {
    // Pokemon spec
    slug: String,
    name: String,
    species: Species,
    #[serde(default)]
    shiny: bool,

    // VPP spec
    points: u32,
    growth: Growth,

    // Layout spec
    position: Position,
    #[serde(default)]
    flipped: bool,
}

impl Pokemon {
    fn sprite(&self, post_count: u32) -> Sprite {
        let level = self.level(post_count);

        let flipped = self.flipped;
        let position = self.position;
        let shiny = self.shiny;
        let slug = &self.slug;

        if level.is_egg() {
            Sprite {
                flipped,
                position,
                url: SpriteUrl::Egg,
            }
        } else {
            Sprite {
                flipped,
                position,
                url: SpriteUrl::Pokemon {
                    slug,
                    shiny,
                    back: false,
                },
            }
        }
    }

    fn level(&self, post_count: u32) -> Lv {
        self.growth.level(post_count)
    }
}

impl Display for Rooted<'_, &'_ Pokemon> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(pokemon, root) = self;

        write_tag!(f, "tab", pokemon.slug)?;
        write!(f, "{}", pokemon.sprite(root.post_count()))?;
        write_tag!(f, end "tab")
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Pokemons(Vec<Pokemon>);

impl Pokemons {
    pub fn total(&self) -> u32 {
        self.0.len() as _
    }

    pub fn total_points(&self) -> u32 {
        self.0.iter().map(|p| p.points).sum()
    }
}

impl Rooted<'_, &'_ Pokemons> {
    const CSS: &'static str =
        "position: absolute; width: var(--w); height: var(--h); top: 0; left: 0;";
}

impl Display for Rooted<'_, &'_ Pokemons> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(pokemons, root) = self;

        write_tag!(f, "div", Self::CSS)?;
        write_tag!(f, "tabgroup")?;

        for pokemon in &pokemons.0 {
            write!(f, "{}", root.with(pokemon))?;
        }

        write!(f, "{}", root.with(Menus::new()))?;

        todo!()
    }
}
