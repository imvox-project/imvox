//! Dynamic module loader.
//!
//! This crate is the *only* place `std` and dynamic-loading logic live.
//! It opens `.so` files, resolves their exported symbols against the
//! ABI defined in `imvoxcore`, and registers the result into a
//! `imvoxcore::Runtime`. Shells (like `imvoxcli`) never touch `.so` files or
//! `core` directly — they go through `Loader`.

use std::collections::HashMap;

use anyhow::{Context, Result};
use libloading::{Library, Symbol};

use imvoxcore::{Plugin, PluginVTable, Runtime};

/// Every module must export a function with this exact C signature,
/// named `imvox_plugin_run`.
type RawRunFn = unsafe extern "C" fn();

/// Loads `.so` modules and feeds them into a `no_std` `imvoxcore::Runtime`.
///
/// Keeps loaded `Library` handles alive for as long as the loader lives,
/// since the function pointers stored in `core::Plugin` point into that
/// mapped memory.
pub struct Loader {
    runtime: Runtime,
    // Keep libraries alive; their symbols back the Plugin vtables in `runtime`.
    libraries: HashMap<String, Library>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
            libraries: HashMap::new(),
        }
    }

    /// Load a `.so` file, resolve its `imvox_plugin_run` symbol, and
    /// register it with the underlying `core::Runtime` under `name`.
    ///
    /// # Safety
    /// This calls into arbitrary external code. The `.so` must export a
    /// symbol `imvox_plugin_run` matching `extern "C" fn()`.
    pub fn load_module(&mut self, name: &'static str, path: &str) -> Result<()> {
        let lib = unsafe { Library::new(path) }
            .with_context(|| format!("failed to open module at '{path}'"))?;

        let run_fn: RawRunFn = unsafe {
            let sym: Symbol<RawRunFn> = lib
                .get(b"imvox_plugin_run")
                .with_context(|| format!("module '{path}' missing symbol 'imvox_plugin_run'"))?;
            *sym
        };

        let plugin = Plugin::new(name, PluginVTable { run: run_fn });

        if !self.runtime.load_plugin(plugin) {
            anyhow::bail!("runtime is full, cannot load module '{path}'");
        }

        // Keep the library mapped so the function pointer stays valid.
        self.libraries.insert(name.to_string(), lib);

        Ok(())
    }

    /// List currently loaded plugin names.
    pub fn list_plugins(&self) -> impl Iterator<Item = &str> {
        self.runtime.plugins().map(|p| p.name)
    }

    /// Run every loaded plugin.
    pub fn run_all(&self) {
        self.runtime.run_all();
    }

    /// Access the underlying `core` runtime, if a caller needs it directly.
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}
