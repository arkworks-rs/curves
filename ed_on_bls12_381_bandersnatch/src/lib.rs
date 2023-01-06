// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the Bendersnatch curve, a twisted Edwards curve
//! whose base field is the scalar field of the curve BLS12-381. This allows
//! defining cryptographic primitives that use elliptic curves over the scalar
//! field of the latter curve. This curve was generated by Simon Masson from
//! Anoma, and Antonio Sanso from Ethereum Foundation, and is also known as [bandersnatch](https://ethresear.ch/t/introducing-bandersnatch-a-fast-elliptic-curve-built-over-the-bls12-381-scalar-field/9957).
//!
//! See [here](https://github.com/asanso/Bandersnatch/blob/main/README.md) for the specification of the curve.
//! There was also a Python implementation [here](https://github.com/asanso/Bandersnatch/).
//!
//! Curve information:
//! * Base field: q =
//!   52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * Scalar field: r =
//!   13108968793781547619861935127046491459309155893440570251786403306729687672801
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 5
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -5
//!    * d = 45022363124591815672509500913686876175488063829319466900776701791074614335719

#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
