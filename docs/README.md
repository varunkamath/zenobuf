# Zenobuf Project Documentation

## Project Structure

The Zenobuf project follows a flat workspace layout with all crates organized under the `crates/` directory:

```
zenobuf/
  Cargo.toml       # Workspace manifest
  Cargo.lock       # Shared lockfile
  crates/
    zenobuf-core/  # Core library functionality
    zenobuf-macros/ # Procedural macros
    zenobuf-cli/   # Command-line tools
    zenobuf-examples/ # Example applications
  docs/           # Documentation
  .github/        # GitHub workflows, templates, etc.
  README.md
  LICENSE
```

## Crates

### zenobuf-core

Core library functionality for the Zenobuf framework. This crate provides the fundamental abstractions and implementations for the ROS-like framework using Zenoh for transport and protobuf for serialization.

### zenobuf-macros

Procedural macros for the Zenobuf framework. This crate provides macros that simplify working with the framework.

### zenobuf-cli

Command-line tools for the Zenobuf framework. This crate provides utilities for working with the framework from the command line.

### zenobuf-examples

Example applications demonstrating how to use the Zenobuf framework. These examples show how to create publishers, subscribers, services, and clients.

## Development

To build the project:

```bash
cargo build
```

To run the examples:

```bash
# Start the service
cargo run --bin service

# In another terminal, run the client
cargo run --bin client 1 2
```
