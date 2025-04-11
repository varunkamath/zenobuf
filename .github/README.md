# GitHub Configuration

This directory contains GitHub-specific configuration files:

- Workflows for CI/CD
- Issue templates
- Pull request templates
- GitHub Actions configuration

## Recommended Workflows

Consider adding the following workflows:

1. **Build and Test**: Run on every push and pull request to verify that the code builds and tests pass
2. **Lint**: Check code formatting and linting rules
3. **Release**: Automate the release process when a new tag is created

## Example Workflow

Here's a simple example of a build and test workflow:

```yaml
name: Build and Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Build
      uses: actions-rs/cargo@v1
      with:
        command: build
    - name: Run tests
      uses: actions-rs/cargo@v1
      with:
        command: test
```
