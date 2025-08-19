//! Toolkit for the Chinese card game Dou Dizhu (斗地主).
//! 
//! This crate implements Dou Dizhu strictly following the [Pagat rules](https://www.pagat.com/climbing/doudizhu.html),
//! though it uses different terminology.

#[doc(hidden)]
pub mod __private;
pub mod core;
mod hand;
mod macros;
mod play;
mod rank;

pub use hand::Hand;
pub use play::{Play, PlayKind, PlayKind::*};
pub use rank::Rank;
