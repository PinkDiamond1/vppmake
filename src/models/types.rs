use serde::Deserialize;
use smol_str::SmolStr;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Type {
    name: SmolStr,
    icon: SmolStr,
    color: Color,
}

// pub struct TypeComponent<'p>(&'p Type);

// impl<'p> Component<'p> for TypeComponent<'p> {
//     type Props = Type;
//     type Body = ();

//     fn new(props: &'p Self::Props) -> Self {
//         Self(props)
//     }

//     fn render(&self, buf: &mut String, _: ()) {
//         let ty = &self.0;

//         bbcode!(
//             in {{ buf }}

//             span({{ &ty.color }}) {

//             }
//         )
//     }
// }

#[derive(PartialEq, Debug, Deserialize)]
pub struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "color: rgb({}, {}, {});", self.0, self.1, self.2)
    }
}
