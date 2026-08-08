use crate::plugin::Plugin;

/// Max number of plugins the runtime can hold at once.
///
/// `core` has zero dependencies, so it cannot use `alloc::Vec`. A fixed
/// capacity array keeps everything on the stack / in static memory.
pub const MAX_PLUGINS: usize = 32;

pub struct Runtime {
    plugins: [Option<Plugin>; MAX_PLUGINS],
    len: usize,
}

impl Runtime {
    pub const fn new() -> Self {
        Self {
            plugins: [None; MAX_PLUGINS],
            len: 0,
        }
    }

    /// Register a plugin that `loader` has already resolved from a
    /// dynamic module. Returns `false` if the runtime is full.
    pub fn load_plugin(&mut self, plugin: Plugin) -> bool {
        if self.len >= MAX_PLUGINS {
            return false;
        }
        self.plugins[self.len] = Some(plugin);
        self.len += 1;
        true
    }

    pub fn plugins(&self) -> impl Iterator<Item = &Plugin> {
        self.plugins[..self.len].iter().filter_map(|p| p.as_ref())
    }

    /// Run every loaded plugin in registration order.
    pub fn run_all(&self) {
        for plugin in self.plugins() {
            plugin.run();
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
