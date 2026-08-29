use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum LogoMode {
    Show,
    Hide,
    Minimal,
}

pub fn print_logo(mode: &LogoMode) {
    match mode {
        LogoMode::Show => {
            println!(
                r#"
     _                                _ _
     (_)_ __ _____   _______  __   ___| (_)
     | | '_ ` _ \ \ / / _ \ \/ /  / __| | |
     | | | | | | \ V / (_) >  <  | (__| | |
     |_|_| |_| |_|\_/ \___/_/\_\  \___|_|_|
                "#
            );
        }
        LogoMode::Hide => {}
        LogoMode::Minimal => {
            println!("imvox cli");
        }
    }
}