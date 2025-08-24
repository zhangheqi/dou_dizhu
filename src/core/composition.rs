//! Structural analysis of hands.
//! 
//! This module provides tools for breaking down a [`Hand`]
//! into its raw structural components.

use std::mem;
use crate::{core::Guard, Hand, Play, PlayKind, Rank};

/// A group of ranks that all appear with the same multiplicity (1, 2, 3, or 4)
/// and whether they form a consecutive run.
#[derive(Debug)]
pub struct Group {
    pub ranks: Vec<Rank>,
    pub consecutive: bool,
}

/// The structural breakdown of a hand into singles, pairs, trios, and fours
/// (each with run information).
/// 
/// All methods of `Composition` are implemented on [`Guard<Composition>`].
/// 
/// # Examples
/// 
/// ```
/// use dou_dizhu::{*, core::CompositionExt};
/// 
/// let comp = hand!(const { Three }).composition();
/// assert_eq!(comp.solos.ranks, vec![Rank::Three]);
/// assert!(comp.solos.consecutive);
/// ```
#[derive(Debug)]
pub struct Composition {
    pub solos: Group,
    pub pairs: Group,
    pub trios: Group,
    pub fours: Group,
}

impl Guard<Composition> {
    /// Try to infer the play represented by this composition,
    /// or `None` if it matches no standard pattern.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::{*, core::CompositionExt};
    /// 
    /// let comp = hand!(const { Three: 4 }).composition();
    /// let guess = comp.guess_play();
    /// assert!(guess.is_some());
    /// assert!(matches!(*guess.unwrap(), Play::Bomb(Rank::Three)));
    /// ```
    pub fn guess_play(&self) -> Option<Guard<Play>> {
        macro_rules! try_methods {
            ($self_:ident $($method:ident)*) => {
                let mut result;
                $(
                    result = $self_.$method();
                    if result.is_some() {
                        return result;
                    }
                )*
            };
        }
        try_methods! {
            self
            to_solo to_chain
            to_pair to_pairs_chain
            to_trio to_airplane
            to_trio_with_solo to_airplane_with_solos
            to_trio_with_pair to_airplane_with_pairs
            to_bomb
            to_four_with_dual_solo
            to_four_with_dual_pair
            to_rocket
        }
        None
    }

    /// Attempt to convert this composition into the requested play kind,
    /// returning `None` if the structure doesn't match.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::{*, core::CompositionExt};
    /// 
    /// let comp = hand!(const { Three: 4 }).composition();
    /// let play = comp.to_play(Bomb);
    /// assert!(play.is_some());
    /// assert!(matches!(*play.unwrap(), Play::Bomb(Rank::Three)));
    /// ```
    pub fn to_play(&self, kind: PlayKind) -> Option<Guard<Play>> {
        match kind {
            PlayKind::Solo => self.to_solo(),
            PlayKind::Chain => self.to_chain(),
            PlayKind::Pair => self.to_pair(),
            PlayKind::PairsChain => self.to_pairs_chain(),
            PlayKind::Trio => self.to_trio(),
            PlayKind::Airplane => self.to_airplane(),
            PlayKind::TrioWithSolo => self.to_trio_with_solo(),
            PlayKind::AirplaneWithSolos => self.to_airplane_with_solos(),
            PlayKind::TrioWithPair => self.to_trio_with_pair(),
            PlayKind::AirplaneWithPairs => self.to_airplane_with_pairs(),
            PlayKind::Bomb => self.to_bomb(),
            PlayKind::FourWithDualSolo => self.to_four_with_dual_solo(),
            PlayKind::FourWithDualPair => self.to_four_with_dual_pair(),
            PlayKind::Rocket => self.to_rocket(),
        }
    }

    /// Return a Solo play if and only if the composition is exactly one single rank.
    pub fn to_solo(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.len() == 1
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Solo(self.solos.ranks[0])))
        } else {
            None
        }
    }

    /// Return a Chain (solo straight, length >= 5) if and only if only consecutive singles are present.
    pub fn to_chain(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.len() >= 5
            && self.solos.consecutive
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Chain(self.solos.ranks.clone())))
        } else {
            None
        }
    }

    /// Return a Pair if and only if the composition is exactly one pair.
    pub fn to_pair(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 1
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Pair(self.pairs.ranks[0])))
        } else {
            None
        }
    }

    /// Return a PairsChain (pair straight, length >= 3) if and only if only consecutive pairs are present.
    pub fn to_pairs_chain(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() >= 3
            && self.pairs.consecutive
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::PairsChain(self.pairs.ranks.clone())))
        } else {
            None
        }
    }

    /// Return a Trio if and only if the composition is exactly one triple.
    pub fn to_trio(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Trio(self.trios.ranks[0])))
        } else {
            None
        }
    }

    /// Return an Airplane (consecutive trios, length >= 2) if and only if only consecutive trios are present.
    pub fn to_airplane(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() >= 2
            && self.trios.consecutive
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Airplane(self.trios.ranks.clone())))
        } else {
            None
        }
    }

    /// Return a TrioWithSolo if and only if there is exactly one trio and one single kicker.
    pub fn to_trio_with_solo(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.len() == 1
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::TrioWithSolo {
                trio: self.trios.ranks[0],
                solo: self.solos.ranks[0],
            }))
        } else {
            None
        }
    }

    /// Return an AirplaneWithSolos if and only if there are consecutive trios with the same number of single kickers (not a rocket).
    pub fn to_airplane_with_solos(&self) -> Option<Guard<Play>> {
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
            Some(Guard(Play::AirplaneWithSolos {
                airplane: self.trios.ranks.clone(),
                solos: self.solos.ranks.clone(),
            }))
        } else {
            None
        }
    }

    /// Return a TrioWithPair if and only if there is exactly one trio and one pair kicker.
    pub fn to_trio_with_pair(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 1
            && self.trios.ranks.len() == 1
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::TrioWithPair {
                trio: self.trios.ranks[0],
                pair: self.pairs.ranks[0],
            }))
        } else {
            None
        }
    }

    /// Return an AirplaneWithPairs if and only if there are consecutive trios with an equal number of pair kickers.
    pub fn to_airplane_with_pairs(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == self.trios.ranks.len()
            && self.trios.ranks.len() >= 2
            && self.trios.consecutive
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::AirplaneWithPairs {
                airplane: self.trios.ranks.clone(),
                pairs: self.pairs.ranks.clone(),
            }))
        } else {
            None
        }
    }

    /// Return a Bomb if and only if the composition is exactly one four-of-a-kind.
    pub fn to_bomb(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Play::Bomb(self.fours.ranks[0])))
        } else {
            None
        }
    }

    /// Return FourWithDualSolo if and only if there is exactly one four-of-a-kind and two non-rocket singles.
    pub fn to_four_with_dual_solo(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.len() == 2
            && self.solos.ranks[0] != Rank::BlackJoker // make sure rocket != kicker cards
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Play::FourWithDualSolo {
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

    /// Return FourWithDualPair if and only if there is exactly one four-of-a-kind and two pairs.
    pub fn to_four_with_dual_pair(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.is_empty()
            && self.pairs.ranks.len() == 2
            && self.trios.ranks.is_empty()
            && self.fours.ranks.len() == 1
        {
            Some(Guard(Play::FourWithDualPair {
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

    /// Return a Rocket if and only if the composition is exactly the two jokers.
    pub fn to_rocket(&self) -> Option<Guard<Play>> {
        if self.solos.ranks.len() == 2
            && self.solos.ranks[0] == Rank::BlackJoker
            && self.solos.ranks[1] == Rank::RedJoker
            && self.pairs.ranks.is_empty()
            && self.trios.ranks.is_empty()
            && self.fours.ranks.is_empty()
        {
            Some(Guard(Play::Rocket))
        } else {
            None
        }
    }
}

/// Extension trait for converting a type into a [`Composition`].
/// 
/// This trait is sealed and cannot be implemented for types outside of `dou_dizhu`.
pub trait CompositionExt: private::Sealed {
    /// Compute the structural [`Composition`] of this hand.
    fn composition(self) -> Guard<Composition>;
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Hand {}
}

impl CompositionExt for Hand {
    fn composition(self) -> Guard<Composition> {
        let Hand(counts) = self;
        let mut comp = Composition {
            solos: Group { ranks: Vec::new(), consecutive: true },
            pairs: Group { ranks: Vec::new(), consecutive: true },
            trios: Group { ranks: Vec::new(), consecutive: true },
            fours: Group { ranks: Vec::new(), consecutive: true },
        };
        macro_rules! update_group {
            ($group:expr, $index:ident) => {
                {
                    if $group.consecutive {
                        if $index >= Rank::Two as u8 {
                            $group.consecutive = false;
                        } else if let Some(&rank) = $group.ranks.last() && $index - rank as u8 != 1 {
                            $group.consecutive = false;
                        }
                    }
                    $group.ranks.push(unsafe { mem::transmute($index) });
                }
            };
        }
        for i in 0u8..15 {
            match counts[i as usize] {
                0 => (),
                1 => update_group!(comp.solos, i),
                2 => update_group!(comp.pairs, i),
                3 => update_group!(comp.trios, i),
                4 => update_group!(comp.fours, i),
                _ => unreachable!(),
            }
        }
        Guard(comp)
    }
}
