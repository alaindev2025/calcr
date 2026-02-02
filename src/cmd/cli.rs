use clap::CommandFactory;
use clap::Parser;

use crate::cmd::{style, tokenizer};

#[derive(Parser, Debug)]
#[command(version, styles = style::get_style())]
pub struct Cli {
    // #[command(subcommand)]
    // subcommand: Option<Subcommand>,
    expr: Option<Vec<String>>,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Version,
    Fortune,
}

impl Cli {
    pub fn start() {
        let cli = Self::parse();

        match cli.expr {
            Some(v) => {
                let result = tokenizer::parse(v);
                println!("Result: {}", result)
            }
            None => {
                let mut cmd = Self::command();
                cmd.print_help().unwrap();
            }
        }
    }
}
