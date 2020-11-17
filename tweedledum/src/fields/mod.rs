#[cfg(feature = "base_field")]
pub mod fp;
#[cfg(feature = "base_field")]
pub use self::fp::*;

#[cfg(feature = "scalar_field")]
pub mod fq;
#[cfg(feature = "scalar_field")]
pub use self::fq::*;
