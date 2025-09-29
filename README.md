# This is A Axum WEB Project Base Structure Template

## Overview

This repository provides a foundational template for building web applications with Axum.
It is designed to accelerate project setup by including commonly used developer tools and configuration, ensuring consistent workflows and improved developer productivity.

## Prerequisites

Before using this template, please ensure that:

- You have a basic understanding of Rust and its ecosystem.
- The Rust toolchain (via rustup
  ) is properly installed and available in your environment.

## Install pre-commit

pre-commit is a code checking tool that runs checks before you commit your code.

```bash
pipx install pre-commit
```

After installation, run pre-commit install to enable it.

## Install typos

typos is a spell-checking tool.

```bash
cargo install typos-cli
```

## Install git cliff

git cliff is a tool for generating changelogs.

```bash
cargo install git-cliff
```

## Install cargo watch

The purpose of cargo watch is to monitor changes in your project’s source files and automatically execute the specified cargo command.
Whenever you save a file, it will automatically recompile, run, or test the project.

```bash
cargo install cargo-watch
```

After installation:

- Run: cargo watch -x 'run' to start the project.
- Run: APP_HOST=127.0.0.1 APP_PORT=8080 cargo watch -x 'run' to start the project with environment variables.
