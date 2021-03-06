![Github CI](https://github.com/lasagnamassage/file_alphabetizer/actions/workflows/rust.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/file_alphabetizer.svg)](https://crates.io/crates/file_alphabetizer)
[![API](https://docs.rs/file_alphabetizer/badge.svg)](https://docs.rs/file_alphabetizer)

# file_alphabetizer
A function that takes a file and alphabetizes it.

## Usage

**Within a Rust project**
1. Import the library into your Cargo.toml like so:
    ```Rust
    [dependencies] 
        file_alphabetizer = "2.1.3"
    ```
2. Import the crate and use the alphabetize method, like so.
    (Currently, path starts at parent directory of code calling it):
    ```rust
        use std::{path::PathBuf};
        use file_alphabetizer;

        fn main() {
            let path = PathBuf::from("src/test");
            file_alphabetizer::alphabetize(&path).unwrap();
        }
    ```

## Crate Features

file_alphabetizer sorts non-case-sensitive alphabetical order.
- Special characters such as brackets are placed after the alphabet
- Numbers are placed before alphabet

# License

file_alphabetizer is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.