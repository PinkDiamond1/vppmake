use serde::Deserialize;
use smol_str::SmolStr;

use crate::{bbcode, render::Buf};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Item {
    name: SmolStr,
    #[serde(default = "default_quantity")]
    quantity: u32,
}

pub fn render_item(buf: Buf, item: &Item) {
    bbcode! {
        in {{ buf }};

        {{ item.name }};

        if {{ item.quantity > 1 }} {
            {{ format!(" ({}x)", item.quantity) }};
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn total(&self) -> u32 {
        self.0.iter().map(|i| i.quantity).sum()
    }
}

pub fn render_items(buf: Buf, items: &Items) {
    bbcode! {
        in {{ buf }};

        for item in {{ &items.0 }} {
            render_item({{ item }});
            "\n";
        }
    }
}

fn default_quantity() -> u32 {
    1
}
