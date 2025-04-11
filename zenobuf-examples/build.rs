//! Build script for the Zenobuf examples

use std::io::Result;

fn main() -> Result<()> {
    // Compile Protocol Buffer definitions
    prost_build::compile_protos(
        &[
            "protos/geometry.proto",
            "protos/example_service.proto",
        ],
        &["protos"],
    )?;
    
    Ok(())
}
