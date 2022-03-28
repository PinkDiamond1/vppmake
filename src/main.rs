// use std::fs;

// mod bbcode;
// mod models;
// mod ui;
mod models;
mod render;

// use models::root::{RawRoot, Root};

// const PATH: &str = "vpp.yaml";

fn main() -> anyhow::Result<()> {
    // let contents = fs::read_to_string(PATH)?;

    // let root: RawRoot = serde_yaml::from_str(&contents)?;
    // let root: Root = root.into();

    // dbg!(root);

    // let mut string = String::new();

    // let root = Root { post_count: 1 };
    // let component = RootComponent::new(&root);

    // component.render(&mut string, ());

    // println!("{}", string);

    let mut buf = String::new();

    bbcode! {
        in {{ buf }};

        for n in {{ [1, 2, 3] }} {
            div {
                {{ n }};
            }
        }
    };

    println!("{}", buf);

    Ok(())
}
