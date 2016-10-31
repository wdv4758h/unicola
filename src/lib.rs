extern crate unicola_core;
extern crate phf;

#[cfg(feature = "width")]
pub mod width;

#[cfg(feature = "width")]
pub use width::*;


/// The version of Unicode,
/// e.g. "pub const UNICODE_VERSION: (u64, u64, u64) = (9, 0, 0);"
#[cfg(not(feature = "pregen"))]
include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[cfg(feature = "pregen")]
include!("generated/version.rs");
