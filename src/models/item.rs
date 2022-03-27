use serde::Deserialize;
use smartstring::alias::String;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Deserialize)]
pub struct Item {
    name: String,
    #[serde(default = "default_quantity")]
    quantity: u32,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)?;

        if self.quantity > 1 {
            write!(f, "({}x)", self.quantity)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn total(&self) -> u32 {
        self.0.iter().map(|i| i.quantity).sum()
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for item in &self.0 {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

fn default_quantity() -> u32 {
    1
}
