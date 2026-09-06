#![no_std]

//! `imvoxcore` - shared contract between `loader` and any plugin module
//!
//! This crate defines only the ABI-level types and traits that the rest of
//! the workspace depends on. It has no runtime logic, no state, and no
//! dependencies. `loader` is the crate that actually implements plugin
//! lifecycle and execution using these types

pub mod plugin;

pub use plugin::{Plugin, PluginVTable};
