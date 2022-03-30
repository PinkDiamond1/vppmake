mod bbcode;
mod css;

pub use bbcode::*;
pub use css::*;

pub type Buf<'t> = &'t mut String;
pub trait Body: Fn(&mut String) {}

impl<F: Fn(&mut String)> Body for F {}
