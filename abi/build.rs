use proto_builder_trait::tonic::BuilderAttributes;
use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_type_attributes(
            &["reservation.ReservationStatus"],
            &["#[derive(sqlx::Type)]"],
        )
        .with_type_attributes(
            &[
                "reservation.ReservationQuery",
                "reservation.ReservationFilter",
            ],
            &["#[derive(derive_builder::Builder)]"], // 生产新的Builder对象，名字带有Builder后缀，
        )
        .with_field_attributes(
            &[
                "reservation.ReservationQuery.user_id",
                "reservation.ReservationQuery.resource_id",
                "reservation.ReservationQuery.status",
                "reservation.ReservationQuery.desc",
                "reservation.ReservationQuery.page",
                // "reservation.ReservationQuery.page_size",
            ],
            &["#[builder(setter(into), default)]"],
        )
        .with_field_attributes(
            &[
                "reservation.ReservationFilter.user_id",
                "reservation.ReservationFilter.resource_id",
                "reservation.ReservationFilter.status",
                "reservation.ReservationFilter.cursor",
                "reservation.ReservationFilter.desc",
                // "reservation.ReservationFilter.page_size",
            ],
            &["#[builder(setter(into), default)]"],
        )
        .with_field_attributes(
            &["page_size"],
            &["#[builder(setter(into), default = \"10\")]"],
        )
        .with_field_attributes(
            &[
                "reservation.ReservationQuery.start",
                "reservation.ReservationQuery.end",
            ],
            &["#[builder(setter(into, strip_option))]"],
        )
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    // fs::remove_file("src/pb/google.protobuf.proto").unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    // 指定哪些文件变化时，重新build它(rebuild)
    println!("cargo:rerun-if-changed=protos/reservation.proto");
    // println!("cargo:rerun-if-changed=protos/google/protobuf/descriptor.proto");
}

// trait BuilderExt {
//     fn with_sql_type(self, paths: &[&str]) -> Self;
//     fn with_builder(self, paths: &[&str]) -> Self;
//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self;
//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self;
// }

// impl BuilderExt for Builder {
//     fn with_sql_type(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |acc, path| {
//             acc.type_attribute(path, "#[derive(sqlx::Type)]")
//         })
//     }

//     fn with_builder(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |acc, path| {
//             acc.type_attribute(path, "#[derive(derive_builder::Builder)]")
//         })
//     }

//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |acc, field| {
//             println!("acc:{:?}-{:?}", acc, field);
//             acc.field_attribute(
//                 &format!("{}.{}", path, field),
//                 "#[builder(setter(into), default)]",
//             )
//         })
//     }

//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |acc, field| {
//             acc.field_attribute(
//                 &format!("{}.{}", path, field),
//                 "#[builder(setter(into, strip_option))]",
//             )
//         })
//     }
// }
