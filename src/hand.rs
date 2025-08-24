use std::{iter, mem, ops::Index};
use crate::{core::{CompositionExt, Guard, PlaySpec, SearchExt}, Play, PlayKind, Rank};

/// Representation of a Dou Dizhu hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// A complete Dou Dizhu deck.
    pub const FULL_DECK: Self = Self([4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1]);

    /// An empty hand.
    pub const EMPTY: Self = Self([0; 15]);

    /// Returns the internal representation of this hand as an array of card counts.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// let bomb = hand!(const {
    ///     Ten: 4,
    /// });
    /// 
    /// assert_eq!(bomb.to_array()[Rank::Ten as usize], 4);
    /// ```
    pub const fn to_array(self) -> [u8; 15] {
        self.0
    }

    /// Attempts to recognize this `Hand` as a standard [`Play`].
    /// 
    /// Returns `None` if the hand does not form a standard play.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// let rocket = hand!(const { BlackJoker, RedJoker })
    ///     .to_play()
    ///     .unwrap();
    /// 
    /// assert!(matches!(rocket.into_inner(), Play::Rocket));
    /// ```
    pub fn to_play(self) -> Option<Guard<Play>> {
        self.composition().guess_play()
    }

    /// Returns an iterator over all standard plays of the given kind available in this hand.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// assert_eq!(
    ///     Hand::FULL_DECK.plays(AirplaneWithSolos).count(),
    ///     7516,
    /// )
    /// ```
    pub fn plays(self, kind: PlayKind) -> impl Iterator<Item = Guard<Play>> {
        match kind {
            PlayKind::Rocket => {
                if self.0[Rank::BlackJoker as usize] == 1
                    && self.0[Rank::RedJoker as usize] == 1
                {
                    Box::new([Guard(Play::Rocket)].into_iter())
                } else {
                    Box::new(iter::empty()) as Box<dyn Iterator<Item = Guard<Play>>>
                }
            }
            kind => Box::new(
                SearchExt::plays(self, PlaySpec::standard(kind))
                    .map(move |x| x.composition().to_play(kind).unwrap()),
            ),
        }
    }

    /// Returns the total number of cards in this hand.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// assert_eq!(
    ///     Hand::FULL_DECK.len(),
    ///     54,
    /// )
    /// ```
    pub const fn len(&self) -> usize {
        let mut sum = 0;
        {
            let mut i = 0;
            while i < 15 {
                sum += self.0[i] as usize;
                i += 1;
            }
        }
        sum
    }

    /// Returns `true` if the hand contains no cards.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::*;
    /// 
    /// assert!(Hand::EMPTY.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        {
            let mut i = 0;
            while i < 15 {
                if self.0[i] != 0 {
                    return false;
                }
                i += 1;
            }
        }
        true
    }
}

impl Index<Rank> for Hand {
    type Output = u8;

    fn index(&self, index: Rank) -> &Self::Output {
        &self.0[index as usize]
    }
}
