# What is imvoxloader?
The loader is a layer between the interface and the core, std by default, needed to easily load libraries into the core without hardcoding this logic into the interface. It may also be no_std in the future.

# This crate includes:
- `Loader`, the plugin manager - it owns the plugin lifecycle end to end
- `.so` loading and symbol resolution via `libloading`
- Its own `Runtime`, which holds loaded plugins and runs them
- Keeping loaded libraries alive so their symbols stay valid while plugins run

# This crate excludes:
- Argument parsing or anything user-facing - that's cli's job
- The ABI contract itself (`Plugin`, `PluginVTable`) - that lives in core, loader just builds on it

# Build project:
Run `cargo build --workspace` from the imvox repo root. loader gets built along with core and cli.

# How to use builded library?
Add it as a path dependency and pull in `Loader`:
```rust
use imvoxloader::Loader;

let mut loader = Loader::new();
loader.load_module("my_plugin", "/path/to/module.so")?;
loader.run_all();
```
