use std::mem;
use crate::{core::{Composition, CompositionExt, Group, Guard}, Play, Rank};

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
