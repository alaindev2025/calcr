use clap::CommandFactory;
use clap::Parser;

use crate::cmd::{style, tokenizer};

#[derive(Parser, Debug)]
#[command(version, styles = style::get_style())]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommand>,

    expr: Option<Vec<String>>,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Version,
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
                Self::handle_subcommands();
            }
        }
    }

    fn handle_subcommands() {
        let cli = Self::parse();

        match cli.subcommand {
            Some(Subcommand::Version) => {
                let version = Self::command().render_version();
                println!("{}", version);
            }
            None => {
                let mut cmd = Self::command();
                cmd.print_help().unwrap();
            }
        }
    }
}
