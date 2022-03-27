use crate::bbcode::write_tag;
use std::fmt::{Display, Formatter, Result};

pub struct Block<T>(T);

impl<T> Block<T> {
    const CSS_OUTER: &'static str = "background-color: white; color: black; display: inline-block; border: 2px solid black; border-radius: 4px; padding: 2px; max-width: 100%;";

    const CSS_INNER: &'static str = "display: inline-block; border: 2px solid black; border-radius: 4px; position: relative; display: flex; align-items: flex-end; overflow-x: scroll; max-width: 100%;";
}

impl<T: Display> Display for Block<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "div", Self::CSS_OUTER)?;
        write_tag!(f, "div", Self::CSS_INNER)?;

        write!(f, "{}", self.0)?;

        write_tag!(f, end "div")?;
        write_tag!(f, end "div")?;

        Ok(())
    }
}

pub struct SubBlock<T>(T);

impl<T> SubBlock<T> {
    const CSS_OUTER: &'static str = "background-color: white; color: black; margin-top: 0.25rem; border: 2px solid black; border-radius: 4px; padding: 2px; width: 608px; max-width: 100%; font-family: 'press start 2p'; text-align: left;";

    const CSS_INNER: &'static str =
        "background-color: white; border: 2px solid black; border-radius: 4px; padding: 1rem;";
}

impl<T: Display> Display for SubBlock<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "div", Self::CSS_OUTER)?;
        write_tag!(f, "div", Self::CSS_INNER)?;

        write!(f, "{}", self.0)?;

        write_tag!(f, end "div")?;
        write_tag!(f, end "div")?;

        Ok(())
    }
}
