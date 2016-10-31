extern crate unicola;

use std::env;

fn main() {
    let word = env::args().skip(1).take(1).next().unwrap();
    for c in word.chars() {
        println!("{} => {:?}", c, unicola::unicode_east_asian_width(c));
    }
}
