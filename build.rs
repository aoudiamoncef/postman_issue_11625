use glob::glob;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = find_protos("proto/postman");

    tonic_build::configure()
        .build_server(true)
        .build_transport(true)
        .build_client(true)
        .type_attribute(
            ".google.api.HttpRule",
            "#[cfg(not(doctest))]\n\
             #[allow(dead_code)]\n\
             pub struct HttpRuleComment{}\n\
             /// HACK: see docs in [`HttpRuleComment`] ignored in doctest pass",
        )
        .file_descriptor_set_path("src/api.bin")
        .include_file("_includes.rs")
        .out_dir("src/")
        .compile(
            &protos,
            &["proto/postman", "proto/third-party"], // specify the root location to search proto dependencies
        )?;

    Ok(())
}

fn find_protos(dir_path: &str) -> Vec<PathBuf> {
    glob(&format!("{dir_path}/**/*.proto"))
        .unwrap()
        .flatten()
        .collect()
}
