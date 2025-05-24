fn main() -> std::io::Result<()> {
    // Automatically add ZenobufMessage derive to all protobuf types
    prost_build::Config::new()
        .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
        .compile_protos(&["protos/messages.proto"], &["protos"])?;
    Ok(())
}
