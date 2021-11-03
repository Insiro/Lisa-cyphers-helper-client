use cargo_metadata::MetadataCommand;
#[test]
fn check_version() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();
    let root = meta.root_package().unwrap();
    // let option = root.metadata["my"]["option"].as_str().unwrap();
    let version = &root.version;
    // println!("{}",option);
    println!("{}",version);
}
