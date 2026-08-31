use clap::{Parser, Subcommand};

use crate::logo::LogoMode;

/// IMVOX CLI — talks to the module loader, never to core directly.
#[derive(Parser)]
#[command(name = "imvox", version, about = "imvox cli interface, for testing purpose only")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Logo display mode: show, hide, or minimal
    #[arg(short, long, value_enum, default_value = "show")]
    pub logo: LogoMode,
}

#[derive(Subcommand)]
pub enum Command {
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