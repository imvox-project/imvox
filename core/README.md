# What is imvoxcore?
The imvox library, no_std, should almost NEVER be linked to an interface (like cli) and should work through a loader layer  
The core should remain almost unchanged  
The interface and loader should  

# This crate includes:
- The `Plugin` and `PluginVTable` types - the ABI contract that a `.so` module and the loader agree on
- A minimal `run()` call that invokes a plugin's vtable function, nothing more

# This crate excludes:
- Any plugin lifecycle logic (loading, unloading, registration) - that's loader's job
- Any runtime state, globals, or constants that bake in a specific behaviour
- Any dependencies at all, including on `std`

# Getting Started:
This crate is not meant to be used on its own - add `imvoxloader` to your project instead, it re-exports what you need from here.
If you're writing a plugin, this is the crate whose types your `.so` needs to match.

# How to use builded library?
You don't build this crate by itself in normal use. It gets pulled in automatically as a dependency of `imvoxloader` when you build the workspace with interface, like cli, via `cargo build --workspace`.