mod bbcode;

pub use bbcode::*;

pub type Buf<'t> = &'t mut String;
pub trait Body: Fn(&mut String) {}

impl<F: Fn(&mut String)> Body for F {}
