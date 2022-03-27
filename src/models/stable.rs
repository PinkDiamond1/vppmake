use super::{pokemon::Pokemons, root::Rooted};
use crate::{ui::Tab, write_tag};
use serde::Deserialize;
use smartstring::alias::String;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Stable {
    slug: String,
    name: String,
    name_short: String,
    pokemon: Pokemons,
}

impl Stable {
    pub fn total_pokemon(&self) -> u32 {
        self.pokemon.total()
    }

    pub fn total_points(&self) -> u32 {
        self.pokemon.total_points()
    }
}

impl Display for Stable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Tab::new(&self.slug, &self.name_short))
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Stables(Vec<Stable>);

impl Stables {
    pub fn total_pokemon(&self) -> u32 {
        self.0.iter().map(|s| s.total_pokemon()).sum()
    }

    pub fn total_points(&self) -> u32 {
        self.0.iter().map(|s| s.total_points()).sum()
    }
}
