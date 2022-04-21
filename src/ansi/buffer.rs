//! `reflors::ansi::buffer`
//!
//! Contains an ANSI escape code aware buffer implementation that can be used to store or stream text, while
//! also being able to measure the **visible** width of the contents.
//!
//! Source: [muesli/reflow/ansi/buffer.go](https://github.com/muesli/reflow/blob/00a9f5c6902562434539e11d2c8f8d3dae851318/ansi/buffer.go)

use bytes::{Bytes, BytesMut};

/// A thin wrapper around [`bytes::BytesMut`] that is able to determine visual string size.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Buffer(BytesMut);

impl Buffer {
    /// Creates a new, default, [`Buffer`].
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let buffer = Buffer::new();
    /// assert_eq!(buffer.len(), 0);
    /// assert_eq!(buffer.capacity(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Buffer`] with the given capacity.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let buffer = Buffer::with_capacity(16);
    /// assert_eq!(buffer.capacity(), 16);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(BytesMut::with_capacity(capacity))
    }

    /// Calculates the "visible" size of the internal buffer / string, ignoring any ANSI
    /// escape sequences that will not be visible in a terminal. Internally, the function
    /// delegates to [`Buffer::visible_len_ascii`] or [`Buffer::visible_len_unicode`] depending
    /// on the value of [`Buffer::is_ascii`]. Unfortunately this *does* mean that the internal
    /// buffer is looped through twice.
    ///
    /// # Examples
    /// ```
    /// use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::with_capacity(32);
    /// // Add some ansi formatting
    /// buffer.push_str("\u{1b}[1;4;38;2;255;255m");
    /// // Add some text
    /// buffer.push_str("Hello World!");
    /// // Add reset at end like a good citizen
    /// buffer.push_str("\u{1b}[0m");
    /// assert_eq!(buffer.len(), 35);
    /// assert_eq!(buffer.visible_len(), "Hello World!".len());
    /// ```
    #[must_use]
    pub fn visible_len(&self) -> usize {
        if self.is_ascii() {
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

    fn visible_len_unicode(&self) -> usize {
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

    /// Adds the given string slice to the internal buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::new();
    /// buffer.push_str("Hello World!");
    /// assert_eq!(buffer.to_str(), Some("Hello World!"));
    /// ```
    pub fn push_str(&mut self, s: &str) {
        self.0.extend_from_slice(s.as_bytes());
    }

    /// Pushes the given `char` into the internal buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.push_char('H');
    /// buffer.push_char('o');
    /// buffer.push_char('w');
    /// buffer.push_char('d');
    /// buffer.push_char('y');
    /// assert_eq!(buffer.to_str(), Some("Howdy"));
    /// ```
    pub fn push_char(&mut self, c: char) {
        self.0.extend(std::iter::once(c as u8));
    }

    /// Mutably borrow the internal [`BytesMut`] buffer, mostly useful in case I
    /// forgot to provide a wrapper to access to any useful methods on that type.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let lowered = b"hello world!";
    /// let mut buffer = Buffer::from("Hello World!");
    /// let mut bytes = buffer.bytes_mut();
    /// assert!(bytes.eq_ignore_ascii_case(&lowered[..]));
    /// ```
    pub fn bytes_mut(&mut self) -> &BytesMut {
        &mut self.0
    }

    /// Gets the raw data / bytes from the internal buffer.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::new();
    /// assert_eq!(buffer.data(), &[]);
    /// buffer.push_char(30 as char);
    /// buffer.push_char(40 as char);
    /// assert_eq!(buffer.data(), &[30, 40]);
    /// ```
    #[must_use]
    pub fn data(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Gets a mutable reference to the raw data / bytes from the internal buffer.
    ///
    /// # Examples
    /// ```
    /// use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::with_capacity(16);
    /// for _ in 0..16 {
    ///     buffer.push_char(1 as char);
    /// }
    /// assert_eq!(buffer.data(), &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    /// buffer.data_mut().fill(2);
    /// assert_eq!(buffer.data(), &[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
    /// ```
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }

    /// Attempts to create a string from the value of the internal buffer.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.push_str("Hello");
    /// assert_eq!(buffer.to_str(), Some("Hello"));
    /// ```
    #[must_use]
    pub fn to_str(&self) -> Option<&str> {
        std::str::from_utf8(self.0.as_ref()).ok()
    }

    /// Attempts to create a string from the value of the internal buffer.
    ///
    /// ### Uses `std::str::from_utf8_unchecked` instead of `std::str::from_utf8`
    ///
    /// # Examples
    ///
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.push_str("Hello");
    /// // We know we pushed a valid utf8 string, so this is presumably safe.
    /// assert_eq!(unsafe { buffer.to_str_unchecked() }, "Hello");
    /// ```
    ///
    /// # Safety
    /// Safety is in the hands of the caller. Only use this function if you are absolutely certain
    /// that the current internal buffer is a valid utf8 string. Use [`Buffer::to_str`] if there
    /// is any chance that this might not be the case.
    #[must_use]
    pub unsafe fn to_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.0.as_ref())
    }

    /// Attempts to create an owned [`String`] from the current value of the internal buffer.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.push_str("Hello");
    /// assert_eq!(buffer.to_string(), Some("Hello".to_string()));
    /// ```
    #[must_use]
    pub fn to_string(&self) -> Option<String> {
        String::from_utf8(self.0.to_vec()).ok()
    }

    /// Attempts to create an owned [`String`] from the value of the internal buffer.
    ///
    /// ### Uses `String::from_utf8_unchecked` instead of `String::from_utf8`
    ///
    /// # Examples
    ///
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// buffer.push_str("Hello");
    /// // We know we pushed a valid utf8 string, so this is presumably safe.
    /// assert_eq!(unsafe { buffer.to_string_unchecked() }, "Hello".to_string());
    /// ```
    ///
    /// # Safety
    /// Safety is in the hands of the caller. Only use this function if you are absolutely certain
    /// that the current internal buffer is a valid utf8 string. Use [`Buffer::to_string`] if there
    /// is any chance that this might not be the case.
    #[must_use]
    pub unsafe fn to_string_unchecked(&self) -> String {
        String::from_utf8_unchecked(self.0.to_vec())
    }

    /// Consumes this [`Buffer`] and returns the internal [`bytes::BytesMut`] buffer.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::from("Hello World!");
    /// let mut inner = buffer.into_bytes();
    /// // This would be an error.
    /// // buffer.push_str("Whoops!");
    /// assert_eq!(inner, bytes::BytesMut::from(&b"Hello World!"[..]));
    /// ```
    #[must_use]
    pub fn into_bytes(self) -> BytesMut {
        self.0
    }

    /// Consumes this [`Buffer`] and returns the raw data as a [`Vec`] of bytes.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let buffer = Buffer::from("Howdy");
    /// let out = buffer.into_vec();
    /// assert_eq!(out, vec!['H' as u8, 'o' as u8, 'w' as u8, 'd' as u8, 'y' as u8]);
    /// ```
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.0.into_iter().collect()
    }

    /// Consumes this [`Buffer`] and attempts to create a `String` from the contents.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let buffer = Buffer::from("Hello");
    /// let string = buffer.into_string();
    /// assert!(string.is_some());
    /// assert_eq!(string, Some("Hello".to_string()));
    /// ```
    #[must_use]
    pub fn into_string(self) -> Option<String> {
        let data = self.into_vec();
        String::from_utf8(data).ok()
    }

    /// Checks if the internal buffer contains only valid ascii characters.
    ///
    /// # Examples
    /// ```
    /// use reflors::ansi::buffer::Buffer;
    ///
    /// let buffer = Buffer::from("Hello");
    /// assert!(buffer.is_ascii());
    /// let buffer = Buffer::from("東京");
    /// assert!(!buffer.is_ascii());
    /// ```
    #[must_use]
    pub fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    /// Checks whether the internal buffer is currently empty.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::new();
    /// assert!(buffer.is_empty());
    /// buffer.push_char('H');
    /// assert!(!buffer.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the current length of the internal buffer.
    ///
    /// # Examples
    /// ```
    /// # use reflors::ansi::buffer::Buffer;
    /// let mut buffer = Buffer::new();
    /// assert_eq!(buffer.len(), 0);
    /// buffer.push_str("Hello");
    /// assert_eq!(buffer.len(), 5);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets the current **capacity** of the internal buffer.
    ///
    /// # Examples
    /// ```
    /// use reflors::ansi::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::new();
    /// assert_eq!(buffer.capacity(), 0);
    /// let mut buffer = Buffer::with_capacity(16);
    /// assert_eq!(buffer.capacity(), 16);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
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

// impl<T> From<T> for Buffer
// where
//     T: Into<BytesMut>,
// {
//     fn from(src: T) -> Self {
//         Self(src.into())
//     }
// }

impl From<BytesMut> for Buffer {
    fn from(src: BytesMut) -> Self {
        Self(src)
    }
}

impl From<&BytesMut> for Buffer {
    fn from(src: &BytesMut) -> Self {
        Self(src.clone())
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(src: Vec<u8>) -> Self {
        let mut bytes = BytesMut::from(src.as_slice());
        Self(bytes)
    }
}

impl From<&Vec<u8>> for Buffer {
    fn from(src: &Vec<u8>) -> Self {
        let mut bytes = BytesMut::from(src.as_slice());
        // bytes.extend_from_slice(src);
        Self(bytes)
    }
}

impl From<Vec<char>> for Buffer {
    fn from(src: Vec<char>) -> Self {
        let mut bytes = src.into_iter().map(|c| c as u8).collect::<BytesMut>();
        Self(bytes)
    }
}

impl From<&Vec<char>> for Buffer {
    fn from(src: &Vec<char>) -> Self {
        let mut bytes = src.iter().map(|c| *c as u8).collect::<BytesMut>();
        // bytes.extend_from_slice(src);
        Self(bytes)
    }
}

impl From<String> for Buffer {
    fn from(src: String) -> Self {
        let mut bytes = BytesMut::with_capacity(src.len());
        bytes.extend_from_slice(src.into_bytes().as_ref());
        Self(bytes)
    }
}

impl From<&String> for Buffer {
    fn from(src: &String) -> Self {
        let mut bytes = BytesMut::with_capacity(src.len());
        bytes.extend_from_slice(src.as_bytes().as_ref());
        Self(bytes)
    }
}

impl<'s> From<&'s str> for Buffer {
    fn from(src: &'s str) -> Self {
        let mut bytes = BytesMut::with_capacity(src.len());
        bytes.extend_from_slice(src.as_bytes().as_ref());
        Self(bytes)
    }
}

impl FromIterator<u8> for Buffer {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(BytesMut::from_iter(iter))
    }
}

impl FromIterator<char> for Buffer {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().map(|c| c as u8).collect())
    }
}

impl<'any> FromIterator<&'any u8> for Buffer {
    fn from_iter<T: IntoIterator<Item = &'any u8>>(iter: T) -> Self {
        Self(BytesMut::from_iter(iter))
    }
}

impl<'any> FromIterator<&'any char> for Buffer {
    fn from_iter<T: IntoIterator<Item = &'any char>>(iter: T) -> Self {
        Self(iter.into_iter().map(|c| *c as u8).collect())
    }
}

impl From<Buffer> for BytesMut {
    fn from(src: Buffer) -> Self {
        src.into_bytes()
    }
}

impl From<&Buffer> for BytesMut {
    fn from(src: &Buffer) -> Self {
        src.clone().into_bytes()
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(src: Buffer) -> Self {
        src.into_vec()
    }
}

impl From<&Buffer> for Vec<u8> {
    fn from(src: &Buffer) -> Self {
        src.clone().into_vec()
    }
}

impl<'b> From<&'b Buffer> for &'b [u8] {
    fn from(src: &'b Buffer) -> Self {
        src.data()
    }
}

impl<'b> From<&'b mut Buffer> for &'b mut [u8] {
    fn from(src: &'b mut Buffer) -> Self {
        src.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn from() {
        let buffer = Buffer::from("Hello");
        let buffer = Buffer::from("Hello".to_string().as_str());
        let buffer = Buffer::from(Vec::<u8>::new());
    }

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
