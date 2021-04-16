#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order pairing-friendly curve Pluto, generated
//! by [Daira Hopwood](https://github.com/daira/pluto-eris). This curve is part of a
//! half-pairing cycle with Eris, i.e. its scalar field and base field respectively
//! are the base field and scalar field of Eris.
//!
//!
//! Curve information:
//! * Base field: q =
//!   0x24000000000024000130e0000d7f70e4a803ca76f439266f443f9a5cda8a6c7be4a7a5fe8fadffd6a2a7e8c30006b9459ffffcd300000001
//! * Scalar field: r =
//!   0x24000000000024000130e0000d7f70e4a803ca76f439266f443f9a5c7a8a6c7be4a775fe8e177fd69ca7e85d60050af41ffffcd300000001
//! * Curve equation: y^2 = x^3 + 57
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32

#[cfg(feature = "r1cs")]
pub mod constraints;
#[cfg(feature = "curve")]
mod curves;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
pub use fields::*;
