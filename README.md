# dou_dizhu

[![Crates.io](https://img.shields.io/crates/v/dou_dizhu)](https://crates.io/crates/dou_dizhu)
[![Documentation](https://img.shields.io/docsrs/dou_dizhu)](https://docs.rs/dou_dizhu)
[![License](https://img.shields.io/crates/l/dou_dizhu)](#license)

This crate is a Rust toolkit for the Chinese card game [Dou Dizhu](https://en.wikipedia.org/wiki/Dou_dizhu).
It provides hand representation, category recognition, ordering, and validation at both compile time and runtime.

## Features

- Hand representation
- All standard Dou Dizhu combinations
- Hand comparison by game rules
- Compile-time and runtime validation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dou_dizhu = "0.1"
```

Then:

```rust
use dou_dizhu::{hand, Hand};

fn main() {
    // Compile-time evaluated hand
    let four_with_dual_solo: Hand = hand!(const {
        King: 4,
        Four,
        Five,
    });

    // Runtime evaluated hand
    let bomb: Hand = hand!({
        Three: {
            println!("evaluating bomb");
            4
        },
    }).unwrap();

    // A bomb beats four with dual solo
    assert!(four_with_dual_solo < bomb);
}
```

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.