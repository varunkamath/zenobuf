# Contributing to Zenobuf

Thank you for your interest in contributing to Zenobuf! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Development Workflow

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/zenobuf.git`
3. Create a new branch: `git checkout -b my-feature-branch`
4. Make your changes
5. Run tests: `cargo test --all-features --workspace`
6. Commit your changes (see below for commit message guidelines)
7. Push to your fork: `git push origin my-feature-branch`
8. Open a pull request

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages. This leads to more readable messages that are easy to follow when looking through the project history and enables automatic generation of the changelog.

### Commit Message Format

Each commit message consists of a **header**, a **body**, and a **footer**:

```
<type>(<scope>): <subject>

<body>

<footer>
```

The **header** is mandatory and must conform to the following format:

```
<type>(<scope>): <subject>
```

#### Type

Must be one of the following:

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to our CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

#### Scope

The scope should be the name of the crate affected (e.g., `core`, `macros`, `cli`, etc.).

#### Subject

The subject contains a succinct description of the change:

- Use the imperative, present tense: "change" not "changed" nor "changes"
- Don't capitalize the first letter
- No dot (.) at the end

#### Body

The body should include the motivation for the change and contrast this with previous behavior.

#### Footer

The footer should contain any information about **Breaking Changes** and is also the place to reference GitHub issues that this commit **Closes**.

Breaking Changes should start with the word `BREAKING CHANGE:` with a space or two newlines. The rest of the commit message is then used for this.

### Examples

```
feat(core): add support for custom serialization

Adds a new trait for custom serialization of messages.

Closes #123
```

```
fix(transport): resolve deadlock in service calls

This fixes a deadlock that could occur when multiple service calls were made simultaneously.

Closes #456
```

```
refactor(macros): simplify code generation

Simplifies the code generation logic to make it more maintainable.
```

```
docs(examples): add more comprehensive examples

Adds more examples to demonstrate how to use the framework in different scenarios.
```

## Development and Release Process

We follow a main-dev branch structure:

1. **Development**: All development work happens on the `dev` branch or feature branches that are merged into `dev`.

2. **Releases**: When ready for a release, create a pull request from `dev` to `main`. Merging this PR will automatically trigger the release process.

3. **Manual Release**: You can also manually trigger a release using the "Unified CI/CD" workflow from the Actions tab. This allows you to specify the version bump type (patch, minor, major) or a custom version.

## Changelog

The changelog is automatically generated from conventional commit messages using [git-cliff](https://github.com/orhun/git-cliff). You can manually generate the changelog with:

```bash
git-cliff --unreleased
```

## Code Style

We use `rustfmt` and `clippy` to maintain code quality. Please ensure your code passes both:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```
