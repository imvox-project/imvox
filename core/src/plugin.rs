#[derive(Debug, Clone, Default)]
pub struct Plugin {
    pub name: String,
    pub desc: String,
    pub version: String,
}

impl Plugin {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Default::default()
        }
    }
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = desc.into();
        self
    }
}