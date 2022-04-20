mod buffer;
mod writer;

/// Marker for the start of an ansi escape code.
pub const MARKER: char = '\u{1b}';

/// Checks whether the given character is a terminating character (i.e. in the range 0x40..=0x5a or 0x61..=0x7a).
#[must_use]
pub fn is_terminator(ch: char) -> bool {
    let u = ch as u8;
    (0x40..=0x5a).contains(&u) || (0x61..=0x7a).contains(&u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_tester() {
        let input = "\u{1b}[1;4;38;2;255;255mHello!\u{1b}[0m";
        let detected = unicode_width::UnicodeWidthStr::width(input);
        println!("The width of '{}' is {}", input, detected);
        assert_eq!(detected, 6);
    }

    #[test]
    fn terminator_tester() {
        for u in 0x40..=0x5au8 {
            let ch = u as char;
            println!("Value '{:x}' is character '{}'", u, ch);
        }
        for u in 0x61..=0x7au8 {
            let ch = u as char;
            println!("Value '{:x}' is character '{}'", u, ch);
        }
    }
}

// \u{1b}[1;4;38;2;255;255m