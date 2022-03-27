use crate::bbcode::write_tag;
use std::fmt::{Display, Formatter, Result};

pub struct Tab<'a> {
    slug: &'a str,
    name: &'a str,
}

impl<'a> Tab<'a> {
    const CSS_OUTER: &'static str = "background-color: white; color: black !important; display: inline-block; font-size: 80%; border: 2px solid black; border-bottom: 0; border-top-left-radius: 4px; border-top-right-radius: 4px; padding: 2px; padding-bottom: 0;";

    const CSS_INNER: &'static str = "border: 2px solid black; border-bottom: 0; border-top-left-radius: 4px; border-top-right-radius: 4px; padding: 6px 9px; padding-bottom: 0;";

    pub fn new(slug: &'a str, name: &'a str) -> Self {
        Self { slug, name }
    }
}

impl Display for Tab<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tab", self.slug)?;
        write_tag!(f, "div", Self::CSS_OUTER)?;
        write_tag!(f, "div", Self::CSS_INNER)?;

        write!(f, "{}", self.name)?;

        write_tag!(f, end "div")?;
        write_tag!(f, end "div")?;
        write_tag!(f, end "tab")?;

        Ok(())
    }
}

pub struct TabRow<T>(T);

impl<T> TabRow<T> {
    const CSS: &'static str = "text-align: left; width: var(--w); max-width: 100%;";
}

impl<T: Display> Display for TabRow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "div", Self::CSS)?;
        write_tag!(f, "tabgroup")?;

        write!(f, "{}", self.0)?;

        write_tag!(f, end "tabgroup")?;
        write_tag!(f, end "div")?;

        Ok(())
    }
}
