// use std::fs;

// mod bbcode;
// mod models;
// mod ui;
mod models;
mod render;

// use models::root::{RawRoot, Root};

// const PATH: &str = "vpp.yaml";

use render::{Body, Buf};

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

        do my_component "Outer" {
            span "Inner!";
        }
    };

    println!("{}", buf);

    Ok(())
}

fn my_component<B: Body>(buf: Buf, name: &'static str, body: B) {
    bbcode! {
        in {{ buf }};

        div {{ name }} {
            yield {{ body }};
        }
    }
}
