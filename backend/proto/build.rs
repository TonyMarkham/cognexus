const PROTO_FILES: &[&str] = &["../../proto/commands.proto", "../../proto/events.proto"];

const PROTO_INCLUDE: &[&str] = &["../../proto/"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(PROTO_FILES, PROTO_INCLUDE)?;
    Ok(())
}
