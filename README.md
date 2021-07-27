# cachewipe
[![Crates.io](https://img.shields.io/crates/v/cachewipe)](https://crates.io/crates/cachewipe) 
[![Docs.rs](https://docs.rs/cachewipe/badge.svg)](https://docs.rs/cachewipe) 
[![Build](https://github.com/Ewpratten/cachewipe/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/cachewipe/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/cachewipe/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/cachewipe/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/cachewipe/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/cachewipe/actions/workflows/audit.yml)


`cachewipe` is a command line utility I originally built to wipe all PYC files from a Python monorepo, while respecting the gitignore (not deleting files that we want to keep). As of now, this tool is fully functional, but does **not** pay attention to gitignore due to some issues im having with the library.

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install cachewipe
```
