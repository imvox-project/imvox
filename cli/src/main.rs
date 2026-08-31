mod cli;
mod logo;

use anyhow::Result;
use clap::Parser;

use imvoxloader::Loader;

use cli::{Cli, Command};
use logo::print_logo;

fn main() -> Result<()> {
    let cli = Cli::parse();
    print_logo(&cli.logo);

    let mut loader = Loader::new();

    match cli.command.unwrap_or(Command::List) {
        Command::Load { path, name } => {
            loader.load_module(&name, &path)?;
            loader.run_all();
        }
        Command::List => {
            for name in loader.list_plugins() {
                println!(" - {name}");
            }
        }
    }

    Ok(())
}
