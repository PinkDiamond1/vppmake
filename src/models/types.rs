use serde::Deserialize;
use smol_str::SmolStr;
use std::fmt;

use crate::{bbcode, render::Buf};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Type {
    name: SmolStr,
    icon: SmolStr,
    color: Color,
}

pub fn render_type(buf: Buf, ty: &Type) {
    bbcode! {
        in {{ buf }};

        span {{ ty.color }} {
            icon {
                {{ ty.icon }};
            }

            " ";
            {{ ty.name }};
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum Types {
    Single(Type),
    Double(Type, Type),
}

pub fn render_types(buf: Buf, types: &Types) {
    match types {
        Types::Single(ty) => {
            render_type(buf, ty);
        }
        Types::Double(ty1, ty2) => {
            bbcode! {
                in {{ buf }};

                render_type({{ ty1 }});
                " ";
                render_type({{ ty2 }});
            }
        }
    };
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "color: rgb({}, {}, {});", self.0, self.1, self.2)
    }
}
