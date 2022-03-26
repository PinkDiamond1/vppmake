use argh::FromArgs;
use std::fs;

mod render;
mod runtime;

use runtime::*;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
#[argh(name = "make")]
#[argh(description = "Builds the CSS for Dakota's VPP thread.")]
pub struct Opts {}

const PATH: &str = "vpp.yaml";

pub fn run(_: Opts) -> anyhow::Result<()> {
    let contents = fs::read_to_string(PATH)?;
    let input: Input = serde_yaml::from_str(&contents)?;
    let runtime = Runtime::new(input);

    println!("{}", runtime);

    Ok(())
}
