use bytes::{Bytes, BytesMut};

/// A thin wrapper around [`bytes::BytesMut`] that is able to determine visual string size.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Buffer(BytesMut);

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(BytesMut::with_capacity(capacity))
    }

    pub fn visible_len(&self) -> usize {
        if self.0.is_ascii() {
            println!("Using ascii version.");
            self.visible_len_ascii()
        } else {
            println!("Using unicode version.");
            self.visible_len_unicode()
        }
    }

    fn visible_len_ascii(&self) -> usize {
        let mut count = 0usize;
        let mut toggle = false;

        for b in &self.0 {
            let ch = *b as char;
            // println!(
            //     "Byte '{:x}' Char '{}' Count = {} Toggle = {}",
            //     b,
            //     ch,
            //     count,
            //     if toggle { "ON" } else { "OFF" }
            // );
            if ch == super::MARKER {
                toggle = true;
            } else if toggle {
                if super::is_terminator(ch) {
                    toggle = false;
                }
            } else {
                count += char::len_utf8(ch);
            }
        }

        count
    }

    pub fn visible_len_unicode(&self) -> usize {
        let mut count = 0usize;
        let mut toggle = false;

        let string = match self.to_str() {
            Some(s) => s,
            None => return 0,
        };

        for ch in string.chars() {
            let b = ch as u8;
            // println!(
            //     "Byte '{:x}' Char '{}' Count = {} Toggle = {}",
            //     b,
            //     ch,
            //     count,
            //     if toggle { "ON" } else { "OFF" }
            // );
            if ch == super::MARKER {
                toggle = true;
            } else if toggle {
                if super::is_terminator(ch) {
                    toggle = false;
                }
            } else {
                count += char::len_utf8(ch);
            }
        }

        count
    }

    pub fn push_str(&mut self, s: &str) {
        self.0.extend_from_slice(s.as_bytes());
    }

    pub fn push_char(&mut self, c: char) {
        self.0.extend(std::iter::once(c as u8));
    }

    pub fn data(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }

    pub fn to_str(&self) -> Option<&str> {
        std::str::from_utf8(self.0.as_ref()).ok()
    }

    pub unsafe fn to_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.0.as_ref())
    }

    pub fn to_string(&self) -> Option<String> {
        String::from_utf8(self.0.to_vec()).ok()
    }

    pub unsafe fn to_string_unchecked(&self) -> String {
        String::from_utf8_unchecked(self.0.to_vec())
    }

    pub fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl bytes::Buf for Buffer {
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.0.chunk()
    }

    fn advance(&mut self, cnt: usize) {
        self.0.advance(cnt);
    }
}

unsafe impl bytes::BufMut for Buffer {
    fn remaining_mut(&self) -> usize {
        self.0.remaining_mut()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.0.advance_mut(cnt);
    }

    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        self.0.chunk_mut()
    }
}

impl<T> From<T> for Buffer
where
    T: Into<BytesMut>,
{
    fn from(src: T) -> Self {
        Self(src.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn basic() {
        let b: Buffer = "Hello!".into();
        assert_eq!(b.len(), 6, "Buffer length should be 6");
        assert_eq!(b.visible_len(), 6, "Buffer visible length should be 6");
        assert!(b.is_ascii(), "Buffer should be ASCII");
    }

    #[test]
    fn ansi() {
        let b: Buffer = "\u{1b}[1;4;38;2;255;255mHello!\u{1b}[0m".into();
        assert_eq!(b.len(), 29, "Buffer length should be 29");
        assert_eq!(b.visible_len(), 6, "Buffer visible length should be 6");
        assert!(b.is_ascii(), "Buffer should be ASCII");

        let b: Buffer = "\u{1b}[1;4;38;2;255;255m_Hello!_\u{1b}[0m".into();
        assert_eq!(b.len(), 31, "Buffer length should be 31");
        assert_eq!(b.visible_len(), 8, "Buffer visible length should be 8");
        assert!(b.is_ascii(), "Buffer should be ASCII");
    }

    #[test]
    fn unicode() {
        let b1: Buffer = "🤔".into();
        let b2: Buffer = "\u{1b}[1;4;38;2;255;255m🤔\u{1b}[0m".into();
        assert_eq!(char::len_utf8('🤔'), 4, "🤔 should be 4 bytes");
        assert_eq!(b1.len(), 4, "Buffer length should be 4");
        assert_eq!(b1.visible_len(), 4, "Buffer visible length should be 4");
        assert_eq!(b2.len(), 27, "Buffer length should be 27");
        assert_eq!(b2.visible_len(), 4, "Buffer visible length should be 4");
        assert_eq!(b1.visible_len(), b2.visible_len());

        let b1: Buffer = "東京".into();
        let b2: Buffer = "\u{1b}[1;4;38;2;255;255m東京\u{1b}[0m".into();
        assert_eq!(b1.len(), 6, "Buffer length should be 6");
        assert_eq!(b1.visible_len(), 6, "Buffer visible length should be 6");
        assert_eq!(b2.len(), 29, "Buffer length should be 29");
        assert_eq!(b2.visible_len(), 6, "Buffer visible length should be 6");
        assert_eq!(b1.visible_len(), b2.visible_len());
    }

    #[test]
    fn single() {
        let b2: Buffer = "\u{1b}[1;4;38;2;255;255m🤔\u{1b}[0m".into();
        assert_eq!(b2.visible_len(), 4, "Buffer visible length should be 4");
        assert!(!b2.is_ascii(), "Buffer should be ASCII");
    }
}
