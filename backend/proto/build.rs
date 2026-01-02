use std::error::Error;
use std::fs;

const PROTO_DIR: &str = "../../proto/";
const PROTO_FILE_EXT: &str = "proto";

fn main() -> Result<(), Box<dyn Error>> {
    // Collect all .proto files from the directory
    let proto_files: Vec<_> = fs::read_dir(PROTO_DIR)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == PROTO_FILE_EXT))
        .collect();

    let proto_files_str: Vec<String> = proto_files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let proto_files_refs: Vec<&str> = proto_files_str.iter().map(|s| s.as_str()).collect();

    // Configure prost to derive serde traits
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&proto_files_refs, &[PROTO_DIR])?;

    Ok(())
}
