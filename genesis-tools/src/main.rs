// mod from_json;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct GenesisCliArgs {

    #[command(subcommand)]
    command: Option<Sub>,
}

#[derive(Subcommand)]
enum Sub {
    /// does testing things
    Fork {
        /// lists test values
        #[arg(short, long)]
        test_mode: bool,
    },
}

fn main() {
    let cli = GenesisCliArgs::parse();
    match cli.command {
        Some(Sub::Fork { test_mode }) => {
          dbg!(&test_mode);
        },
        _ => {},
    }



    // Continued program logic goes here...
}