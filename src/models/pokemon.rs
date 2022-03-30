use crate::{
    bbcode, css,
    render::{render_sub_block, Buf},
};

use super::{
    render_growth_contents, render_menu_contents, render_menu_tabs, render_sprite, render_types,
    Growth, Level, Position, Root, Sprite, Types,
};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Pokemon {
    // Pokemon spec
    slug: SmolStr,
    name: SmolStr,
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
        let sprite = Sprite::build(self.flipped, self.position);

        if level.is_egg() {
            sprite.egg()
        } else {
            sprite.species(&self.species, self.shiny)
        }
    }

    fn level(&self, post_count: u32) -> Level {
        self.growth.level(post_count)
    }
}

pub fn render_pokemon_tab(buf: Buf, pokemon: &Pokemon, post_count: u32) {
    bbcode! {
        in {{ buf }};

        tab {{ pokemon.slug }} {
            render_sprite({{ &pokemon.sprite(post_count) }});
        }
    }
}

pub fn render_pokemon_contents(buf: Buf, pokemon: &Pokemon, root: &Root) {
    css! {
        shiny_css {
            if {{ pokemon.shiny }} {
                background_image: "url('/images/sparkle.gif')";
            }
        }
    }

    bbcode! {
        in {{ buf }};

        tabpanel {{ pokemon.slug }} {
            span {{ shiny_css }} {
                {{ pokemon.name }};
            }

            div "font-size: 80%;" {
                {{ format!("• {}\n", pokemon.species.name) }};

                render_growth_contents({{ &pokemon.growth }}, {{ root.post_count() }});

                "• ";
                render_types({{ &pokemon.species.types }});
            }
        }
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

pub fn render_pokemons_tabs(buf: Buf, pokemons: &Pokemons, stable_slug: &str, root: &Root) {
    css! {
        div_css {
            position: "absolute";
            width: "var(--w)";
            height: "var(--h)";
            top: "0";
            left: "0";
        }
    }

    bbcode! {
        in {{ buf }};

        div {{ div_css }} {
            tabgroup {
                for pokemon in {{ &pokemons.0 }} {
                    render_pokemon_tab({{ pokemon }}, {{ root.post_count() }});
                }

                render_menu_tabs({{ stable_slug }});
            }
        }
    }
}

pub fn render_pokemons_contents(buf: Buf, pokemons: &Pokemons, stable_slug: &str, root: &Root) {
    bbcode! {
        in {{ buf }};

        render_sub_block() {
            tabcontent {
                for pokemon in {{ &pokemons.0 }} {
                    render_pokemon_contents({{ pokemon }}, {{ root }});
                }

                render_menu_contents({{ stable_slug }}, {{ root }});
            }
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Species {
    slug: SmolStr,
    name: SmolStr,
    types: Types,
}

impl Species {
    pub fn slug(&self) -> &SmolStr {
        &self.slug
    }
}
