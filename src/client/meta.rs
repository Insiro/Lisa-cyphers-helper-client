use cargo_metadata::MetadataCommand;

pub struct Meta {
    version: String,
}
impl Default for Meta {
    fn default() -> Self {
        return Self::new();
    }
}

impl Meta {
    pub fn new() -> Self {
        //TODO: check It work
        return Self {
            version: Self::load(),
        };
    }
    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn load() -> String {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .current_dir(&path)
            .exec()
            .unwrap();
        let root = meta.root_package().unwrap();
        let version = (&root.version).to_string();
        return version;
    }
}
