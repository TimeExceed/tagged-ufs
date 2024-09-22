#![doc = include_str!("../README.md")]

mod prelude;
pub use self::prelude::*;
mod tag;
pub use self::tag::*;

#[cfg(test)]
mod test;
