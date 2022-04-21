// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::Write;

use bytes::{buf::Writer as BufWriter, BufMut, BytesMut};

use crate::{
    ansi::{self, writer::Writer as AnsiWriter},
    Error, Result,
};

pub struct Writer<B> {
    padding: usize,
    ansi_writer: AnsiWriter<B>,
    buf: BytesMut,
    cache: BytesMut,
    line_len: usize,
    in_ansi: bool,
    char_buffer: [u8; 4],
}

impl Writer<BytesMut> {
    pub fn new(padding: usize) -> Self {
        Self {
            padding,
            ansi_writer: AnsiWriter::new(BytesMut::new().writer()),
            buf: BytesMut::new(),
            cache: BytesMut::new(),
            line_len: 0,
            in_ansi: false,
            char_buffer: [0; 4],
        }
    }

    pub fn with_capacity(padding: usize, cap: usize) -> Self {
        Self {
            padding,
            ansi_writer: AnsiWriter::new(BytesMut::with_capacity(cap).writer()),
            buf: BytesMut::with_capacity(cap),
            cache: BytesMut::with_capacity(cap),
            line_len: 0,
            in_ansi: false,
            char_buffer: [0; 4],
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize> {
        // TODO: Remove unwrap, make Error/Result
        let s = std::str::from_utf8(bytes).unwrap();
        self.write_str(s)
    }

    pub fn write_str(&mut self, s: &str) -> Result<usize> {
        for ch in s.chars() {
            if ch == ansi::MARKER {
                self.in_ansi = true;
            } else if self.in_ansi {
                if ansi::is_terminator(ch) {
                    self.in_ansi = false;
                }
            } else {
                self.line_len += char::len_utf8(ch);
                if ch == '\n' {
                    self.pad()?;
                    self.ansi_writer.reset_ansi()?;
                    self.line_len = 0;
                }
            }

            let encoded = ch.encode_utf8(&mut self.char_buffer);
            let _ = self.ansi_writer.write_str(encoded)?;
        }

        Ok(0)
    }

    fn pad(&mut self) -> Result<()> {
        if self.padding > 0 && self.line_len < self.padding {
            self.ansi_writer
                .write_str(" ".repeat(self.padding - self.line_len).as_str())?;
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.flush()
    }

    fn flush(&mut self) -> Result<()> {
        use bytes::{Buf, BufMut};
        use std::io::{Read, Write};
        if self.line_len != 0 {
            self.pad()?;
        }

        self.cache.clear();
        self.cache.resize(self.buf.len(), 0);
        self.cache.copy_from_slice(&self.buf);
        self.line_len = 0;
        self.in_ansi = false;
        Ok(())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.cache.to_vec()
    }

    pub fn to_string(&self) -> Result<String> {
        String::from_utf8(self.cache.to_vec()).map_err(Error::from)
    }
}

impl<B> Writer<B> {
    pub fn new_piped(width: usize, buffer: BufWriter<B>) -> Self {
        Writer {
            padding: width,
            ansi_writer: AnsiWriter::new(buffer),
            buf: BytesMut::new(),
            cache: BytesMut::new(),
            line_len: 0,
            in_ansi: false,
            char_buffer: [0; 4],
        }
    }
}

pub fn pad_bytes(bytes: &[u8], padding: usize) -> Result<Vec<u8>> {
    let mut writer = Writer::new(padding);
    writer.write_bytes(bytes)?;
    writer.close()?;
    Ok(writer.to_vec())
}

pub fn pad_string(string: &str, padding: usize) -> Result<String> {
    let mut writer = Writer::new(padding);
    writer.write_str(string)?;
    writer.close()?;
    writer.to_string()
}
