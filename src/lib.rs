#![doc=include_str!("../crates-readme.md")]
#![no_std]

mod b32;
mod b64;

pub use crate::b32::SplitMix32;
pub use crate::b64::SplitMix64;
