[![Rust](https://github.com/rickrain/rust-cli-solutions/actions/workflows/ci.yaml/badge.svg)](https://github.com/rickrain/rust-cli-solutions/actions/workflows/ci.yaml)

# Introduction

This repo contains code I created as I worked through the first third-to-half of the book [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/).

The book, in 2022, was based on version 2 of [clap](https://crates.io/crates/clap). However, at the time I was reading the book, version 3 of `clap` was already available. So, as I worked through the challenges in the book, I also worked through the challenge of getting my code to work with version 3 of `clap`.

> Note: There is a new version of [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/) that was updated in 2024 that is based on version 4 of `clap`.
>
> There is also a short online Rust book dedicated to building [Command line apps in Rust](https://rust-cli.github.io/book/index.html). 

## Getting Started

Each command-line tool is a separate Rust project in it's own folder. The `Cargo.toml` in the root folder establishes a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) that includes all the child project folders.

To build the projects, run `cargo build` from the root folder. Similarly, to run the tests for each project, run `cargo test` from the root folder. Finally, you can execute a particular CLI tool in the repo to try it out. For example, run `cargo run -p echor -- --help` to run the `echor` project and get help on it's command-line parameters.

> Note: If you want to experiment with a particular CLI tool without building and testing all the projects, you can navigate to its folder and run `cargo build`, `cargo test`, etc. from there.