// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::{buf::Writer as BufWriter, BufMut, Bytes, BytesMut};

use crate::{
    ansi::{self, buffer::Buffer as AnsiBuffer, writer::Writer as AnsiWriter},
    Error, Result,
};

pub struct Writer {
    width: usize,
    tail: String,
    ansi_writer: AnsiWriter<BytesMut>,
    buf: AnsiBuffer,
    is_ansi: bool,
}

impl Writer {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            tail: String::from("..."),
            ansi_writer: AnsiWriter::new(BytesMut::new().writer()),
            buf: AnsiBuffer::new(),
            is_ansi: false,
        }
    }

    pub fn with_ending(width: usize, tail: &str) -> Self {
        Self {
            width,
            tail: tail.to_string(),
            ansi_writer: AnsiWriter::new(BytesMut::new().writer()),
            buf: AnsiBuffer::new(),
            is_ansi: false,
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize> {
        let s = std::str::from_utf8(bytes)?;
        self.write_str(s)
    }

    pub fn write_str(&mut self, s: &str) -> Result<usize> {
        let tw = ansi::visible_width(self.tail.as_str());
        if self.width < tw {
            self.buf.push_str(self.tail.as_str());
            return Ok(tw);
        }

        let target_width = self.width - tw;
        let mut current = 0;
        for ch in s.chars() {}

        Ok(0)
    }
}

pub fn truncate_string(input: &str, width: usize, ending: &str) -> Result<String> {
    let mut writer = Writer::with_ending(width, ending);
    todo!();
}
