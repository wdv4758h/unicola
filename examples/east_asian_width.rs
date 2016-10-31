extern crate unicola;

use std::env;

#[cfg(feature = "width")]
fn main() {
    let word = env::args().skip(1).take(1).next().unwrap();
    for c in word.chars() {
        println!("{} => {:?}", c, unicola::east_asian_width(c));
    }
}

#[cfg(not(feature = "width"))]
fn main() {}
