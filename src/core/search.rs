//! Play search utilities.
//! 
//! This module provides functionality for enumerating possible plays
//! within a [`Hand`]. These plays are not necessarily standard ones.

use std::ops::{RangeBounds, RangeInclusive};
use crate::{Hand, PlayKind};

/// Specification for searching for plays in a hand.
/// Can be used to search for even non-standard plays.
/// 
/// Searching for `Rocket` is unsupported.
pub struct PlaySpec<R, F>
where
    R: RangeBounds<u8>,
    F: FnMut(u8) -> u8,
{
    /// Number of cards in each primal element. Examples:
    /// 
    /// - `1` for `Solo`, `Chain`,
    /// - `2` for `Pair`, `PairsChain`,
    /// - `3` for `Trio`, `Airplane`, `TrioWithSolo`, `AirplaneWithSolos`,
    ///   `TrioWithPair`, `AirplaneWithPairs`,
    /// - `4` for `Bomb`, `FourWithDualSolo`, `FourWithDualPair`.
    pub primal_size: u8,

    /// Range of the number of primal elements.
    /// 
    /// - For chain-like plays, the number of primal elements equals the chain length.
    /// - For other standard plays, the number of primal elements is always one.
    /// 
    /// Note that primal elements are always consecutive.
    pub primal_count: R,

    /// Number of cards in each kicker element. Examples:
    /// 
    /// - `0` for `Solo`, `Chain`, `Pair`, `PairsChain`, `Trio`, `Airplane`, `Bomb`,
    /// - `1` for `TrioWithSolo`, `AirplaneWithSolos`, `FourWithDualSolo`,
    /// - `2` for `TrioWithPair`, `AirplaneWithPairs`, `FourWithDualPair`.
    pub kicker_size: u8,

    /// Closure called to compute the number of kicker elements.
    /// 
    /// The closure takes the number of primal elements (`u8`) and returns
    /// the number of kicker elements (`u8`).
    pub kicker_count: F,
}

impl PlaySpec<RangeInclusive<u8>, fn(u8) -> u8> {
    pub const fn standard(kind: PlayKind) -> Self {
        match kind {
            PlayKind::Solo => Self { primal_size: 1, primal_count: 1..=1, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::Chain => Self { primal_size: 1, primal_count: 5..=12, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::Pair => Self { primal_size: 2, primal_count: 1..=1, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::PairsChain => Self { primal_size: 2, primal_count: 3..=12, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::Trio => Self { primal_size: 3, primal_count: 1..=1, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::Airplane => Self { primal_size: 3, primal_count: 2..=12, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::TrioWithSolo => Self { primal_size: 3, primal_count: 1..=1, kicker_size: 1, kicker_count: |_| 1 },
            PlayKind::AirplaneWithSolos => Self { primal_size: 3, primal_count: 2..=7, kicker_size: 1, kicker_count: |x| x },
            PlayKind::TrioWithPair => Self { primal_size: 3, primal_count: 1..=1, kicker_size: 2, kicker_count: |_| 1 },
            PlayKind::AirplaneWithPairs => Self { primal_size: 3, primal_count: 2..=7, kicker_size: 2, kicker_count: |x| x },
            PlayKind::Bomb => Self { primal_size: 4, primal_count: 1..=1, kicker_size: 0, kicker_count: |_| 0 },
            PlayKind::FourWithDualSolo => Self { primal_size: 4, primal_count: 1..=1, kicker_size: 1, kicker_count: |_| 2 },
            PlayKind::FourWithDualPair => Self { primal_size: 4, primal_count: 1..=1, kicker_size: 2, kicker_count: |_| 2 },
            PlayKind::Rocket => panic!("`Rocket` cannot be expressed as a `PlaySpec`"),
        }
    }
}

/// Extension trait for searching for possible plays within a [`Hand`].
/// 
/// This trait is sealed and cannot be implemented for types outside of `dou_dizhu`.
pub trait SearchExt: private::Sealed {
    fn plays<R, F>(self, spec: PlaySpec<R, F>) -> impl Iterator<Item = Hand>
    where
        R: RangeBounds<u8>,
        F: FnMut(u8) -> u8;
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::hand::Hand {}
}
