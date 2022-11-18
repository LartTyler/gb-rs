#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_option)]
#![feature(const_mut_refs)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]

pub mod containers;
pub mod instructions;
pub mod operations;
pub mod parse;
pub mod read;
pub mod sets;
