use imvoxcore::{Plugin, Runtime};

fn main() {
    println!(r#"
     _                                _ _ 
     (_)_ __ _____   _______  __   ___| (_)
     | | '_ ` _ \ \ / / _ \ \/ /  / __| | |
     | | | | | | \ V / (_) >  <  | (__| | |
     |_|_| |_| |_|\_/ \___/_/\_\  \___|_|_|
    "#);

    let mut runtime = Runtime::new();

    runtime.load_plugin(
        Plugin::new("settings", "0.0.1").desc("Settings"),
    );
    runtime.list_plugins();
}