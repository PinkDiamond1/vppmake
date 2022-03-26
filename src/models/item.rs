use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Item {
    name: String,
    #[serde(default = "default_quantity")]
    quantity: u32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        if self.quantity > 1 {
            write!(f, " ({}x)", self.quantity)?;
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn count(&self) -> u32 {
        self.0.iter().map(|i| i.quantity).sum()
    }
}

impl<'a> IntoIterator for &'a Items {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

fn default_quantity() -> u32 {
    1
}
