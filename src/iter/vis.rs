// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ansi;

/// An iterator over the **visible** characters in a string.
///
/// Hence it is a Vis(ible-It)erator! I'm such a word ...guy.
pub struct Viserator<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    pos: usize,
    in_ansi: bool,
}

impl<'a> Viserator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            pos: 0,
            in_ansi: false,
        }
    }

    // pub fn new_at(input: &'a str, pos: usize) -> Self {
    //     Self { input, pos }
    // }
}

impl<'a> Iterator for Viserator<'a> {
    type Item = char;

    #[allow(clippy::while_let_on_iterator)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ch) = self.chars.next() {
            if ch == ansi::MARKER {
                self.in_ansi = true;
            } else if self.in_ansi {
                if ansi::is_terminator(ch) {
                    self.in_ansi = false;
                }
            } else {
                return Some(ch);
            }
        }

        None
    }
}
