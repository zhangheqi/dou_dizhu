use std::{cmp::Ordering, mem, ops::Deref};

pub mod __private;

#[macro_export]
macro_rules! hand {
    (const {$($t:tt)*}) => {
        $crate::__const_hand!(($($t)*) -> ())
    };
    ({$($t:tt)*}) => {
        $crate::__hand!(($($t)*) -> ()()(var))
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Hand([u8; 15]);

impl TryFrom<[u8; 15]> for Hand {
    type Error = String;

    fn try_from(counts: [u8; 15]) -> Result<Self, Self::Error> {
        for i in 0u8..13 {
            if counts[i as usize] > 4 {
                return Err(format!("more than four `{:?}`s are specified", unsafe { mem::transmute::<_, Rank>(i) }));
            }
        }
        for i in 13u8..15 {
            if counts[i as usize] > 1 {
                return Err(format!("more than one `{:?}` is specified", unsafe { mem::transmute::<_, Rank>(i) }));
            }
        }
        Ok(Hand(counts))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(|x| x.is_eq())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Some(self_category) = self.analyze().category() else {
            return None;
        };
        let Some(other_category) = other.analyze().category() else {
            return None;
        };
        self_category.partial_cmp(&other_category)
    }
}

impl Hand {
    pub fn to_array(self) -> [u8; 15] {
        self.0
    }

    pub fn analyze(self) -> Guard<HandAnalysis> {
        let Hand(counts) = self;
        let mut analysis = HandAnalysis {
            solos: HandAnalysisItem { ranks: Vec::new(), consecutive: true },
            pairs: HandAnalysisItem { ranks: Vec::new(), consecutive: true },
            trios: HandAnalysisItem { ranks: Vec::new(), consecutive: true },
            fours: HandAnalysisItem { ranks: Vec::new(), consecutive: true },
        };
        macro_rules! update_analysis {
            ($field:expr, $i:ident) => {
                {
                    if $field.consecutive {
                        if $i >= Rank::Two as u8 {
                            $field.consecutive = false;
                        } else if let Some(&rank) = $field.ranks.last() && $i - rank as u8 != 1 {
                            $field.consecutive = false;
                        }
                    }
                    $field.ranks.push(unsafe { mem::transmute($i) });
                }
            };
        }
        for i in 0u8..15 {
            match counts[i as usize] {
                0 => (),
                1 => update_analysis!(analysis.solos, i),
                2 => update_analysis!(analysis.pairs, i),
                3 => update_analysis!(analysis.trios, i),
                4 => update_analysis!(analysis.fours, i),
                _ => unreachable!(),
            }
        }
        Guard(analysis)
    }
}

#[derive(Debug)]
pub struct HandAnalysisItem {
    pub ranks: Vec<Rank>,
    pub consecutive: bool,
}

#[derive(Debug)]
pub struct HandAnalysis {
    pub solos: HandAnalysisItem,
    pub pairs: HandAnalysisItem,
    pub trios: HandAnalysisItem,
    pub fours: HandAnalysisItem,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Landlord,
    Peasant,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
    BlackJoker,
    RedJoker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
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

#[derive(Debug)]
pub struct Guard<T>(T);

impl<T> Guard<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}

impl Guard<HandAnalysis> {
    pub fn category(&self) -> Option<Guard<Category>> {
        macro_rules! test_categories {
            ($self:ident, $($method:ident),*) => {
                let mut result;
                $(
                    result = $self.$method();
                    if result.is_some() {
                        return result;
                    }
                )*
            };
        }
        test_categories!(
            self,
            solo, chain, pair, pairs_chain, trio, airplane,
            trio_with_solo, airplane_with_solos,
            trio_with_pair, airplane_with_pairs,
            bomb, four_with_dual_solo, four_with_dual_pair,
            rocket
        );
        None
    }

    pub fn solo(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() == 1
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Solo(self.solos.ranks[0])))
        } else {
            None
        }
    }

    pub fn chain(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() >= 5
            && self.solos.consecutive
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Chain(self.solos.ranks.clone())))
        } else {
            None
        }
    }

    pub fn pair(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 1
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Pair(self.pairs.ranks[0])))
        } else {
            None
        }
    }

    pub fn pairs_chain(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() >= 3
            && self.pairs.consecutive
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::PairsChain(self.pairs.ranks.clone())))
        } else {
            None
        }
    }

    pub fn trio(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Trio(self.trios.ranks[0])))
        } else {
            None
        }
    }

    pub fn airplane(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() >= 2
            && self.trios.consecutive
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Airplane(self.trios.ranks.clone())))
        } else {
            None
        }
    }

    pub fn trio_with_solo(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() == 1
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::TrioWithSolo {
                trio: self.trios.ranks[0],
                solo: self.solos.ranks[0],
            }))
        } else {
            None
        }
    }

    pub fn airplane_with_solos(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() == self.trios.ranks.len()
            && self.solos.ranks.len() >= 2
            // make sure rocket not in kicker cards
            && !(
                self.solos.ranks[self.solos.ranks.len() - 1] == Rank::RedJoker
                && self.solos.ranks[self.solos.ranks.len() - 2] == Rank::BlackJoker
            )
            && self.pairs.ranks.is_empty()
            && self.trios.consecutive
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::AirplaneWithSolos {
                airplane: self.trios.ranks.clone(),
                solos: self.solos.ranks.clone(),
            }))
        } else {
            None
        }
    }

    pub fn trio_with_pair(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 1
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::TrioWithPair {
                trio: self.trios.ranks[0],
                pair: self.pairs.ranks[0],
            }))
        } else {
            None
        }
    }

    pub fn airplane_with_pairs(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == self.trios.ranks.len()
            && self.trios.ranks.len() >= 2
            && self.trios.consecutive
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::AirplaneWithPairs {
                airplane: self.trios.ranks.clone(),
                pairs: self.pairs.ranks.clone(),
            }))
        } else {
            None
        }
    }

    pub fn bomb(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Category::Bomb(self.fours.ranks[0])))
        } else {
            None
        }
    }

    pub fn four_with_dual_solo(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() == 2
            && self.solos.ranks[0] != Rank::BlackJoker // make sure rocket != kicker cards
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Category::FourWithDualSolo {
                four: self.fours.ranks[0],
                dual_solo: [
                    self.solos.ranks[0],
                    self.solos.ranks[1],
                ],
            }))
        } else {
            None
        }
    }

    pub fn four_with_dual_pair(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 2
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Category::FourWithDualPair {
                four: self.fours.ranks[0],
                dual_pair: [
                    self.pairs.ranks[0],
                    self.pairs.ranks[1],
                ],
            }))
        } else {
            None
        }
    }

    pub fn rocket(&self) -> Option<Guard<Category>> {
        if self.solos.ranks.len() == 2
            && self.solos.ranks[0] == Rank::BlackJoker
            && self.solos.ranks[1] == Rank::RedJoker
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Category::Rocket))
        } else {
            None
        }
    }
}

// NOTE: We intentionally do NOT implement `DerefMut`.
// Doing so would allow external code to modify the wrapped value,
// which violates our guarantee.
impl<T> Deref for Guard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Guard<Category> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(|x| x.is_eq())
    }
}

impl PartialOrd for Guard<Category> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if mem::discriminant(&self.0) != mem::discriminant(&other.0) {
            let self_level = match self.0 {
                Category::Bomb(_) => 1,
                Category::Rocket => 2,
                _ => 0,
            };
            let other_level = match other.0 {
                Category::Bomb(_) => 1,
                Category::Rocket => 2,
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
                    Category::Rocket => Some(Ordering::Equal),
                }
            };
            (($self_: ident, $other: ident)($variant:ident { $field:tt: _, .. } => _, $($t:tt)*) -> ($($body:tt)*)) => {
                generate_match_helper!(($self_, $other)($($t)*) -> (
                    $($body)*
                    Category::$variant { $field: self_rank, .. } => {
                        let Category::$variant { $field: other_rank, .. } = $other.0 else { unreachable!() };
                        self_rank.partial_cmp(&other_rank)
                    }
                ))
            };
            (($self_: ident, $other: ident)($variant:ident { $field:tt: ref _, .. } => _, $($t:tt)*) -> ($($body:tt)*)) => {
                generate_match_helper!(($self_, $other)($($t)*) -> (
                    $($body)*
                    Category::$variant { $field: ref self_ranks, .. } => {
                        let Category::$variant { $field: ref other_ranks, .. } = $other.0 else { unreachable!() };
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
