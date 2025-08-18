use std::{mem, ops::{Bound, RangeBounds}};
use itertools::Itertools;
use crate::{core::{Composition, CompositionExt, Group, Guard, PlaySpec, SearchExt}, Play, Rank};

#[derive(Debug, Clone, Copy)]
pub struct Hand(pub(crate) [u8; 15]);

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

impl TryFrom<&[u8]> for Hand {
    type Error = String;

    fn try_from(counts: &[u8]) -> Result<Self, Self::Error> {
        if counts.len() != 15 {
            return Err(format!("invalid slice length: expected 15, got {}", counts.len()));
        }
        <Hand as TryFrom<[u8; 15]>>::try_from(counts.try_into().unwrap())
    }
}

impl Hand {
    pub const fn full_deck() -> Self {
        let mut counts = [4u8; 15];
        counts[Rank::BlackJoker as usize] = 1;
        counts[Rank::RedJoker as usize] = 1;
        Self(counts)
    }

    pub fn to_array(self) -> [u8; 15] {
        self.0
    }

    pub fn to_play(self) -> Option<Guard<Play>> {
        self.composition().to_play()
    }
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

impl SearchExt for Hand {
    fn plays<R, F>(self, spec: PlaySpec<R, F>) -> impl Iterator<Item = Hand>
    where
        R: RangeBounds<u8>,
        F: Fn(u8) -> u8,
    {
        let primal_count_min = match spec.primal_count.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 1,
        }
        .max(1);

        let primal_count_max = match spec.primal_count.end_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n - 1,
            Bound::Unbounded => 12,
        }
        .min(12);

        (primal_count_min..=primal_count_max).flat_map(move |primal_count| {
            let kicker_count = (spec.kicker_count)(primal_count);
            self.0
                .into_iter()
                .zip(0u8..15)
                .filter(|&(count, rank)| count >= spec.primal_size && (rank < Rank::Two as u8 || primal_count == 1))
                .map(|(_, rank)| unsafe { mem::transmute(rank) })
                .collect::<Vec<Rank>>()
                .chunk_by(|&a, &b| a as u8 + 1 == b as u8)
                .map(Vec::from)
                .collect::<Vec<_>>()
                .into_iter()
                .flat_map(move |chunk| {
                    chunk
                        .windows(primal_count as usize)
                        .map(Vec::from)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .flat_map(move |primal| {
                            let mut jokers = Vec::new();
                            let kicker_candidates = if kicker_count != 0 {
                                self.0
                                    .into_iter()
                                    .zip(0u8..15)
                                    .map(|(count, rank)| (count, unsafe { mem::transmute(rank) }))
                                    .filter(|&(count, rank)| {
                                        if count >= spec.kicker_size && !primal.contains(&rank) {
                                            if rank > Rank::Two {
                                                jokers.push(rank);
                                                false
                                            } else {
                                                true
                                            }
                                        } else {
                                            false
                                        }
                                    })
                                    .map(|(_, rank)| rank)
                                    .collect::<Vec<Rank>>()
                            } else {
                                Vec::new()
                            };
                            kicker_candidates
                                .clone()
                                .into_iter()
                                .combinations(kicker_count as usize)
                                .chain(
                                    jokers
                                        .into_iter()
                                        .flat_map(move |joker| {
                                            kicker_candidates
                                                .clone()
                                                .into_iter()
                                                .combinations(kicker_count as usize - 1)
                                                .map(move |mut kicker| {
                                                    kicker.push(joker);
                                                    kicker
                                                })
                                        })
                                )
                                .map(move |kicker| {
                                    let mut counts = [0u8; 15];
                                    for rank in primal.clone() {
                                        counts[rank as usize] = spec.primal_size;
                                    }
                                    for rank in kicker {
                                        counts[rank as usize] = spec.kicker_size;
                                    }
                                    Hand(counts)
                                })
                        })
                })
        })
    }
}
