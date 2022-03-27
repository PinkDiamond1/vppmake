use std::fmt::{Display, Formatter, Result};

pub struct Body<F>(F);

impl<F: Fn(&mut Formatter<'_>) -> Result> Display for Body<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (&self.0)(f)
    }
}
