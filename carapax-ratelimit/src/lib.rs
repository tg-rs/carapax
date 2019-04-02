#![warn(missing_docs)]
//! A rate limit handler for carapax

mod direct;
mod keyed;

pub use self::direct::*;
pub use self::keyed::*;

pub use nonzero_ext::nonzero;
