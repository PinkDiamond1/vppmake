use crate::bbcode::write_tag;
use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Type {
    name: String,
    color: Color,
    icon: String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tag!(f, "span", self.color)?;

        write_tag!(f, "icon")?;
        write!(f, "{}", self.icon)?;
        write_tag!(f, end "icon")?;

        write!(f, " {}", self.name)?;

        write_tag!(f, end "span")?;

        Ok(())
    }
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(untagged)]
pub enum Types {
    Single(Type),
    Double(Type, Type),
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Types::Single(ty) => write!(f, "{}", ty),
            Types::Double(ty1, ty2) => write!(f, "{} {}", ty1, ty2),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
struct Color(u32, u32, u32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "color: rgb({}, {}, {});", self.0, self.1, self.2)
    }
}
