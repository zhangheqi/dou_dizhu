use std::{cmp::Ordering, mem};
use crate::{core::Guard, Rank};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Play {
    /// Any single card.
    Solo(Rank),
    /// Five or more consecutive individual cards.
    Chain(Vec<Rank>),
    /// Two matching cards of equal rank.
    Pair(Rank),
    /// Three or more consecutive pairs.
    PairsChain(Vec<Rank>),
    /// Three-of-a-kind: Three individual cards of the same rank.
    Trio(Rank),
    /// Two or more consecutive trios.
    Airplane(Vec<Rank>),
    /// Three cards of the same rank with a solo as the kicker.
    TrioWithSolo { trio: Rank, solo: Rank },
    /// Two or more consecutive trios with each carries a distinct individual card as the kicker.
    AirplaneWithSolos {
        airplane: Vec<Rank>,
        solos: Vec<Rank>,
    },
    /// Three cards of the same rank with a pair as the kicker.
    TrioWithPair { trio: Rank, pair: Rank },
    /// Two or more consecutive trios with each carrying a pair as the kicker.
    AirplaneWithPairs {
        airplane: Vec<Rank>,
        pairs: Vec<Rank>,
    },
    /// Four-of-a-kind. Four cards of the same rank without the kicker is called a bomb, which defies category rules, even beats four with a kicker.
    Bomb(Rank),
    /// Four-of-a-kind with two distinct individual cards as the kicker.
    FourWithDualSolo { four: Rank, dual_solo: [Rank; 2] },
    /// Four-of-a-kind with two sets of pair as the kicker.
    FourWithDualPair { four: Rank, dual_pair: [Rank; 2] },
    /// Red Joker and Black Joker.
    Rocket,
}

impl PartialEq for Guard<Play> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(|x| x.is_eq())
    }
}

impl PartialOrd for Guard<Play> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if mem::discriminant(&self.0) != mem::discriminant(&other.0) {
            let self_level = match self.0 {
                Play::Bomb(_) => 1,
                Play::Rocket => 2,
                _ => 0,
            };
            let other_level = match other.0 {
                Play::Bomb(_) => 1,
                Play::Rocket => 2,
                _ => 0,
            };
            if self_level == other_level { // they are implicitly both zero
                return None;
            }
            return self_level.partial_cmp(&other_level);
        }
        macro_rules! generate_match {
            (
                match ($self_: ident, $other: ident) {
                    $($t:tt)*
                }
            ) => {
                generate_match_helper!(($self_, $other)($($t)*) -> ())
            };
        }
        macro_rules! generate_match_helper {
            (($self_: ident, $other: ident)() -> ($($body:tt)*)) => {
                match $self_.0 {
                    $($body)*
                    Play::Rocket => Some(Ordering::Equal),
                }
            };
            (($self_: ident, $other: ident)($variant:ident { $field:tt: _, .. } => _, $($t:tt)*) -> ($($body:tt)*)) => {
                generate_match_helper!(($self_, $other)($($t)*) -> (
                    $($body)*
                    Play::$variant { $field: self_rank, .. } => {
                        let Play::$variant { $field: other_rank, .. } = $other.0 else { unreachable!() };
                        self_rank.partial_cmp(&other_rank)
                    }
                ))
            };
            (($self_: ident, $other: ident)($variant:ident { $field:tt: ref _, .. } => _, $($t:tt)*) -> ($($body:tt)*)) => {
                generate_match_helper!(($self_, $other)($($t)*) -> (
                    $($body)*
                    Play::$variant { $field: ref self_ranks, .. } => {
                        let Play::$variant { $field: ref other_ranks, .. } = $other.0 else { unreachable!() };
                        if self_ranks.len() == other_ranks.len() {
                            self_ranks[0].partial_cmp(&other_ranks[0])
                        } else {
                            None
                        }
                    }
                ))
            };
        }
        generate_match!(
            match (self, other) {
                Solo { 0: _, .. } => _,
                Chain { 0: ref _, .. } => _,
                Pair { 0: _, .. } => _,
                PairsChain { 0: ref _, .. } => _,
                Trio { 0: _, .. } => _,
                Airplane { 0: ref _, .. } => _,
                TrioWithSolo { trio: _, .. } => _,
                AirplaneWithSolos { airplane: ref _, .. } => _,
                TrioWithPair { trio: _, .. } => _,
                AirplaneWithPairs { airplane: ref _, .. } => _,
                Bomb { 0: _, .. } => _,
                FourWithDualSolo { four: _, .. } => _,
                FourWithDualPair { four: _, .. } => _,
            }
        )
    }
}
