use std::{fs, process::Command};

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    // fs::remove_file("src/pb/google/protobuf/descriptor.proto").unwrap();

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    // 指定哪些文件变化时，重新build它(rebuild)
    println!("cargo:rerun-if-changed=protos/reservation.proto");
    // println!("cargo:rerun-if-changed=protos/google/protobuf/descriptor.proto");
}
