#![cfg_attr(feature = "bench", feature(test))]

mod backtracker;

pub use backtracker::{Config, solve};
pub mod queen;
pub mod trunks;
