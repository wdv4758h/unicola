use unicola_core::*;
use phf;


pub fn unicode_east_asian_width(c: char) -> EastAsianWidth {
    // PHF map
    *EAST_ASIAN_WIDTH_TABLE.get(&(c as u32)).unwrap()
}


/// character width table
/// e.g. "static WIDTH_TABLE: pdf::Map<char, u8> = ... ;"
#[cfg(not(feature = "pregen"))]
include!(concat!(env!("OUT_DIR"), "/east_asian_width_table.rs"));

#[cfg(feature = "pregen")]
include!("generated/east_asian_width_table.rs");


#[test]
fn test_east_asian_width() {
    assert_eq!(EastAsianWidth::Na, unicode_east_asian_width('a'));
    assert_eq!(EastAsianWidth::W, unicode_east_asian_width('測'));
}
