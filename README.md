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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dou_dizhu = "0.2"
```

Then:

```rust
use dou_dizhu::{Hand, Play, Rank, core::Guard, hand};

fn main() {
    // Hand created during compilation, recognized as a play at runtime
    let p1: Guard<Play> = hand!(const {
        King: 4,
        BlackJoker,
        Ace,
    })
    .to_play()
    .unwrap();

    // Hand created from an explicit counts array, recognized as a play
    let p2: Guard<Play> = Hand::try_from({
        let mut counts = [0u8; 15];
        counts[Rank::Three as usize] = 4;
        counts
    })
    .unwrap()
    .to_play()
    .unwrap();

    // A bomb beats four with dual solo
    assert!(p1 < p2);
    assert!(matches!(p1.into_inner(), Play::FourWithDualSolo { .. }));
    assert!(matches!(p2.into_inner(), Play::Bomb(_)));
}
```

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.