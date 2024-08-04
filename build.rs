use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(
            &["src/protos/models.proto"],
            &["src/", "src/protos", "third_party/protobuf/src"],
        )
        .unwrap();

    // copy files to generated dir
    let src_generated_dir = PathBuf::from("src/generated");
    std::fs::create_dir_all(&src_generated_dir).unwrap();

    // Copy generated files to the generated directory
    for entry in fs::read_dir(&out_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, src_generated_dir.join(path.file_name().unwrap())).unwrap();
        }
    }
}
