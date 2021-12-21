use std::{
    env, fs,
    path::{Path, PathBuf},
};

const COPY_DIR: &'static str = "build_artifacts";

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

fn main() {
    // Request the output directory
    let out = env::var("OUT_DIR").unwrap();
    let out = PathBuf::from(out);

    // If it is already in the output directory, delete it and start over
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    // Create the out directory
    fs::create_dir(&out).unwrap();

    // Copy the directory
    copy_dir(COPY_DIR, &out);

println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i8_8x8.tmpl");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i8_8x8.tmpl");
println!("cargo:rerun-if-changed=src/frame/mmm/fuse.rs");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_row.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/dispatcher.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_scalar.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_col.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_sigmoid_f32.tmpl");
println!("cargo:rerun-if-changed=x86_64/fma/fma_sigmoid_f32.tmpl");
println!("cargo:rerun-if-changed=src/frame/mmm/fuse.rs");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_row.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/dispatcher.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_scalar.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_col.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_64x1.tmpl");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_64x1.tmpl");
println!("cargo:rerun-if-changed=src/frame/mmm/fuse.rs");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_row.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/dispatcher.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_scalar.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_col.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_tanh_f32.tmpl");
println!("cargo:rerun-if-changed=x86_64/fma/fma_tanh_f32.tmpl");
println!("cargo:rerun-if-changed=src/frame/mmm/fuse.rs");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_row.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/dispatcher.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_scalar.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_col.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_16x6.tmpl");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_16x6.tmpl");
println!("cargo:rerun-if-changed=src/frame/mmm/fuse.rs");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_row.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_scalars.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_cols.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/dispatcher.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_scalar.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_ymm_per_col.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_i32_per_rows.tmpliq");
println!("cargo:rerun-if-changed=x86_64/fma/fma_mmm_f32_scalars.tmpliq");
println!("cargo:rustc-link-lib=static=x86_64_fma");
println!("cargo:rustc-link-search=native=/workspace/oak_functions/target/x86_64-unknown-linux-musl/release/build/tract-linalg-51a99002cb70de6f/out");
}
