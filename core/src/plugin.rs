//! Plugin ABI definitions.
//!
//! `core` is `no_std`, zero-dep. Defines the stable contract for `.so` modules.
//! `loader` handles dynamic loading and fills the function pointers.

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PluginVTable {
    pub run: unsafe extern "C" fn(),
}

#[derive(Clone, Copy)]
pub struct Plugin {
    pub name: &'static str,
    pub vtable: PluginVTable,
}

impl Plugin {
    pub const fn new(name: &'static str, vtable: PluginVTable) -> Self {
        Self { name, vtable }
    }

    // Caller must ensure the symbol actually exists and matches this signature.
    pub fn run(&self) {
        unsafe { (self.vtable.run)() }
    }
}