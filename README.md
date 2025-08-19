# dou_dizhu

[![Crates.io](https://img.shields.io/crates/v/dou_dizhu)](https://crates.io/crates/dou_dizhu)
[![Documentation](https://img.shields.io/docsrs/dou_dizhu)](https://docs.rs/dou_dizhu)
[![License](https://img.shields.io/crates/l/dou_dizhu)](#license)

A Rust toolkit for the Chinese card game Dou Dizhu (斗地主).

This crate implements Dou Dizhu strictly following the [Pagat rules](https://www.pagat.com/climbing/doudizhu.html), though it uses different terminology.

## Features

- Hand representation
- Compile-time and runtime validation
- All standard Dou Dizhu play kinds (solo, pair, trio, chain, airplane, rocket, etc.)
- Play kind recognition
- Play comparison
- Enumerating all plays of specified characteristics within a hand

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dou_dizhu = "0.2"
```

Then:

```rust
use dou_dizhu::*;

fn main() {
    // Construct a play
    let airplane_with_solos = play!(const {
        Queen: 3,
        King: 3,
        Three,
        Four,
    })
    .unwrap();

    // Count how many stronger plays of the same kind exist in a full deck
    assert_eq!(
        Hand::full_deck()
            .plays(airplane_with_solos.kind())
            .filter(|p| p > &airplane_with_solos)
            .count(),
        77,
    );
}
```

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.