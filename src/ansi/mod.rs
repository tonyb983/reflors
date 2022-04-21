pub mod buffer;
pub mod writer;

/// Marker for the start of an ansi escape code.
pub const MARKER: char = '\u{1b}';

/// Checks whether the given character is a terminating character (i.e. in the range 0x40..=0x5a or 0x61..=0x7a).
#[must_use]
pub fn is_terminator(ch: char) -> bool {
    let u = ch as u32;
    (0x40..=0x5a).contains(&u) || (0x61..=0x7a).contains(&u)
}

/// Returns the **visble** width of the given string, ignoring ansi escape sequences.
///
/// TODO: Figure out how to handle '\n' in this function (and in general)
pub fn visible_width(input: &str) -> usize {
    let mut count = 0usize;
    let mut in_ansi_seq = false;

    for ch in input.chars() {
        if ch == MARKER {
            in_ansi_seq = true;
        } else if in_ansi_seq {
            if is_terminator(ch) {
                in_ansi_seq = false;
            }
        } else {
            match ch {
                '\t' => count += 8 - (count % 8),
                '\n' => count = 0,
                _ => count += char::len_utf8(ch),
            }
        }
    }

    count
}

/// Strips any ansi escape sequences from the given string.
/// ## Panics
/// Because it is not implemented.
/// TODO: Write this function!
#[must_use]
pub fn strip_ansi(input: &str) -> String {
    todo!("strip_ansi not implemented");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminator_tester() {
        for u in 0x40..=0x5au8 {
            // let ch = u as char;
            // println!("Value '{:x}' is character '{}'", u, ch);
            assert!(is_terminator(u as char));
        }
        for u in 0x61..=0x7au8 {
            // let ch = u as char;
            // println!("Value '{:x}' is character '{}'", u, ch);
            assert!(is_terminator(u as char));
        }
    }
}

// \u{1b}[1;4;38;2;255;255m
