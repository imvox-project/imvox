// dynamic module loader — the plugin manager and runtime for imvox.
// only crate using std and dlopen logic; owns plugin lifecycle, module
// management and execution. shells (like cli) go through Loader, never
// touch .so files or imvoxcore directly.

mod loader;
mod runtime;
mod types;

pub use loader::Loader;
pub use runtime::Runtime;
