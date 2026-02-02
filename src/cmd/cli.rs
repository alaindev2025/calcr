use crate::cmd::style::get_style;
use std::io;

use clap::{
    CommandFactory, Parser,
    clap_derive::{Args, Subcommand},
};

use crate::config;

#[derive(Parser)]
#[command(version, styles = get_style())]
pub struct Cli {
    #[command(subcommand)]
    pub subcommands: Cmd,
}

#[derive(Subcommand, Clone)]
#[allow(non_camel_case_types)]
pub enum Cmd {
    #[command(about = "Add two numbers", name = "add")]
    Add(Add),

    #[command(about = "Substract two numbers", name = "sub")]
    Sub(Sub),

    #[command(about = "Multiplies two numbers")]
    Mul(Mul),

    #[command(about = "Divisies two numbers")]
    Div(Div),

    #[command(name = "print-config", about = "Prints the config")]
    PrintConfig,

    /// Generates completions for shell
    Completion,
}

#[derive(Args, Clone)]
pub struct Add {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,

    /// Just print the result (without `Result:`)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Args, Clone)]
pub struct Sub {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,

    /// Just print the result (without `Result:`)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Args, Clone)]
pub struct Mul {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,

    /// Just print the result (without `Result:`)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Args, Clone)]
pub struct Div {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,

    /// Just print the result (without `Result:`)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

trait Printer {
    fn print_result(&self);
}

impl Printer for Add {
    fn print_result(&self) {
        if self.quiet {
            println!("{}", self.a + self.b);
        } else {
            println!("Result: {}", self.a + self.b);
        }
    }
}

impl Printer for Sub {
    fn print_result(&self) {
        if self.quiet {
            println!("{}", self.a - self.b);
        } else {
            println!("Result: {}", self.a - self.b);
        }
    }
}

impl Printer for Mul {
    fn print_result(&self) {
        if self.quiet {
            println!("{}", self.a * self.b);
        } else {
            println!("Result: {}", self.a * self.b);
        }
    }
}
impl Printer for Div {
    fn print_result(&self) {
        if self.b != 0.0 {
            if self.quiet {
                println!("{}", self.a / self.b);
            } else {
                println!("Result: {}", self.a / self.b);
            }
        }
    }
}

impl Cli {
    /// Starts the Cli app
    pub fn run() {
        #[allow(unused_variables)]
        let opts = config::Config::load().unwrap();
        let cli = Cli::parse();

        match &cli.subcommands {
            Cmd::Add(add) => add.print_result(),
            Cmd::Sub(sub) => sub.print_result(),
            Cmd::Mul(mul) => mul.print_result(),
            Cmd::Div(div) => div.print_result(),
            Cmd::PrintConfig => println!("Coming soon"),
            Cmd::Completion => {
                clap_complete::generate(
                    clap_complete::Shell::Fish,
                    &mut Cli::command(),
                    "calcr",
                    &mut io::stdout(),
                );
            }
        };
    }
}
