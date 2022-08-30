//!
#![no_std]

mod fpcr;
mod guard;

pub use fpcr::{disable_subnormal, enable_subnormal, RoundingMode};
pub use guard::SubnormalGuard;
