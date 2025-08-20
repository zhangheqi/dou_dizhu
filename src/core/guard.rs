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
    /// use dou_dizhu::{core::Guard, hand, Play};
    /// 
    /// let rocket: Guard<Play> = hand!(const {
    ///     BlackJoker,
    ///     RedJoker,
    /// })
    /// .to_play()
    /// .unwrap();
    /// 
    /// assert!(matches!(
    ///     // Cannot perform pattern matching on the inner Play
    ///     // unless you consume the Guard via `into_inner()`.
    ///     rocket.into_inner(),
    ///     Play::Rocket,
    /// ));
    /// ```
    pub fn into_inner(self) -> T {
        self.0
    }

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
