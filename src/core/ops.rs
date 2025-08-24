use std::ops::{Add, Sub};
use crate::{core::Guard, Hand, Play};

pub trait UncheckedAddExt<Rhs = Self>
where
    Self: private::Sealed,
    Rhs: private::SealedRhs,
{
    type Output;

    unsafe fn unchecked_add(self, rhs: Rhs) -> Self::Output;
}

pub trait UncheckedSubExt<Rhs = Self>
where
    Self: private::Sealed,
    Rhs: private::SealedRhs,
{
    type Output;

    unsafe fn unchecked_sub(self, rhs: Rhs) -> Self::Output;
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Hand {}
    pub trait SealedRhs {}
    impl SealedRhs for crate::Hand {}
    impl SealedRhs for &crate::core::Guard<crate::Play> {}
}

impl UncheckedAddExt for Hand {
    type Output = Self;

    unsafe fn unchecked_add(mut self, rhs: Self) -> Self::Output {
        for i in 0..15 {
            self.0[i] += rhs.0[i];
        }
        self
    }
}

impl UncheckedSubExt for Hand {
    type Output = Self;

    unsafe fn unchecked_sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..15 {
            self.0[i] = self.0[i].wrapping_sub(rhs.0[i]);
        }
        self
    }
}

impl UncheckedAddExt<&Guard<Play>> for Hand {
    type Output = Self;

    unsafe fn unchecked_add(self, rhs: &Guard<Play>) -> Self::Output {
        unsafe { self.unchecked_add(rhs.to_hand()) }
    }
}

impl UncheckedSubExt<&Guard<Play>> for Hand {
    type Output = Self;

    unsafe fn unchecked_sub(self, rhs: &Guard<Play>) -> Self::Output {
        unsafe { self.unchecked_sub(rhs.to_hand()) }
    }
}

impl Add for Hand {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::try_from(unsafe { self.unchecked_add(rhs).0 }).ok()
    }
}

impl Add<&Guard<Play>> for Hand {
    type Output = Option<Self>;

    fn add(self, rhs: &Guard<Play>) -> Self::Output {
        Self::try_from(unsafe { self.unchecked_add(rhs).0 }).ok()
    }
}

impl Add<Hand> for &Guard<Play> {
    type Output = Option<Hand>;

    fn add(self, rhs: Hand) -> Self::Output {
        rhs + self
    }
}

impl Add<Option<Self>> for Hand {
    type Output = Option<Self>;

    fn add(self, rhs: Option<Self>) -> Self::Output {
        rhs.and_then(|y| self + y)
    }
}

impl Add<Hand> for Option<Hand> {
    type Output = Self;

    fn add(self, rhs: Hand) -> Self::Output {
        rhs + self
    }
}

impl Add<&Guard<Play>> for Option<Hand> {
    type Output = Self;

    fn add(self, rhs: &Guard<Play>) -> Self::Output {
        self.and_then(|x| x + rhs)
    }
}

impl Add<Option<Hand>> for &Guard<Play> {
    type Output = Option<Hand>;

    fn add(self, rhs: Option<Hand>) -> Self::Output {
        rhs + self
    }
}

impl Sub for Hand {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::try_from(unsafe { self.unchecked_sub(rhs).0 }).ok()
    }
}

impl Sub<&Guard<Play>> for Hand {
    type Output = Option<Self>;

    fn sub(self, rhs: &Guard<Play>) -> Self::Output {
        Self::try_from(unsafe { self.unchecked_sub(rhs).0 }).ok()
    }
}

impl Sub<Hand> for &Guard<Play> {
    type Output = Option<Hand>;

    fn sub(self, rhs: Hand) -> Self::Output {
        self.to_hand() - rhs
    }
}

impl Sub<Option<Self>> for Hand {
    type Output = Option<Self>;

    fn sub(self, rhs: Option<Self>) -> Self::Output {
        rhs.and_then(|y| self - y)
    }
}

impl Sub<Hand> for Option<Hand> {
    type Output = Self;

    fn sub(self, rhs: Hand) -> Self::Output {
        self.and_then(|x| x - rhs)
    }
}

impl Sub<&Guard<Play>> for Option<Hand> {
    type Output = Self;

    fn sub(self, rhs: &Guard<Play>) -> Self::Output {
        self.and_then(|x| x - rhs)
    }
}

impl Sub<Option<Hand>> for &Guard<Play> {
    type Output = Option<Hand>;

    fn sub(self, rhs: Option<Hand>) -> Self::Output {
        rhs.and_then(|y| self - y)
    }
}
