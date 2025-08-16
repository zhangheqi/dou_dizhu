use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Guard<T>(pub(crate) T);

impl<T> Guard<T> {
    pub fn into_inner(self) -> T {
        self.0
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
