use std::process::Command;

use tonic_build::Builder;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sql_type(&["reservation.ReservationStatus"])
        .with_builder(&["reservation.ReservationQuery"])
        .with_builder_into(
            "reservation.ReservationQuery",
            &[
                "user_id",
                "resource_id",
                "status",
                "desc",
                "page",
                "page_size",
            ],
        )
        .with_builder_option("reservation.ReservationQuery", &["start", "end"])
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    // fs::remove_file("src/pb/google.protobuf.proto").unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    // 指定哪些文件变化时，重新build它(rebuild)
    println!("cargo:rerun-if-changed=protos/reservation.proto");
    // println!("cargo:rerun-if-changed=protos/google/protobuf/descriptor.proto");
}

trait BuilderExt {
    fn with_sql_type(self, paths: &[&str]) -> Self;
    fn with_builder(self, paths: &[&str]) -> Self;
    fn with_builder_into(self, path: &str, fields: &[&str]) -> Self;
    fn with_builder_option(self, path: &str, fields: &[&str]) -> Self;
}

impl BuilderExt for Builder {
    fn with_sql_type(self, paths: &[&str]) -> Self {
        paths.into_iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(sqlx::Type)]")
        })
    }

    fn with_builder(self, paths: &[&str]) -> Self {
        paths.into_iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(derive_builder::Builder)]")
        })
    }

    fn with_builder_into(self, path: &str, fields: &[&str]) -> Self {
        fields.into_iter().fold(self, |acc, field| {
            acc.field_attribute(
                &format!("{}.{}", path, field),
                "#[builder(setter(into), default)]",
            )
        })
    }

    fn with_builder_option(self, path: &str, fields: &[&str]) -> Self {
        fields.into_iter().fold(self, |acc, field| {
            acc.field_attribute(
                &format!("{}.{}", path, field),
                "#[builder(setter(into, strip_option))]",
            )
        })
    }
}
