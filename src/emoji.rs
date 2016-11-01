use phf;


// with Emoji Modifier, we will need two chars to save it
// http://unicode.org/reports/tr51/#Emoji_Modifiers_Table
pub fn emoji(code: &str) -> Option<&&str> {
    EMOJI_ALPHA_CODES.get(code)
}


/// Emoji Alpha Codes
/// e.g. "static EMOJI_ALPHA_CODES: pdf::Map<&'static str, &'static str> = ... ;"
#[cfg(not(feature = "pregen"))]
include!(concat!(env!("OUT_DIR"), "/eac.rs"));

#[cfg(feature = "pregen")]
include!("generated/eac.rs");


#[test]
fn test_emoji() {
    assert_eq!("😃", *emoji(":smiley:").unwrap());
    assert_eq!("🔥", *emoji(":fire:").unwrap());
    assert_eq!("✋🏼", *emoji(":raised_hand_tone2:").unwrap());
}
