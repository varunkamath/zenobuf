# Zenobuf Starter Template

This is a simple starter template for using Zenobuf with your own protobuf definitions.

## Quick Start

1. **Copy this template to your project:**
   ```bash
   cp -r starter-template my-zenobuf-app
   cd my-zenobuf-app
   ```

2. **Update dependencies in Cargo.toml** (change path dependencies to version dependencies):
   ```toml
   [dependencies]
   zenobuf-core = "0.2"
   zenobuf-macros = "0.2"
   ```

3. **Run the example:**
   ```bash
   cargo run
   ```

## Customizing for Your Use Case

### 1. Define Your Messages

Edit `protos/messages.proto` with your own protobuf definitions:

```protobuf
syntax = "proto3";

package my_app;

message MyMessage {
  string name = 1;
  int32 value = 2;
}
```

### 2. Use Your Messages

The build script automatically generates Rust code with the `ZenobufMessage` derive macro. Just use your messages:

```rust
use proto::MyMessage;

// Create publisher
let publisher = node
    .publisher::<MyMessage>("my_topic")
    .build()
    .await?;

// Publish message
let msg = MyMessage {
    name: "Hello".to_string(),
    value: 42,
};
publisher.publish(&msg)?;
```

### 3. Add More Protobuf Files

To add more `.proto` files, update `build.rs`:

```rust
prost_build::Config::new()
    .type_attribute(".", "#[derive(zenobuf_macros::ZenobufMessage)]")
    .compile_protos(&[
        "protos/messages.proto",
        "protos/services.proto",  // Add more files here
    ], &["protos"])?;
```

## What's Included

- ✅ Automatic `ZenobufMessage` derive macro
- ✅ Publisher/Subscriber example
- ✅ Service/Client example
- ✅ Proper error handling
- ✅ Async/await support
- ✅ Clear console output with emojis

## Next Steps

- Replace the example messages with your own
- Add more complex message types
- Implement your business logic
- Deploy with Docker or systemd

Happy coding! 🚀
