#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the secp256k1 curve.
//!
//! Curve information:
//! Vesta:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Curve equation: y^2 = x^3 + 5

#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
