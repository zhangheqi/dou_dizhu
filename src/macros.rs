/// Macro for constructing `Hand` instances.
/// 
/// # Examples
/// 
/// ```
/// # fn main() -> Result<(), String> {
/// use dou_dizhu::{hand, Hand};
/// 
/// // Compile-time hand
/// const FOUR_WITH_DUAL_SOLO: Hand = hand!(const {
///     Three: 4,
///     Four,
///     Five,
/// });
/// 
/// // Hand with runtime-computed card count
/// let computed: Hand = hand!({
///     Three: {
///         println!("computing number of `Three`s...");
///         2
///     },
/// })?;
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! hand {
    (const {$($t:tt)*}) => {
        $crate::__const_hand!(($($t)*) -> ())
    };
    ({$($t:tt)*}) => {
        $crate::__hand!(($($t)*) -> ()()(var))
    };
}

#[doc(hidden)]
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

#[doc(hidden)]
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
