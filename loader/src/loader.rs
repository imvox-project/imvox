use std::collections::HashMap;

use anyhow::{Context, Result};
use libloading::{Library, Symbol};

use imvoxcore::{Plugin, PluginVTable};

use crate::runtime::Runtime;
use crate::types::RawRunFn;

// The plugin manager: owns the plugin lifecycle, resolves .so modules,
// keeps loaded libraries alive, and orchestrates execution through its
// own Runtime. This is the brain of imvox — cli only ever talks to this.
pub struct Loader<'a> {
    runtime: Runtime<'a>,
    // keep libraries alive, their symbols back the plugin vtables in runtime
    libraries: HashMap<String, Library>,
}

impl<'a> Loader<'a> {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
            libraries: HashMap::new(),
        }
    }

    // load a .so file, resolve imvox_plugin_run, register it under name
    pub fn load_module(&mut self, name: &'a str, path: &str) -> Result<()> {
        let lib = unsafe { Library::new(path) }
            .with_context(|| format!("failed to open module at '{path}'"))?;

        let run_fn: RawRunFn = unsafe {
            let sym: Symbol<RawRunFn> = lib
                .get(b"imvox_plugin_run")
                .with_context(|| format!("module '{path}' missing symbol 'imvox_plugin_run'"))?;
            *sym
        };

        let plugin = Plugin::new(name, PluginVTable { run: run_fn });
        self.runtime.load_plugin(plugin);

        // keep the library mapped so the function pointer stays valid
        self.libraries.insert(name.to_string(), lib);

        Ok(())
    }

    // list currently loaded plugin names
    pub fn list_plugins(&self) -> impl Iterator<Item = &str> {
        self.runtime.plugins().map(|p| p.name)
    }

    // run every loaded plugin
    pub fn run_all(&self) {
        self.runtime.run_all();
    }

    // access the underlying runtime
    pub fn runtime(&self) -> &Runtime<'a> {
        &self.runtime
    }
}

impl<'a> Default for Loader<'a> {
    fn default() -> Self {
        Self::new()
    }
}
