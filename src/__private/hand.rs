use crate::{Hand, Rank};

pub struct Spec {
    pub rank: Rank,
    pub count: u8,
    pub texts: SpecTexts,
}

pub struct SpecTexts {
    pub more_than_four_error: &'static str,
    pub duplicate_error: &'static str,
}

pub struct PartialSpec {
    pub rank: Rank,
    pub texts: PartialSpecTexts,
}

pub struct PartialSpecTexts {
    pub duplicate_error: &'static str,
}

pub const fn from_specs<const N: usize>(specs: [Spec; N]) -> Hand {
    let mut counts = [0u8; 15];
    let mut specified = [false; 15];
    {
        let mut i = 0;
        while i < N {
            if specified[specs[i].rank as usize] {
                panic!("{}", specs[i].texts.duplicate_error);
            }
            if specs[i].count > 1 {
                match specs[i].rank {
                    Rank::BlackJoker => panic!("more than one `BlackJoker` is specified"),
                    Rank::RedJoker => panic!("more than one `RedJoker` is specified"),
                    _ => (),
                }
            }
            if specs[i].count > 4 {
                panic!("{}", specs[i].texts.more_than_four_error);
            }
            counts[specs[i].rank as usize] = specs[i].count;
            specified[specs[i].rank as usize] = true;
            i += 1;
        }
    }
    Hand(counts)
}

pub const fn check_partial_specs<const N: usize>(specs: [PartialSpec; N]) {
    let mut specified = [false; 15];
    {
        let mut i = 0;
        while i < N {
            if specified[specs[i].rank as usize] {
                panic!("{}", specs[i].texts.duplicate_error);
            }
            specified[specs[i].rank as usize] = true;
            i += 1;
        }
    }
}
