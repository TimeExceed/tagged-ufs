#![doc = include_str!("../README.md")]

pub mod raw;
pub use self::raw::Mergable;
mod prelude;
pub use self::prelude::*;

#[cfg(test)]
mod test;
