extern crate phf_codegen;
#[cfg(feature = "eac")]
extern crate csv;

pub mod generator;
pub use generator::*;
