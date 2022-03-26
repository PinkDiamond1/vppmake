use super::runtime::Runtime;
use crate::bbcode::{write_tag, Css};
use crate::models::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            Shell(Body(|f: &mut Formatter<'_>| {
                write!(
                    f,
                    "{}",
                    TabRow(Body(|f: &mut Formatter<'_>| {
                        for stable in &self.input.stables {
                            write!(f, "{}", AsTab(stable))?;
                        }

                        Ok(())
                    }))
                )?;

                write_tag!(f, "tabcontent")?;

                for stable in &self.input.stables {
                    write!(f, "{}", AsContent(&WithRt::new(stable, self)))?;
                }

                write_tag!(f, end "tabcontent")?;

                Ok(())
            }))
        )
    }
}

/* -------------------------------------------------------------------------- */
/*                             Internal Utilities                             */
/* -------------------------------------------------------------------------- */

struct WithRt<'a, T> {
    inner: &'a T,
    rt: &'a Runtime,
}

impl<'a, T> WithRt<'a, T> {
    fn new(inner: &'a T, rt: &'a Runtime) -> Self {
        Self { inner, rt }
    }
}

struct AsTab<'a, T>(&'a T);
struct AsContent<'a, T>(&'a T);

struct Body<F>(F);

impl<F: Fn(&mut Formatter<'_>) -> Result> Display for Body<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&self.0)(f)
    }
}

/* -------------------------------------------------------------------------- */
/*                             Generic Components                             */
/* -------------------------------------------------------------------------- */

struct Shell<T>(T);

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

struct Block<T>(T);

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

struct Detail<T>(T);

impl<T> Detail<T> {
    const CSS_OUTER: &'static str = "background-color: white; color: black; margin-top: 0.25rem; border: 2px solid black; border-radius: 4px; padding: 2px; width: 608px; max-width: 100%; font-family: 'press start 2p'; text-align: left;";

    const CSS_INNER: &'static str =
        "background-color: white; border: 2px solid black; border-radius: 4px; padding: 1rem;";
}

impl<T: Display> Display for Detail<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "div", Self::CSS_OUTER)?;
        write_tag!(f, "div", Self::CSS_INNER)?;

        write!(f, "{}", self.0)?;

        write_tag!(f, end "div")?;
        write_tag!(f, end "div")?;

        Ok(())
    }
}

struct BackgroundImage<T>(T);

impl<T> BackgroundImage<T> {
    const CSS: &'static str = "width: var(--w); height: var(--h); max-width: unset !important;";
}

impl<T: Display> Display for BackgroundImage<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "cimg", Self::CSS)?;
        write!(f, "{}", self.0)?;
        write_tag!(f, end "cimg")?;

        Ok(())
    }
}

struct Tab<'a> {
    slug: &'a str,
    name: &'a str,
}

impl<'a> Tab<'a> {
    const CSS_OUTER: &'static str = "background-color: white; color: black !important; display: inline-block; font-size: 80%; border: 2px solid black; border-bottom: 0; border-top-left-radius: 4px; border-top-right-radius: 4px; padding: 2px; padding-bottom: 0;";

    const CSS_INNER: &'static str = "border: 2px solid black; border-bottom: 0; border-top-left-radius: 4px; border-top-right-radius: 4px; padding: 6px 9px; padding-bottom: 0;";

    fn new(slug: &'a str, name: &'a str) -> Self {
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

struct TabRow<T>(T);

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

/* -------------------------------------------------------------------------- */
/*                             Concrete Components                            */
/* -------------------------------------------------------------------------- */

impl Display for AsTab<'_, stable::Stable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Tab::new(&self.0.slug, &self.0.name_short))
    }
}

impl AsContent<'_, WithRt<'_, stable::Stable>> {
    const CSS_TITLE: &'static str = "position: absolute; z-index: 999; top: 1rem; left: 1rem; color: white; text-shadow: 2px 2px 2px black; border-bottom: 2px solid white; box-shadow: 0 2px black;";
}

impl Display for AsContent<'_, WithRt<'_, stable::Stable>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tabpanel", self.0.inner.slug)?;

        write!(
            f,
            "{}",
            Block(Body(|f: &mut Formatter<'_>| {
                write!(f, "{}", BackgroundImage(&self.0.inner.image))?;

                write_tag!(f, "div", Self::CSS_TITLE)?;
                write!(f, "{}", self.0.inner.name)?;
                write_tag!(f, end "div")?;

                write!(
                    f,
                    "{}",
                    AsTab(&PokemonsWithBackref(&self.0.inner.pokemon, self.0.inner))
                )?;

                Ok(())
            }))
        )?;

        write!(
            f,
            "{}",
            AsContent(&WithRt::new(
                &PokemonsWithBackref(&self.0.inner.pokemon, self.0.inner),
                self.0.rt
            ))
        )?;

        write_tag!(f, end "tabpanel")?;

        Ok(())
    }
}

struct PokemonsWithBackref<'a>(&'a pokemon::Pokemons, &'a stable::Stable);

impl AsTab<'_, PokemonsWithBackref<'_>> {
    const CSS: &'static str =
        "position: absolute; width: var(--w); height: var(--h); top: 0; left: 0;";
}

impl Display for AsTab<'_, PokemonsWithBackref<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "div", Self::CSS)?;
        write_tag!(f, "tabgroup")?;

        for pokemon in self.0 .0 {
            write!(f, "{}", AsTab(pokemon))?;
        }

        write!(f, "{}", AsTab(&Menus(self.0 .1)))?;

        write_tag!(f, end "tabgroup")?;
        write_tag!(f, end "div")?;

        Ok(())
    }
}

impl Display for AsContent<'_, WithRt<'_, PokemonsWithBackref<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            Detail(Body(|f: &mut Formatter<'_>| {
                write_tag!(f, "tabcontent")?;

                for pokemon in self.0.inner.0 {
                    write!(f, "{}", AsContent(pokemon))?;
                }

                write!(
                    f,
                    "{}",
                    AsContent(&WithRt::new(&Menus(self.0.inner.1), self.0.rt))
                )?;

                write_tag!(f, end "tabcontent")?;

                Ok(())
            }))
        )
    }
}

impl Display for AsTab<'_, pokemon::Pokemon> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tab", self.0.slug)?;
        write!(f, "{}", self.0.sprite())?;
        write_tag!(f, end "tab")?;

        Ok(())
    }
}

impl Display for AsContent<'_, pokemon::Pokemon> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tabpanel", self.0.slug)?;

        let mut shiny_css = Css::new();

        if self.0.shiny {
            shiny_css.set("background-image", "url('/images/sparkle.gif')");
        }

        write_tag!(f, "span", shiny_css)?;
        write!(f, "{}", self.0.name)?;
        write_tag!(f, end "span")?;

        write_tag!(f, "div", "font-size: 80%;")?;

        writeln!(f, "• {}", self.0.species.name)?;
        write!(f, "• {}", self.0.species.types)?;

        write_tag!(f, end "div")?;

        write_tag!(f, end "tabpanel")?;

        Ok(())
    }
}

struct Menus<'a>(&'a stable::Stable);

impl Display for AsTab<'_, Menus<'_>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", AsTab(&Bag::new(&self.0 .0.slug)))?;
        write!(f, "{}", AsTab(&Stats::new(&self.0 .0.slug)))?;

        Ok(())
    }
}

impl Display for AsContent<'_, WithRt<'_, Menus<'_>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            AsContent(&WithRt::new(&Bag::new(&self.0.inner.0.slug), self.0.rt))
        )?;

        write!(
            f,
            "{}",
            AsContent(&WithRt::new(&Stats::new(&self.0.inner.0.slug), self.0.rt))
        )?;

        Ok(())
    }
}

struct Bag {
    slug: String,
}

impl Bag {
    fn new(slug: &str) -> Self {
        Self {
            slug: format!("{}_bag", slug),
        }
    }
}

impl AsTab<'_, Bag> {
    const CSS: &'static str =
        "position: absolute; bottom: 1rem; left: 1rem; color: white !important; text-shadow: 2px 2px 2px black;";
}

impl Display for AsTab<'_, Bag> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tab", self.0.slug)?;
        write_tag!(f, "div", Self::CSS)?;
        write!(f, "> Bag")?;
        write_tag!(f, end "div")?;
        write_tag!(f, end "tab")?;

        Ok(())
    }
}

impl Display for AsContent<'_, WithRt<'_, Bag>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tabpanel", self.0.inner.slug)?;

        for item in &self.0.rt.input.items {
            writeln!(f, "{}", item)?;
        }

        write_tag!(f, end "tabpanel")?;

        Ok(())
    }
}

struct Stats {
    slug: String,
}

impl Stats {
    fn new(slug: &str) -> Self {
        Self {
            slug: format!("{}_stats", slug),
        }
    }
}

impl AsTab<'_, Stats> {
    const CSS: &'static str =
        "position: absolute; bottom: 2.5rem; left: 1rem; color: white !important; text-shadow: 2px 2px 2px black;";
}

impl Display for AsTab<'_, Stats> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tab", self.0.slug)?;
        write_tag!(f, "div", Self::CSS)?;
        write!(f, "> Stats")?;
        write_tag!(f, end "div")?;
        write_tag!(f, end "tab")?;

        Ok(())
    }
}

impl Display for AsContent<'_, WithRt<'_, Stats>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write_tag!(f, "tabpanel", self.0.inner.slug)?;

        writeln!(f, "• {} Pokémon", self.0.rt.analytics.total_pokemon)?;
        writeln!(f, "• {} Items", self.0.rt.input.items.count())?;
        write!(f, "• {} Points", self.0.rt.analytics.current_points)?;

        write_tag!(f, end "tabpanel")?;

        Ok(())
    }
}
