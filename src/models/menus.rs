use super::{render_items, render_ribbons, Root};
use crate::render::Buf;
use crate::{bbcode, css};

pub fn render_menu_tabs(buf: Buf, stable_slug: &str) {
    bbcode! {
        in {{ buf }};

        render_bag_tab({{ stable_slug }});
        render_stats_tab({{ stable_slug }});
        render_ribbons_tab({{ stable_slug }});
    }
}

macro_rules! render_menu_tab {
    ($fn:ident, $slug:literal, $name:literal, bottom = $bottom:literal) => {
        fn $fn(buf: Buf, stable_slug: &str) {
            let slug = format!(concat!("{}_", $slug), stable_slug);
            let name = concat!("> ", $name);

            css! {
                css {
                    position: "absolute";
                    bottom: $bottom;
                    left: "1rem";

                    color: "white !important";
                    text_shadow: "2px 2px 2px black";
                }
            }

            bbcode! {
                in {{ buf }};

                tab {{ slug }} {
                    div {{ css }} {
                        {{ name }};
                    }
                }
            }
        }
    };
}

render_menu_tab!(render_bag_tab, "bag", "Bag", bottom = "4rem");
render_menu_tab!(render_stats_tab, "stats", "Stats", bottom = "2.5rem");
render_menu_tab!(render_ribbons_tab, "ribbons", "Ribbons", bottom = "1rem");

pub fn render_menu_contents(buf: Buf, stable_slug: &str, root: &Root) {
    bbcode! {
        in {{ buf }};

        render_bag_contents({{ stable_slug }}, {{ root }});
        render_stats_contents({{ stable_slug }}, {{ root }});
        render_ribbons_contents({{ stable_slug }}, {{ root }});
    }
}

fn render_bag_contents(buf: Buf, stable_slug: &str, root: &Root) {
    let slug = format!("{}_bag", stable_slug);

    bbcode! {
        in {{ buf }};

        tabpanel {{ slug }} {
            render_items({{ root.items() }});
        }
    }
}

fn render_stats_contents(buf: Buf, stable_slug: &str, root: &Root) {
    let slug = format!("{}_stats", stable_slug);

    bbcode! {
        in {{ buf }};

        tabpanel {{ slug }} {
            {{ format!("• {} Pokémon\n", root.pokemon_count()) }};
            {{ format!("• {} Items\n", root.item_count() )}};
            {{ format!("• {} Points\n", root.points_count()) }};
            {{ format!("• {} Ribbons\n", root.ribbon_count()) }};
        }
    }
}

fn render_ribbons_contents(buf: Buf, stable_slug: &str, root: &Root) {
    let slug = format!("{}_ribbons", stable_slug);

    bbcode! {
        in {{ buf }};

        tabpanel {{ slug }} {
            render_ribbons({{ root.ribbons() }});
        }
    }
}
