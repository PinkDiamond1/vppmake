use crate::bbcode::{write_tag, Detail};
use crate::models::root::Rooted;
use smartstring::alias::String;
use std::fmt::{Display, Formatter, Result};

pub struct Menus<'a>(&'a str);

impl<'a> Menus<'a> {
    pub fn new(slug: &'a str) -> Self {
        Self(slug)
    }
}

impl Display for Rooted<'_, Menus<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(Menus(slug), _) = self;

        write!(f, "{}", Bag::new(slug))?;
        write!(f, "{}", Stats::new(slug))
    }
}

impl Display for Rooted<'_, Detail<Menus<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(Detail(Menus(slug)), root) = self;

        write!(f, "{}", root.with(Detail(Bag::new(slug))))?;
        write!(f, "{}", root.with(Detail(Stats::new(slug))))
    }
}

macro_rules! menu {
    ($vis:vis struct $struct:ident { name = $name:expr, slug = $slug:expr, bottom = $bottom:expr$(,)? }) => {
        $vis struct $struct(String);

        impl $struct {
            const CSS: &'static str = concat!("bottom: ", $bottom, "; position: absolute; left: 1rem; color: white !important; text-shadow: 2px 2px 2px black;");

            $vis fn new(slug: &str) -> Self {
                Self(format!(concat!("{}_", $slug), slug).into())
            }
        }

        impl Display for $struct {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write_tag!(f, "tab", self.0)?;
                write_tag!(f, "div", Self::CSS)?;
                write!(f, concat!("> ", $name))?;
                write_tag!(f, end "div")?;
                write_tag!(f, end "tab")
            }
        }
    };
}

menu! {
    pub struct Bag {
        name = "Bag",
        slug = "bag",
        bottom = "1rem",
    }
}

impl Display for Rooted<'_, Detail<Bag>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(Detail(ref bag), root) = self;

        write_tag!(f, "tabpanel", bag.0)?;
        write!(f, "{}", root.items())?;
        write_tag!(f, end "tabpanel")
    }
}

menu! {
    pub struct Stats {
        name = "Stats",
        slug = "stats",
        bottom = "2.5rem",
    }
}

impl Display for Rooted<'_, Detail<Stats>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Rooted(Detail(ref stats), root) = self;

        write_tag!(f, "tabpanel", stats.0)?;

        writeln!(f, "• {} Pokémon", root.pokemon_count())?;
        writeln!(f, "• {} Items", root.item_count())?;
        writeln!(f, "• {} Points", root.points_count())?;

        write_tag!(f, end "tabpanel")
    }
}
