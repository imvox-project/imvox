#![no_std]

pub mod plugin;
pub mod runtime;

pub use plugin::{Plugin, PluginVTable};
pub use runtime::Runtime;
