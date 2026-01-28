use color_eyre::Result;
mod cmd;
mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    cmd::cli::Cli::run();

    Ok(())
}
