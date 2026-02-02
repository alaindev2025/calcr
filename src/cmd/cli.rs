use std::io;

use clap::CommandFactory;
use clap::Parser;
use clap_complete;

use crate::cmd::{style, tokenizer};

#[derive(Parser, Debug)]
#[command(version, styles = style::get_style())]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommand>,

    /// Algebraic expression
    expr: Option<Vec<String>>,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    /// Display version information
    Version,

    /// Generate shell completions
    Completion {
        /// Target shell
        sh: clap_complete::Shell,
    },
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
                print!("{}", version);
            }

            Some(Subcommand::Completion { sh }) => {
                let mut cmd = Self::command();
                clap_complete::generate(sh, &mut cmd, "calcr", &mut io::stdout());
            }

            None => {
                let mut cmd = Self::command();
                cmd.print_help().unwrap();
            }
        }
    }
}
