//! Test module for the loader: prints "Hello, world!" when run.
//! Compiled as a `.so` and loaded dynamically via `imvoxloader::Loader`.

#[unsafe(no_mangle)]
pub extern "C" fn imvox_plugin_run() {
    println!("Hello, world! (from imvox_hello.so)");
}
