# Zenobuf Documentation

Welcome to the Zenobuf documentation! Zenobuf is a lightweight, ergonomic framework for building distributed systems in Rust, inspired by ROS (Robot Operating System) but designed specifically for Rust's strengths.

## 📚 Documentation Structure

### For New Users
- **[Getting Started Guide](getting-started.md)** - Complete tutorial from installation to building your first distributed system
- **[Starter Template](../starter-template/)** - Ready-to-use project template

### For Developers
- **[API Reference Guide](api-guide.md)** - Comprehensive API documentation with examples
- **[Architecture Overview](architecture.md)** - System design and internal architecture

### For Contributors
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute to the project
- **[Changelog](../CHANGELOG.md)** - Project history and changes

## 🚀 Quick Start

The fastest way to get started:

```bash
# Clone and use the starter template
git clone https://github.com/varunkamath/zenobuf
cp -r zenobuf/starter-template my-zenobuf-app
cd my-zenobuf-app

# Update Cargo.toml to use published crates
# zenobuf-core = "0.2"
# zenobuf-macros = "0.2"

cargo run
```

## 📦 Crate Overview

### [zenobuf-core](https://docs.rs/zenobuf-core)
The main library providing:
- **Node management** - Central coordination point
- **Pub/Sub messaging** - Type-safe publish-subscribe communication
- **RPC services** - Request-response communication
- **Parameter system** - Distributed configuration management
- **QoS profiles** - Quality of service guarantees

### [zenobuf-macros](https://docs.rs/zenobuf-macros)
Procedural macros for seamless Protocol Buffer integration:
- **`#[derive(ZenobufMessage)]`** - Automatic Message trait implementation
- **Build script integration** - Automatic code generation

### [zenobuf-cli](https://docs.rs/zenobuf-cli)
Command-line tools for development and debugging:
- **Topic monitoring** - `zenobuf-cli monitor <topic>`
- **Service calls** - `zenobuf-cli call <service>`
- **Parameter management** - `zenobuf-cli param get/set`
- **System inspection** - `zenobuf-cli list nodes/topics/services`

### [zenobuf-examples](https://docs.rs/zenobuf-examples)
Complete working examples:
- **Publisher/Subscriber** - Basic messaging patterns
- **Service/Client** - RPC communication
- **Parameter usage** - Configuration management
- **Complete applications** - Real-world scenarios

## 🏗️ Project Structure

```
zenobuf/
├── Cargo.toml              # Workspace configuration
├── README.md               # Project overview
├── CHANGELOG.md            # Version history
├── CONTRIBUTING.md         # Contribution guidelines
├── crates/                 # All Rust crates
│   ├── zenobuf-core/       # Core library
│   ├── zenobuf-macros/     # Procedural macros
│   ├── zenobuf-cli/        # Command-line tools
│   └── zenobuf-examples/   # Example applications
├── docs/                   # Documentation
│   ├── getting-started.md  # Tutorial guide
│   ├── api-guide.md        # API reference
│   ├── architecture.md     # System design
│   └── README.md           # This file
└── starter-template/       # Quick start template
    ├── Cargo.toml
    ├── build.rs
    ├── protos/
    └── src/
```

## 🔧 Development Workflow

### Building the Project
```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p zenobuf-core

# Build with all features
cargo build --all-features
```

### Running Examples
```bash
# Run the complete example
cargo run --bin complete_app

# Run publisher/subscriber pair
cargo run --bin talker &
cargo run --bin listener

# Run service/client pair
cargo run --bin service &
cargo run --bin client 5 3
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p zenobuf-core

# Run integration tests
cargo test --test integration_tests
```

### Documentation
```bash
# Generate and open docs
cargo doc --open

# Generate docs for all crates
cargo doc --workspace --no-deps
```

## 🌟 Key Features

### Type Safety
- **Compile-time guarantees** - Message types are checked at compile time
- **Protocol Buffer integration** - Automatic code generation with type safety
- **Builder patterns** - Fluent, type-safe API design

### Performance
- **Zero-copy messaging** - Efficient serialization with minimal copying
- **Async-first design** - Built on Tokio for high concurrency
- **Zenoh transport** - High-performance, low-latency networking

### Developer Experience
- **Ergonomic API** - Intuitive, Rust-idiomatic interfaces
- **Comprehensive examples** - Real-world usage patterns
- **Rich tooling** - CLI tools for development and debugging
- **Excellent documentation** - Guides, API docs, and architecture overview

## 🎯 Use Cases

### Robotics
- **Sensor data processing** - High-frequency sensor data streams
- **Control systems** - Real-time control loops
- **Multi-robot coordination** - Distributed robot fleets

### IoT Systems
- **Device communication** - Sensor networks and actuator control
- **Edge computing** - Distributed processing at the edge
- **Data aggregation** - Collecting and processing IoT data

### Distributed Applications
- **Microservices** - Service-oriented architectures
- **Event-driven systems** - Reactive, event-based applications
- **Real-time systems** - Low-latency, high-throughput applications

## 📖 Learning Path

1. **Start Here**: [Getting Started Guide](getting-started.md)
   - Installation and setup
   - Your first Zenobuf application
   - Basic concepts and patterns

2. **Deep Dive**: [API Reference Guide](api-guide.md)
   - Complete API documentation
   - Advanced usage patterns
   - Performance optimization

3. **Understand the System**: [Architecture Overview](architecture.md)
   - System design principles
   - Internal architecture
   - Performance characteristics

4. **Explore Examples**: [zenobuf-examples](../crates/zenobuf-examples/)
   - Working code examples
   - Best practices
   - Real-world patterns

## 🤝 Community and Support

- **Documentation**: [docs.rs](https://docs.rs/zenobuf-core)
- **Source Code**: [GitHub](https://github.com/varunkamath/zenobuf)
- **Issues**: [GitHub Issues](https://github.com/varunkamath/zenobuf/issues)
- **Discussions**: [GitHub Discussions](https://github.com/varunkamath/zenobuf/discussions)

## 📄 License

Zenobuf is dual-licensed under either:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.

---

**Ready to build distributed systems in Rust?** Start with the [Getting Started Guide](getting-started.md)! 🚀
