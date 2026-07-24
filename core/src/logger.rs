pub struct Logger;
impl Logger {
    pub fn info(message: impl AsRef<str>) {
        println!("[INFO] {}", message.as_ref());
    }

    pub fn warn(message: impl AsRef<str>) {
        println!("[WARN] {}", message.as_ref());
    }

    pub fn debug(message: impl AsRef<str>) {
        println!("[DEBUG] {}", message.as_ref());
    }

    pub fn error(message: impl AsRef<str>) {
        eprintln!("[ERROR] {}", message.as_ref());
    }
}