use imvoxcore::{Plugin, Runtime};

fn main() {
    println!("IMVOX CLI alpha-alpha build");

    let mut runtime = Runtime::new();

    runtime.load_plugin(
        Plugin::new("settings", "0.0.1"),
    );

    runtime.load_plugin(
        Plugin::new("voicevox", "0.16.0"),
    );

    runtime.list_plugins();
}