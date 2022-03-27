use crate::bbcode::write_tag;
use std::fmt::{Display, Formatter, Result};

pub struct Shell<T>(T);

impl<T> Shell<T> {
    const CSS: &'static str = "--w: 608px; --h: 404.44px; font-family: 'press start 2p';";
}

impl<T: Display> Display for Shell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "center")?;
        write_tag!(f, "div", Self::CSS)?;

        write!(f, "{}", self.0)?;

        write_tag!(f, end "div")?;
        write_tag!(f, end "center")?;

        Ok(())
    }
}
