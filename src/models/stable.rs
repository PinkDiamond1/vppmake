use super::pokemon::Pokemons;
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Stable {
    pub slug: String,
    pub name: String,
    pub name_short: String,
    pub pokemon: Pokemons,
    pub image: String,
}

impl Stable {
    pub fn total_points(&self) -> u32 {
        self.pokemon.total_points()
    }

    pub fn total_pokemon(&self) -> u32 {
        self.pokemon.count()
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Stables(Vec<Stable>);

impl Stables {
    pub fn total_points(&self) -> u32 {
        self.0.iter().map(|p| p.total_points()).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Stable> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a Stables {
    type Item = &'a Stable;
    type IntoIter = std::slice::Iter<'a, Stable>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
