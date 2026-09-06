//! The actual plugin runtime.
//!
//! This used to live in `imvoxcore`, but holding plugin state and driving
//! execution is runtime logic, not a shared contract — so it lives here,
//! in the crate that owns the plugin lifecycle. `core` only supplies the
//! `Plugin`/`PluginVTable` types this module is built on.

use imvoxcore::Plugin;

/// Owns every plugin that has been registered and can run them.
///
/// Unlike the old `core`-side version, this isn't capped by a fixed-size
/// array — `loader` is a `std` crate, so a `Vec` is the natural fit and
/// there's no artificial `MAX_PLUGINS` limit baked into the contract crate.
pub struct Runtime<'a> {
    plugins: Vec<Plugin<'a>>,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Register a plugin that `Loader` has already resolved from a
    /// dynamic module.
    pub fn load_plugin(&mut self, plugin: Plugin<'a>) {
        self.plugins.push(plugin);
    }

    pub fn plugins(&self) -> impl Iterator<Item = &Plugin<'a>> {
        self.plugins.iter()
    }

    /// Run every loaded plugin in registration order.
    pub fn run_all(&self) {
        for plugin in self.plugins() {
            plugin.run();
        }
    }
}

impl<'a> Default for Runtime<'a> {
    fn default() -> Self {
        Self::new()
    }
}
