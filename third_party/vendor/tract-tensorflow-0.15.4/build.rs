use std::{env, fs, path};

fn main() -> std::io::Result<()> {
    let mut inputs: Vec<path::PathBuf> = vec![];

    for dir in &[
        "protos/tensorflow/core/framework",
        "protos/tensorflow/core/protobuf",
    ] {
        for pb in fs::read_dir(dir)? {
            inputs.push(pb?.path())
        }
    }

    let gen = path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("prost");
    let _ = fs::create_dir_all(&gen);

    fs::copy(
        "protos/tensorflow/core/pregenerated/tensorflow.rs",
        path::PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("prost")
            .join("tensorflow.rs"),
    )?;

    fs::copy(
        "protos/tensorflow/core/pregenerated/google.protobuf.rs",
        path::PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("prost")
            .join("google.protobuf.rs"),
    )?;

    Ok(())
}
