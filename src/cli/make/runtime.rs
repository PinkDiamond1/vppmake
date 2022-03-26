use crate::models::{item::Items, stable::Stables};
use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Input {
    pub spent_points: u32,
    pub stables: Stables,
    pub items: Items,
}

#[derive(PartialEq, Debug)]
pub struct Analytics {
    pub current_points: u32,
    pub total_pokemon: u32,
}

impl Analytics {
    pub fn new(input: &Input) -> Self {
        let current_points = input
            .stables
            .total_points()
            .checked_sub(input.spent_points)
            .expect("Spent points is above budget!");

        let total_pokemon = input.stables.iter().map(|s| s.total_pokemon()).sum();

        Self {
            current_points,
            total_pokemon,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Runtime {
    pub input: Input,
    pub analytics: Analytics,
}

impl Runtime {
    pub fn new(input: Input) -> Self {
        let analytics = Analytics::new(&input);
        Self { input, analytics }
    }
}
