use argh::FromArgs;

mod bbcode;
mod cli;
mod models;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "Commands for building Dakota's VPP thread.")]
struct Cli {
    #[argh(subcommand)]
    command: CliCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CliCommand {
    MakeCss(cli::make::Opts),
}

impl CliCommand {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Self::MakeCss(opts) => cli::make::run(opts),
        }
    }
}

fn main() -> anyhow::Result<()> {
    argh::from_env::<Cli>().command.run()
}
