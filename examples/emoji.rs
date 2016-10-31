extern crate unicola;

use std::env;

#[cfg(feature = "eac")]
fn main() {
    let code = env::args().skip(1).take(1).next().unwrap();
    println!("\"{}\" => {:?}", code, unicola::emoji(code.as_str()));
}

#[cfg(not(feature = "eac"))]
fn main() {}
