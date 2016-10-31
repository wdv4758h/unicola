/// http://www.unicode.org/reports/tr44/#General_Category_Values
pub enum UnicodeCategory {
    // L -> Letter
    /// an uppercase letter
    Lu,
    /// a lowercase letter
    Ll,
    /// a digraphic character, with first part uppercase
    Lt,
    /// a modifier letter
    Lm,
    /// other letters, including syllables and ideographs
    Lo,

    // M -> Mark
    /// a nonspacing combining mark (zero advance width)
    Mn,
    /// a spacing combining mark (positive advance width)
    Mc,
    /// an enclosing combining mark
    Me,

    // N -> Number
    /// a decimal digit
    Nd,
    /// a letterlike numeric character
    Nl,
    /// a numeric character of other type
    No,

    // P -> Punctuation
    /// a connecting punctuation mark, like a tie
    Pc,
    /// a dash or hyphen punctuation mark
    Pd,
    /// an opening punctuation mark (of a pair)
    Ps,
    /// a closing punctuation mark (of a pair)
    Pe,
    /// an initial quotation mark
    Pi,
    /// a final quotation mark
    Pf,
    /// a punctuation mark of other type
    Po,

    // S -> Symbol
    /// a symbol of mathematical use
    Sm,
    /// a currency sign
    Sc,
    /// a non-letterlike modifier symbol
    Sk,
    /// a symbol of other type
    So,

    // Z -> Seperator
    /// a space character (of various non-zero widths)
    Zs,
    /// U+2028 LINE SEPARATOR only
    Zl,
    /// U+2029 PARAGRAPH SEPARATOR only
    Zp,

    // C -> Other
    /// a C0 or C1 control code
    Cc,
    /// a format control character
    Cf,
    /// a surrogate code point
    Cs,
    /// a private-use character
    Co,
    /// a reserved unassigned code point or a noncharacter
    Cn,
}

/// http://www.unicode.org/reports/tr44/#Bidi_Class_Values
pub enum UnicodeBidirection {
    // Strong Types
    /// any strong left-to-right character
    L,
    /// any strong right-to-left (non-Arabic-type) character
    R,
    /// any strong right-to-left (Arabic-type) character
    AL,

    // Weak Types
    /// any ASCII digit or Eastern Arabic-Indic digit
    EN,
    /// plus and minus signs
    ES,
    /// a terminator in a numeric format context, includes currency signs
    ET,
    /// any Arabic-Indic digit
    AN,
    /// commas, colons, and slashes
    CS,
    /// any nonspacing mark
    NSM,
    /// most format characters, control codes, or noncharacters
    BN,

    // Neutral Types
    /// various newline characters
    B,
    /// various segment-related control codes
    S,
    /// spaces
    WS,
    /// most other symbols and punctuation marks
    ON,

    // Explicit Formatting Types
    /// U+202A: the LR embedding control
    LRE,
    /// U+202D: the LR override control
    LRO,
    /// U+202B: the RL embedding control
    RLE,
    /// U+202E: the RL override control
    RLO,
    /// U+202C: terminates an embedding or override control
    PDF,
    /// U+2066: the LR isolate control
    LRI,
    /// U+2067: the RL isolate control
    RLI,
    /// U+2068: the first strong isolate control
    FSI,
    /// U+2069: terminates an isolate control
    PDI,
}

/// http://www.unicode.org/reports/tr11/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EastAsianWidth {
    /// East Asian Fullwidth
    F,
    /// East Asian Halfwidth
    H,
    /// East Asian Wide
    W,
    /// East Asian Narrow
    Na,
    /// East Asian Ambiguous
    A,
    /// Neutral (Not East Asian)
    N,
}

pub enum MandatoryLineBreaks {
    /// Mandatory Break, cause a line break (after)
    BK,
    /// Carriage Return, cause a line break (after), except between CR and LF
    CR,
    /// Line Feed, cause a line break (after)
    LF,
    /// Next Line, cause a line break (after)
    NL,
}
