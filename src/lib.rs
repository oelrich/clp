//! # CLP : A Constraint Logic Programming library
//! Automatically solve problems without thinking!
//! Amaze your friends! Confidently deal with the thing
//! that actually needs doing while dumping the core
//! work to some random guy with a keyboard.
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod expressions;

pub mod solver;

#[cfg(test)]
mod tests;
