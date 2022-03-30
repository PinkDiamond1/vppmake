use serde::Deserialize;
use smol_str::SmolStr;

use super::{render_pokemons_contents, render_pokemons_tabs, Pokemons, Root};
use crate::render::{render_bg_img, render_block, render_tab, render_tab_row, Buf};
use crate::{bbcode, css};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Stable {
    slug: SmolStr,
    name: SmolStr,
    image: SmolStr,
    name_short: SmolStr,
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

pub fn render_stable_tab(buf: Buf, stable: &Stable) {
    bbcode! {
        in {{ buf }};

        render_tab({{ &stable.slug }}, {{ &stable.name_short }});
    }
}

pub fn render_stable_contents(buf: Buf, stable: &Stable, root: &Root) {
    css! {
        title {
            position: "absolute";
            z_index: "999";
            top: "1rem";
            left: "1rem";

            border_bottom: "2px solid white";
            box_shadow: "0 2px black";

            color: "white";
            text_shadow: "2px 2px 2px black";
        }
    }

    bbcode! {
        in {{ buf }};

        tabpanel {{ &stable.slug }} {
            render_block() {
                render_bg_img({{ &stable.image }});

                div {{ title }} {
                    {{ &stable.name }};
                }

                render_pokemons_tabs({{ &stable.pokemon }}, {{ &stable.slug }}, {{ root }});
            }

            render_pokemons_contents({{ &stable.pokemon }}, {{ &stable.slug }}, {{ root }});
        }
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

pub fn render_stables_tabs(buf: Buf, stables: &Stables) {
    bbcode! {
        in {{ buf }};

        render_tab_row() {
            for stable in {{ &stables.0 }} {
                render_stable_tab({{ stable }});
            }
        }
    }
}

pub fn render_stables_contents(buf: Buf, stables: &Stables, root: &Root) {
    bbcode! {
        in {{ buf }};

        tabcontent {
            for stable in {{ &stables.0 }} {
                render_stable_contents({{ stable }}, {{ root }});
            }
        }
    }
}
