use crate::{Hand, Rank};

#[macro_export]
macro_rules! __hand {
    (() -> ($($spec:tt)*)($($body:tt)*)($var:ident)) => {
        {
            const { $crate::__private::hand::check_partial_specs([$($spec)*]) }
            let mut $var = [0u8; 15];
            $($body)*
            $crate::Hand::try_from($var)
        }
    };
    (($rank:ident $(: $count:expr)?) -> ($($spec:tt)*)($($body:tt)*)($var:ident)) => {
        $crate::__hand!(($rank $(: $count)?,) -> ($($spec)*)($($body)*)($var))
    };
    (($rank:ident: $count:expr, $($t:tt)*) -> ($($spec:tt)*)($($body:tt)*)($var:ident)) => {
        $crate::__hand!(($($t)*) -> (
            $($spec)*
            $crate::__private::hand::PartialSpec {
                rank: $crate::Rank::$rank,
                texts: $crate::__private::hand::PartialSpecTexts {
                    duplicate_error: concat!("duplicate card count specified for `", stringify!($rank), "`"),
                },
            },
        )(
            $($body)*
            $var[$crate::Rank::$rank as usize] = $count;
        )($var))
    };
    (($rank:ident, $($t:tt)*) -> ($($spec:tt)*)($($body:tt)*)($var:ident)) => {
        $crate::__hand!(($rank: 1, $($t)*) -> ($($spec)*)($($body)*)($var))
    };
}

#[macro_export]
macro_rules! __const_hand {
    (() -> ($($body:tt)*)) => {
        const { $crate::__private::hand::from_specs([$($body)*]) }
    };
    (($rank:ident $(: $count:expr)?) -> ($($body:tt)*)) => {
        $crate::__const_hand!(($rank $(: $count)?,) -> ($($body)*))
    };
    (($rank:ident: $count:expr, $($t:tt)*) -> ($($body:tt)*)) => {
        $crate::__const_hand!(($($t)*) -> ($($body)* $crate::__private::hand::Spec {
            rank: $crate::Rank::$rank,
            count: $count,
            texts: $crate::__private::hand::SpecTexts {
                more_than_four_error: concat!("more than four `", stringify!($rank), "`s are specified"),
                duplicate_error: concat!("duplicate card count specified for `", stringify!($rank), "`"),
            },
        },))
    };
    (($rank:ident, $($t:tt)*) -> ($($body:tt)*)) => {
        $crate::__const_hand!(($rank: 1, $($t)*) -> ($($body)*))
    };
}

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
