use clap::Parser;

use crate::cmd::{style, tokenizer};

#[derive(Parser, Debug)]
#[command(version, styles = style::get_style())]
pub struct Cli {
    // #[command(subcommand)]
    // subcommand: Option<Subcommand>,
    expr: String,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Version,
    Fortune,
}

impl Cli {
    pub fn start() {
        let cli = Self::parse();

        println!("{}", tokenizer::parse(&cli.expr));
    }
}
