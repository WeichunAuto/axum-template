# Axum Web Project Template

This repository provides a **basic project structure template** for building web applications using [Axum](https://github.com/tokio-rs/axum).
It is suitable for quickly bootstrapping new projects while maintaining flexibility for future growth.

---

## How to use ?

```bash
cargo generate https://github.com/WeichunAuto/axum-template
```

## Prerequisites

Before using this template, please ensure that:

- You have a **basic understanding of Rust**.
- The **Rust toolchain** is properly installed. If not, install it via [rustup](https://rustup.rs/).
- Git is installed and configured.
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) installed:

```bash
cargo install cargo-generate
```

---

## Development Tools Required

The template is using the following tools:

### Install [pre-commit](https://pre-commit.com/)

pre-commit is a code checking tool that runs checks before you commit your code.

```bash
pipx install pre-commit
pre-commit install
```

### Install [typos](https://github.com/crate-ci/typos)

typos is a spell-checking tool.

```bash
cargo install typos-cli
```

### Install [git cliff](https://github.com/orhun/git-cliff)

git cliff is a tool for generating changelogs.

```bash
cargo install git-cliff
```

### Install [cargo watch](https://github.com/watchexec/cargo-watch)

The purpose of cargo watch is to monitor changes in your project’s source files and automatically execute the specified cargo command.
Whenever you save a file, it will automatically recompile, run, or test the project.

```bash
cargo install cargo-watch
```

### Install [sea-orm](https://www.sea-ql.org/SeaORM/docs)
SeaORM is a relational ORM to help you build web services in Rust
```bash
cargo install sea-orm-cli@^2.0.0-rc
```

Usage:

```bash
# Start the project with auto-reload
cargo watch -x 'run'

# Start with custom host & port
APP_SERVER_HOST=127.0.0.1 APP_SERVER_PORT=8080 cargo watch -x 'run'
```
