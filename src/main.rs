use models::{render_root, RawRoot, Root};
use std::fs;

mod models;
mod render;

const PATH: &str = "vpp.yaml";

fn main() {
    let contents = fs::read_to_string(PATH).unwrap();

    let root: RawRoot = serde_yaml::from_str(&contents).unwrap();
    let root: Root = root.into();

    let mut buf = String::new();

    render_root(&mut buf, &root);
    println!("{}", buf);
}
