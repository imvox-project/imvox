mod cli;
mod logo;

// for random name hash
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher,Hasher};
use std::path::Path;

use anyhow::Result;
use clap::Parser;

use imvoxloader::Loader;

use cli::{Cli, Command};
use logo::print_logo;

/// builds a default plugin name from the module's file stem plus a random hash
/// e.g. "my_plugin-xxxxxxxxxxxxxxxx"
fn default_plugin_name(path: &str) -> String {
    let stem = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");
    let hash = RandomState::new().build_hasher().finish();
    format!("{stem}-{hash:x}")
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    print_logo(&cli.logo);

    let mut loader = Loader::new();

    match cli.command.unwrap_or(Command::List) {
        Command::Load { path, name } => {
            let name = name.unwrap_or_else(|| default_plugin_name(&path));
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
