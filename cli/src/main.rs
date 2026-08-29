use anyhow::Result;
use clap::Parser;

use imvoxloader::Loader;

mod cli;
mod logo;

use cli::{Cli, Command};
use logo::print_logo;

fn main() -> Result<()> {
    let cli = Cli::parse();
    print_logo(&cli.logo);

    let mut loader = Loader::new();

    match cli.command.unwrap_or(Command::List) {
        Command::Load { path, name } => {
            let leaked_name: &'static str = Box::leak(name.into_boxed_str());
            loader.load_module(leaked_name, &path)?;
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
