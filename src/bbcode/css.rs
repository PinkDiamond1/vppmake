use std::collections::HashMap;
use std::fmt;

#[derive(Default, Debug)]
pub struct Css {
    static_rules: Option<&'static str>,
    dynamic_rules: HashMap<&'static str, String>,
}

impl Css {
    #[allow(unused)]
    pub fn new() -> Self {
        Css {
            static_rules: None,
            dynamic_rules: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn with_capacity(cap: usize) -> Self {
        Css {
            static_rules: None,
            dynamic_rules: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn with_static(rules: &'static str) -> Self {
        Css {
            static_rules: Some(rules),
            dynamic_rules: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn with_static_and_capacity(rules: &'static str, cap: usize) -> Self {
        Css {
            static_rules: Some(rules),
            dynamic_rules: HashMap::with_capacity(cap),
        }
    }

    pub fn set(&mut self, property: &'static str, value: impl fmt::Display) {
        self.dynamic_rules.insert(property, value.to_string());
    }
}

impl fmt::Display for Css {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(static_rules) = self.static_rules {
            write!(f, "{}", static_rules)?;
        }

        for (k, v) in self.dynamic_rules.iter() {
            write!(f, "{}: {};", k, v)?;
        }

        Ok(())
    }
}

impl<V: fmt::Display> Extend<(&'static str, V)> for Css {
    fn extend<T: IntoIterator<Item = (&'static str, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.set(k, v);
        }
    }
}
