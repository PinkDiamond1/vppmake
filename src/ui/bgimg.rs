use crate::bbcode::write_tag;
use std::fmt::{Display, Formatter, Result};

pub struct BgImg<T>(T);

impl<T> BgImg<T> {
    const CSS: &'static str = "width: var(--w); height: var(--h); max-width: unset !important;";
}

impl<T: Display> Display for BgImg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "cimg", Self::CSS)?;
        write!(f, "{}", self.0)?;
        write_tag!(f, end "cimg")?;

        Ok(())
    }
}
