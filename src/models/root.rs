use super::{render_stables_contents, render_stables_tabs, Items, Stables};
use crate::bbcode;
use crate::render::{render_shell, Buf};
use serde::Deserialize;

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

    #[allow(unused)]
    pub fn stables(&self) -> &Stables {
        &self.stables
    }

    pub fn items(&self) -> &Items {
        &self.items
    }
}

pub fn render_root(buf: Buf, root: &Root) {
    bbcode! {
        in {{ buf }};

        render_shell() {
            render_stables_tabs({{ &root.stables }});
            render_stables_contents({{ &root.stables }}, {{ root }});
        }
    }
}
