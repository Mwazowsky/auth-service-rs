use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set the output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Configure and compile the protobuf files
    tonic_build::configure()
        .build_server(true)
        .out_dir(&out_dir)
        .file_descriptor_set_path(out_dir.join("user_descriptor.bin"))
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
    
    Ok(())
}
