// dynamic module loader
// only crate using std and dlopen logic
// shells go through loader, never touch .so or core directly

mod loader;
mod types;

pub use loader::Loader;
