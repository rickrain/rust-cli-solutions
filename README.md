# Introduction

This repo contains code I created as I worked through about the first half of the book [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/).

The book, in 2022, was based on version 2 of [clap](https://crates.io/crates/clap). However, at the time I was reading through the book, version 3 of `clap` was already available. So, as I worked through the challenges in the book, I also worked through the challenge of getting my code to work with version 3 of `clap`.

> Note: There is a new version of [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/) that was updated in 2024 that is based on version 4 of `clap`.
>
> There is also a short online Rust book dedicated to building [Command line apps in Rust](https://rust-cli.github.io/book/index.html). 

## Getting Started

Each command-line tool is a separate Rust project in it's own folder. Navigate to a particular CLI tool folder and run `cargo build`, `cargo test`, etc. from there. 

To run a particular CLI tool run `cargo run -- --help` to see what the command-line options are for that tool. Alternatively, you can navigate directly to the `./target/debug` folder and run the executable from there.
