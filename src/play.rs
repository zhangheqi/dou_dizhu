use std::{cmp::Ordering, mem};
use crate::{core::Guard, Hand, Rank};

/// A standard Dou Dizhu play.
/// 
/// For the full specification of standard plays, see the
/// [Pagat rules for Dou Dizhu](https://www.pagat.com/climbing/doudizhu.html).
/// 
/// Many of the methods of `Play` are implemented on [`Guard<Play>`].
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

impl Play {
    /// Returns the category of this play as a [`PlayKind`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// assert_eq!(play!(const { Three: 4 }).unwrap().kind(), Bomb);
    /// ```
    pub const fn kind(&self) -> PlayKind {
        match self {
            Play::Solo(_) => PlayKind::Solo,
            Play::Chain(_) => PlayKind::Chain,
            Play::Pair(_) => PlayKind::Pair,
            Play::PairsChain(_) => PlayKind::PairsChain,
            Play::Trio(_) => PlayKind::Trio,
            Play::Airplane(_) => PlayKind::Airplane,
            Play::TrioWithSolo { .. } => PlayKind::TrioWithSolo,
            Play::AirplaneWithSolos { .. } => PlayKind::AirplaneWithSolos,
            Play::TrioWithPair { .. } => PlayKind::TrioWithPair,
            Play::AirplaneWithPairs { .. } => PlayKind::AirplaneWithPairs,
            Play::Bomb(_) => PlayKind::Bomb,
            Play::FourWithDualSolo { .. } => PlayKind::FourWithDualSolo,
            Play::FourWithDualPair { .. } => PlayKind::FourWithDualPair,
            Play::Rocket => PlayKind::Rocket,
        }
    }
}

impl Guard<Play> {
    /// Converts this play into a [`Hand`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// assert_eq!(
    ///     play!(const { Three: 4 })
    ///         .unwrap()
    ///         .to_hand()
    ///         .len(),
    ///     4,
    /// );
    /// ```
    pub fn to_hand(&self) -> Hand {
        let mut counts = [0u8; 15];
        macro_rules! solitary {
            ($counts:ident, $rank:ident, $count:literal) => {
                $counts[*$rank as usize] = $count
            };
        }
        macro_rules! multiple {
            ($counts:ident, $ranks:ident, $count:literal) => {
                for rank in $ranks {
                    $counts[*rank as usize] = $count;
                }
            };
        }
        match self {
            Guard(Play::Solo(rank)) => solitary!(counts, rank, 1),
            Guard(Play::Chain(ranks)) => multiple!(counts, ranks, 1),
            Guard(Play::Pair(rank)) => solitary!(counts, rank, 2),
            Guard(Play::PairsChain(ranks)) => multiple!(counts, ranks, 2),
            Guard(Play::Trio(rank)) => solitary!(counts, rank, 3),
            Guard(Play::Airplane(ranks)) => multiple!(counts, ranks, 3),
            Guard(Play::TrioWithSolo { trio, solo }) => {
                solitary!(counts, trio, 3);
                solitary!(counts, solo, 1);
            }
            Guard(Play::AirplaneWithSolos { airplane, solos }) => {
                multiple!(counts, airplane, 3);
                multiple!(counts, solos, 1);
            }
            Guard(Play::TrioWithPair { trio, pair }) => {
                solitary!(counts, trio, 3);
                solitary!(counts, pair, 2);
            }
            Guard(Play::AirplaneWithPairs { airplane, pairs }) => {
                multiple!(counts, airplane, 3);
                multiple!(counts, pairs, 2);
            }
            Guard(Play::Bomb(rank)) => solitary!(counts, rank, 4),
            Guard(Play::FourWithDualSolo { four, dual_solo }) => {
                solitary!(counts, four, 4);
                multiple!(counts, dual_solo, 1);
            }
            Guard(Play::FourWithDualPair { four, dual_pair }) => {
                solitary!(counts, four, 4);
                multiple!(counts, dual_pair, 2);
            }
            Guard(Play::Rocket) => {
                counts[Rank::BlackJoker as usize] = 1;
                counts[Rank::RedJoker as usize] = 1;
            }
        }
        Hand(counts)
    }
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

/// Category of a standard Dou Dizhu play.
/// 
/// For the full specification of standard plays, see the
/// [Pagat rules for Dou Dizhu](https://www.pagat.com/climbing/doudizhu.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayKind {
    /// Any single card.
    Solo,
    /// Five or more consecutive individual cards.
    Chain,
    /// Two matching cards of equal rank.
    Pair,
    /// Three or more consecutive pairs.
    PairsChain,
    /// Three-of-a-kind: Three individual cards of the same rank.
    Trio,
    /// Two or more consecutive trios.
    Airplane,
    /// Three cards of the same rank with a solo as the kicker.
    TrioWithSolo,
    /// Two or more consecutive trios with each carries a distinct individual card as the kicker.
    AirplaneWithSolos,
    /// Three cards of the same rank with a pair as the kicker.
    TrioWithPair,
    /// Two or more consecutive trios with each carrying a pair as the kicker.
    AirplaneWithPairs,
    /// Four-of-a-kind. Four cards of the same rank without the kicker is called a bomb, which defies category rules, even beats four with a kicker.
    Bomb,
    /// Four-of-a-kind with two distinct individual cards as the kicker.
    FourWithDualSolo,
    /// Four-of-a-kind with two sets of pair as the kicker.
    FourWithDualPair,
    /// Red Joker and Black Joker.
    Rocket,
}

impl PartialOrd for PlayKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }
        let self_level = match self {
            Self::Bomb => 1,
            Self::Rocket => 2,
            _ => 0,
        };
        let other_level = match other {
            Self::Bomb => 1,
            Self::Rocket => 2,
            _ => 0,
        };
        match self_level.cmp(&other_level) {
            Ordering::Equal => None,
            ord => Some(ord),
        }
    }
}
