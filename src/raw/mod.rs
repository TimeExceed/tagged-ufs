//! Raw implementation of union-find sets,
//! which does not provide iterability over elements.

mod r#impl;
pub use self::r#impl::*;

#[cfg(test)]
pub(crate) mod test;
