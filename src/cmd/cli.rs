use clap::{
    Parser,
    clap_derive::{Args, Subcommand},
};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommands: Cmd,
}

#[derive(Subcommand, Clone)]
pub enum Cmd {
    #[command(about = "Add two numbers", name = "add")]
    Add(Add),

    #[command(about = "Substract two numbers", name = "sub")]
    Sub(Sub),
}

#[derive(Args, Clone)]
pub struct Add {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,
}

#[derive(Args, Clone)]
pub struct Sub {
    #[arg()]
    pub a: f64,

    #[arg()]
    pub b: f64,
}

pub fn run() {
    let cli = Cli::parse();
    let result = match cli.subcommands {
        Cmd::Add(add) => add.a + add.b,
        Cmd::Sub(sub) => sub.a - sub.b,
    };
    println!("{}", result);
}
