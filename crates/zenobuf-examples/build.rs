//! Build script for the Zenobuf examples

use std::io::Result;

fn main() -> Result<()> {
    // Compile Protocol Buffer definitions with derive macro
    prost_build::Config::new()
        .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
        .compile_protos(
            &["protos/geometry.proto", "protos/example_service.proto"],
            &["protos"],
        )?;

    Ok(())
}
