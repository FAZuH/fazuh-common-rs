# fazuh-common

Personal collection of common Rust code for my projects.

## Features

This crate is heavily feature-gated to ensure downstream projects only compile what they need:

- `log`: Core tracing configuration and initialization logic.
- `log-file`: Adds support for rolling daily file appenders using `tracing-appender`.
- `log-cli`: Adds CLI-aware ANSI color parsing with `clap`.
- `types`: Common utility types (e.g., `Percent`).

## Examples

Check code examples at [examples/](examples/)

Run the included examples to see features in action:

```sh
cargo run --example logging --features "log,log-file,log-cli"
```
