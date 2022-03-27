use std::fs;

mod bbcode;
mod models;
mod ui;

use models::root::{RawRoot, Root};

const PATH: &str = "vpp.yaml";

fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string(PATH)?;

    let root: RawRoot = serde_yaml::from_str(&contents)?;
    let root: Root = root.into();

    dbg!(root);

    Ok(())
}
