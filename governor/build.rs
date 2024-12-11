use std::io::Result;

const PROTO_DIR: &str = "src/proto";

fn main() -> Result<()> {
    let proto_files = std::fs::read_dir(PROTO_DIR)?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.path().extension().is_some_and(|extension| extension == "proto") {
                    return Some(entry.path())
                }
            }
            None
        })
        .collect::<Vec<_>>();

    prost_build::compile_protos(&proto_files, &["src/proto/"])?;

    Ok(())
}