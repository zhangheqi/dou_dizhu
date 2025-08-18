//! Core building blocks.
//! 
//! This module contains the intermediate types and traits that power the
//! high–level public APIs. They can be valuable if you need fine–grained
//! control beyond the high–level wrappers.

pub mod composition;
pub mod guard;
pub mod search;

pub use composition::{Composition, CompositionExt, Group};
pub use guard::Guard;
pub use search::{PlaySpec, SearchExt};
