use anyhow::Result;
use clap::{Parser, Subcommand};
use imvoxloader::Loader;

/// IMVOX CLI — talks to the module loader, never to core directly.
#[derive(Parser)]
#[command(name = "imvox", version, about = "IMVOX CLI interface, for testing only")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Load a .so module and run it.
    Load {
        /// Path to the .so file.
        path: String,
        /// Name to register the plugin under.
        #[arg(short, long, default_value = "module")]
        name: String,
    },
    /// List currently loaded plugins.
    List,
}

fn main() -> Result<()> {
    println!(
        r#"
     _                                _ _ 
     (_)_ __ _____   _______  __   ___| (_)
     | | '_ ` _ \ \ / / _ \ \/ /  / __| | |
     | | | | | | \ V / (_) >  <  | (__| | |
     |_|_| |_| |_|\_/ \___/_/\_\  \___|_|_|
    "#
    );

    let cli = Cli::parse();
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
