use unicode_xid::UnicodeXID;

/// Returns whether or not the specified character is a Unicode white space character.  This function
/// excludes any line terminating white spaces.
#[inline]
pub fn is_white_space(c: char) -> bool {
    match c {
        '\u{9}' => true,
        '\u{20}' => true,
        '\u{A0}' => true,
        '\u{1680}' => true,
        '\u{2000}'..='\u{200A}' => true,
        '\u{202f}' => true,
        '\u{205f}' => true,
        '\u{3000}' => true,
        _ => false,
    }
}

/// Returns whether or not the specified character is a Unicode line breaking white space character.
#[inline]
pub fn is_line_term(c: char) -> bool {
    match c {
        '\u{A}'..='\u{D}' => true,
        '\u{85}' => true,
        '\u{2028}'..='\u{2029}' => true,
        _ => false
    }
}

/// Returns whether or not the specified character is an identifier starting character.  Matches an
/// extended version of the Unicode XID start character group, which allows underscores.
#[inline]
pub fn is_iden_start(c: char) -> bool {
    match c {
        '_' => true,
        _ => UnicodeXID::is_xid_start(c)
    }
}

/// Returns whether or not the specified character is a Unicode XID continuing character.
#[inline]
pub fn is_iden_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}

/// Returns whether or not the specified character is a punctuator.  Punctuators may be operators or
/// any other punctuation character.
#[inline]
pub fn is_punctuator(c: char) -> bool {
    match c {
        ';' | ',' | '.' | '(' | ')' | '{' | '}' | '[' | ']' | '@' | '#' | '~' | '?' | ':' | '$' | '='
        | '!' | '<' | '>' | '-' | '&' | '|' | '+' | '*' | '/' | '^' | '%' => true,
        _ => false
    }
}