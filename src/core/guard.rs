//! Invariant–preserving wrapper.
//! 
//! This module defines [`Guard`], a generic wrapper that marks values as
//! having passed validation.

use std::ops::Deref;

/// Invariant–preserving wrapper.
/// 
/// `Guard<T>` is a generic wrapper that marks a value of type `T` as having
/// passed invariant checks enforced by the crate. It is primarily used to
/// ensure that types with public constructors, such as enums, are created
/// or checked through crate APIs.
#[derive(Debug, Clone)]
pub struct Guard<T>(pub(crate) T);

impl<T> Guard<T> {
    /// Consumes the `Guard`, returning the underlying value.
    /// 
    /// After calling this, the value is no longer protected by the invariant
    /// checks enforced by the crate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::{*, core::Guard};
    /// 
    /// let rocket: Guard<Play> = play!(const {
    ///     BlackJoker,
    ///     RedJoker,
    /// })
    /// .unwrap();
    /// 
    /// assert!(matches!(
    ///     rocket.into_inner(),
    ///     Play::Rocket,
    /// ));
    /// ```
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Creates a Guard without performing any validation.
    /// 
    /// Use only when you have already proven that `value` upholds all invariants
    /// required by this crate for `T` (e.g., via prior validation or construction
    /// from a trusted source). This bypass must not be used if the value may be
    /// mutated in ways that could later violate those invariants, including through
    /// interior mutability.
    /// 
    /// # Safety
    /// 
    /// The caller must guarantee that:
    /// - `value` satisfies all invariants expected by `Guard<T>` for the entirety
    ///   of the returned guard’s lifetime.
    /// - No aliasing or interior mutations can break those invariants after creation.
    /// 
    /// Prefer using safe constructors and validation APIs provided by this crate.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dou_dizhu::{*, core::Guard};
    /// 
    /// let play: Guard<Play> = unsafe {
    ///     Guard::new_unchecked(
    ///         Play::Bomb(Rank::Three)
    ///     )
    /// };
    /// assert!((Hand::FULL_DECK - &play).is_some());
    /// ```
    pub unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
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
