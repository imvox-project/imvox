#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

impl Plugin {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}