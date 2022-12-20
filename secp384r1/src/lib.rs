#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the secp384r1 curve.
//! Source: <https://neuromancer.sk/std/nist/P-384>
//!
//! Curve information:
//! * Base field: q = TODO
//! * Scalar field: r = TODO
//! * a = TODO
//! * b = TODO
//! * Curve equation: y^2 = x^3 + ax + b

#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
