#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order curve Tweedledee, generated by
//! [Daira Hopwood](https://github.com/daira/tweedle). The main feature of this
//! curve is that its scalar field and base field respectively equal the
//! base field and scalar field of Tweedledum.
//!
//!
//! Curve information:
//! Tweedledee:
//! * Base field: p = 28948022309329048855892746252171976963322203655954433126947083963168578338817
//! * Scalar field: q = 28948022309329048855892746252171976963322203655955319056773317069363642105857
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(p - 1, 2) = 34
//! * Valuation(q - 1, 2) = 33

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
