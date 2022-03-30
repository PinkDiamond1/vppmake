// use std::fs;

// mod bbcode;
// mod models;
// mod ui;
mod models;
mod render;

// use models::root::{RawRoot, Root};

// const PATH: &str = "vpp.yaml";

fn main() -> anyhow::Result<()> {
    css! {
        x {
            a: "1";
            b: {{ "5px" }};
        }

        y {

        }
    };

    dbg!((x, y));

    Ok(())
}
