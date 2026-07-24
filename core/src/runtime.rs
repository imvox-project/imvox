use crate::logger::Logger;
use crate::plugin::Plugin;

pub struct Runtime {
    plugins: Vec<Plugin>,
}

impl Runtime {
    pub fn new() -> Self {
        Logger::info("Runtime created");

        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin: Plugin) {
        Logger::info(format!("Loading plugin '{}'", plugin.name));

        self.plugins.push(plugin);
    }

    pub fn list_plugins(&self) {
        Logger::info("Loaded plugins:");

        for plugin in &self.plugins {
            println!(" - {} {}: '{}'", plugin.name, plugin.version, plugin.desc);
        }
    }
}