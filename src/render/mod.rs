mod bbcode;
mod css;
mod layout;

pub use bbcode::*;
pub use css::*;
pub use layout::*;

pub type Buf<'t> = &'t mut String;
pub trait Body: Fn(&mut String) {}

impl<F: Fn(&mut String)> Body for F {}
