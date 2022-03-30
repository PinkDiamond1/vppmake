use crate::{bbcode, css, render::Buf};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Ribbon {
    image: SmolStr,
}

pub fn render_ribbon(buf: Buf, ribbon: &Ribbon) {
    css! {
        cimg_css {
            width: "100px";
            height: "100px";
        }
    }

    bbcode! {
        in {{ buf }};

        cimg {{ cimg_css }} {
            {{ ribbon.image }};
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Ribbons(Vec<Ribbon>);

impl Ribbons {
    pub fn total(&self) -> u32 {
        self.0.len() as u32
    }
}

pub fn render_ribbons(buf: Buf, ribbons: &Ribbons) {
    css! {
        grid {
            display: "flex";
            flex_wrap: "wrap";
        }
    }

    bbcode! {
        in {{ buf }};

        div {{ grid }} {
            for ribbon in {{ &ribbons.0 }} {
                render_ribbon({{ ribbon }});
            }
        }
    }
}
