use super::{item::Items, stable::Stables};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize)]
pub struct RawRoot {
    post_count: u32,
    #[serde(default)]
    points_add: u32,
    #[serde(default)]
    points_sub: u32,
    stables: Stables,
    items: Items,
}

#[derive(PartialEq, Debug)]
pub struct Root {
    item_count: u32,
    post_count: u32,
    points_count: u32,
    pokemon_count: u32,
    stables: Stables,
    items: Items,
}

impl From<RawRoot> for Root {
    fn from(root: RawRoot) -> Self {
        let pokemon_count = root.stables.total_pokemon();
        let points_count = root
            .stables
            .total_points()
            .checked_add(root.points_add)
            .and_then(|p| p.checked_sub(root.points_sub))
            .expect("Negative points!");

        Self {
            pokemon_count,
            points_count,
            item_count: root.items.total(),
            post_count: root.post_count,
            stables: root.stables,
            items: root.items,
        }
    }
}

impl Root {
    pub fn item_count(&self) -> u32 {
        self.item_count
    }

    pub fn post_count(&self) -> u32 {
        self.post_count
    }

    pub fn pokemon_count(&self) -> u32 {
        self.pokemon_count
    }

    pub fn points_count(&self) -> u32 {
        self.points_count
    }

    pub fn stables(&self) -> &Stables {
        &self.stables
    }

    pub fn items(&self) -> &Items {
        &self.items
    }

    pub fn with<'rt, T>(&'rt self, value: T) -> Rooted<'rt, T> {
        Rooted(value, self)
    }
}

pub struct Rooted<'rt, T>(pub T, pub &'rt Root);

impl<'rt, T> Rooted<'rt, T> {
    pub fn as_ref(&self) -> Rooted<'rt, &T> {
        Rooted(&self.0, self.1)
    }

    pub fn map<R>(self, f: impl Fn(T) -> R) -> Rooted<'rt, R> {
        Rooted(f(self.0), self.1)
    }
}

// Rooted values that don't *need* to be rooted to render will
// automatically defer to their own implementation. This makes it
// easier to just pass around rooted values everywhere.
impl<T: Display> Display for Rooted<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}
