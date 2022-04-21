// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//! `reflors::ansi::writer`
//!
//! Contains an ANSI escape code aware [`Writer`] that implements [`std::io::Write`]. It stores the most recent ansi styling information,
//! and can be used to reset and restore that style.
//!
//! I am currently lacking a bit of direction on this struct, as there aren't a ton of examples of it being used in the go library. I've
//! gone back and forth about what type the [`Writer::writer`] field should be, first having a [`Box<dyn std::io::Write>`], and then having
//! a generic parameter [`W: std::io::Write`], and now I'm using the [`Writer`](bytes::buf::Writer) type, with a generic parameter for the
//! type of underlying buffer used by the bytes version of writer.
//!
//! Source: [muesli/reflow/ansi/writer.go](https://github.com/muesli/reflow/blob/00a9f5c6902562434539e11d2c8f8d3dae851318/ansi/writer.go)

use std::{
    fmt::{Error as FmtError, Result as FmtResult, Write as FmtWrite},
    io::{Error as IoError, Result as IoResult, Write as IoWrite},
};

use bytes::{buf::Writer as BufWriter, BufMut, BytesMut};

use crate::{Error, Result};

///
pub struct Writer<B> {
    writer: BufWriter<B>,
    ansi: bool,
    ansi_seq: bytes::BytesMut,
    last_seq: bytes::BytesMut,
    seq_changed: bool,
    char_buff: [u8; 4],
}

impl<B> Writer<B> {
    /// Create a new `Writer` with the given internal `writer`.
    #[must_use]
    pub fn new(writer: BufWriter<B>) -> Self {
        Self {
            writer,
            ansi: false,
            ansi_seq: BytesMut::new(),
            last_seq: BytesMut::new(),
            seq_changed: false,
            char_buff: [0; 4],
        }
    }

    /// Create a new [`ansi::Writer`](`Writer`) with the given capacity for its internal buffers.
    #[must_use]
    pub fn with_capacity(writer: BufWriter<B>, cap: usize) -> Self {
        Self {
            writer,
            ansi: false,
            ansi_seq: BytesMut::with_capacity(cap),
            last_seq: BytesMut::with_capacity(cap),
            seq_changed: false,
            char_buff: [0; 4],
        }
    }
}

impl Writer<Vec<u8>> {
    /// Creates a new [`Writer`] using the given `Vec` as its output buffer.
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Writer::new(vec.writer())
    }

    /// Creates a new [`Writer`] using the given `String` as its output buffer.
    #[must_use]
    pub fn from_string(string: String) -> Self {
        Writer::from_vec(string.into_bytes())
    }

    /// **Consumes** this [`Writer`] and tries to create a string from it's internal buffer.
    ///
    /// ## Errors
    /// - `crate::Error::Utf8` if the buffer is not a valid utf8 string
    pub fn into_string(self) -> Result<String> {
        String::from_utf8(self.writer.into_inner()).map_err(Error::from)
    }
}

impl<'b> Writer<&'b mut [u8]> {
    /// Creates a new [`Writer`] using the given bytes as its output buffer.
    pub fn from_byte_slice(bytes: &'b mut [u8]) -> Self {
        Writer::new(bytes.writer())
    }
}

impl Writer<BytesMut> {
    /// Creates a new [`Writer`] using the given [`BytesMut`] as its output buffer.
    #[must_use]
    pub fn from_bytes(bytes: BytesMut) -> Self {
        Writer::new(bytes.writer())
    }
}

impl<B: BufMut> Writer<B> {
    /// **Consumes** this [`Writer`] and returns the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reflors::ansi::writer::Writer;
    /// let mut buffer = Vec::new();
    /// let writer = Writer::from_vec(buffer);
    /// assert_eq!(writer.into_inner(), vec![]);
    /// ```
    pub fn into_inner(self) -> B {
        self.writer.into_inner()
    }

    /// Write the given bytes to this [`ansi::Writer`]
    ///
    /// ## Errors
    /// - `Error::Utf8` - If the conversion from `b` to `&str` fails
    /// - `Error::Io` - If writing to any of the internal buffers fails
    pub fn write_bytes(&mut self, b: &[u8]) -> Result<usize> {
        let s = std::str::from_utf8(b)?;
        self.write_str(s)
    }

    /// Write the given string to this [`Writer`]
    ///
    /// ## Errors
    /// - `Error::Io` - If writing to any of the internal buffers fails
    pub fn write_str(&mut self, s: &str) -> Result<usize> {
        for ch in s.chars() {
            if ch == super::MARKER {
                self.ansi = true;
                self.seq_changed = true;
                self.ansi_seq.write_char(ch)?;
            } else if self.ansi {
                self.ansi_seq.write_char(ch)?;
                if super::is_terminator(ch) {
                    self.ansi = false;

                    if self.ansi_seq.ends_with(b"[0m") {
                        // reset sequence
                        self.last_seq.clear();
                        self.seq_changed = false;
                    } else if ch == 'm' {
                        // color code
                        self.last_seq.put(self.ansi_seq.as_ref());
                    }

                    self.writer.write_all(self.ansi_seq.as_ref())?;
                }
            } else {
                self.write_char(ch)?;
            }
        }

        self.flush_writer()?;
        Ok(s.len())
    }

    /// Get the string value of the internal buffer [`Writer::last_seq`].
    ///
    /// ## Errors
    /// - `Error::Utf8` - If the conversion from `[u8]` to `str` fails
    pub fn last_sequence(&self) -> Result<&str> {
        std::str::from_utf8(self.last_seq.as_ref()).map_err(Error::from)
    }

    /// Insert an ansi reset sequence into the internal buffer.
    ///
    /// ## Errors
    /// - `Error::Io` - If writing the ansi reset sequence to the internal buffer fails
    pub fn reset_ansi(&mut self) -> Result<()> {
        if !self.seq_changed {
            return Ok(());
        }

        self.writer
            .write(b"\x1b[0m")
            .map(|_| ())
            .map_err(Error::from)
    }

    /// Restore the last style used by inserting the last ansi sequence into the internal buffer.
    ///
    /// ## Errors
    /// - `Error::Io` - If writing the ansi sequence to the internal buffer fails
    pub fn restore_ansi(&mut self) -> Result<()> {
        self.writer
            .write(self.last_seq.as_ref())
            .map(|_| ())
            .map_err(Error::from)
    }

    /// Flush the internal [`Writer::writer`] buffer.
    ///
    /// ## Errors
    /// - `Error::Io` - If flushing the internal writer fails
    pub fn flush_writer(&mut self) -> Result<()> {
        self.writer.flush().map_err(Error::from)
    }

    fn write_char(&mut self, ch: char) -> Result<()> {
        let s = ch.encode_utf8(self.char_buff.as_mut());
        self.writer.write_all(s.as_ref())?;

        // I dont think it is necessary to clear the char_buffer since we are never
        //   using it directly, instead using the `&mut str` slice that is returned from
        //   `char::encode_utf8`
        // self.char_buff.copy_from_slice(&[0; 4]);

        Ok(())
    }

    // pub fn to_string(&self) -> Result<String> {
    //     String::from_utf8()
    // }
}

impl<B: BufMut + Sized> IoWrite for Writer<B> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        use std::convert::Into;
        self.write_bytes(buf).map_err(Into::into)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn basic_usage() {
        let mut vec = Vec::with_capacity(25);
        let buf_writer = vec.writer();
        let mut writer = Writer::new(buf_writer);
        assert_eq!(writer.write_str("Hello World!").unwrap(), 12);
        vec = writer.into_inner();

        assert_eq!(String::from_utf8(vec), Ok("Hello World!".to_string()));
    }

    #[test]
    fn write() {
        let text = "\x1B[38;2;249;38;114m你好reflow\x1B[0m";
        let mut buffer = bytes::BytesMut::new();
        let mut writer = Writer::new(buffer.writer());
        let result = writer.write_str(text);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), text.len());
    }

    #[test]
    fn from() {
        // Works with Vec<u8>
        let mut writer = Writer::from_vec(Vec::<u8>::new());
        // Works with &mut [u8]
        let mut bytes: [u8; 16] = [0; 16];
        let mut writer = Writer::from_byte_slice(&mut bytes[..]);
        // Works with String
        let mut writer = Writer::from_string(String::new());
        // Works with ansi::Buffer
    }
}
