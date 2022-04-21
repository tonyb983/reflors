// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// How text can be indented, spaces (correct) vs tabs (incorrect).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LineEnding {
    /// POSIX-style line endings, '\n'
    Newline,
    /// Windows-style line endings, '\r\n'
    CarriageReturn,
}

impl LineEnding {
    /// Create a windows-style [`LineEnding`].
    #[must_use]
    pub const fn windows() -> Self {
        LineEnding::CarriageReturn
    }

    /// Create a posix-style [`LineEnding`].
    #[must_use]
    pub const fn posix() -> Self {
        LineEnding::Newline
    }

    /// Detect the [`LineEnding`] from the given `input`.
    #[must_use]
    pub fn detect(input: &str) -> Self {
        if input.contains("\r\n") {
            LineEnding::CarriageReturn
        } else {
            LineEnding::Newline
        }
    }

    /// Get a string containing the actual [`LineEnding`].
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Newline => "\n",
            LineEnding::CarriageReturn => "\r\n",
        }
    }
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Newline
    }
}

/// How text can be indented, spaces (correct) vs tabs (incorrect).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndentStyle {
    /// Indent text with spaces.
    Spaces,
    /// Indent text with tabs.
    Tabs,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle::Spaces
    }
}

impl IndentStyle {
    /// Gets the character representing this indentation style (aka ' ' or '\t').
    #[must_use]
    pub const fn as_char(&self) -> char {
        match self {
            IndentStyle::Spaces => ' ',
            IndentStyle::Tabs => '\t',
        }
    }

    /// Gets the string representing this indentation style (aka " " or "\t").
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            IndentStyle::Spaces => " ",
            IndentStyle::Tabs => "\t",
        }
    }
}

/// Options for determining how text should be indented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndentOptions {
    /// Whether to indent text with spaces or tabs.
    style: IndentStyle,
    /// How much should text be indented.
    number: usize,
    /// The type of line ending the output should be rejoined with. If none, they will be detected from the input.
    line_endings: Option<LineEnding>,
}

impl Default for IndentOptions {
    fn default() -> Self {
        let style = IndentStyle::default();
        let number = match style {
            IndentStyle::Spaces => 4,
            IndentStyle::Tabs => 1,
        };
        IndentOptions {
            style,
            number,
            line_endings: None,
        }
    }
}

impl IndentOptions {
    /// Creates an [`IndentOptions`] representing 4 spaces.
    #[must_use]
    pub const fn four_spaces() -> Self {
        Self::spaces(4)
    }

    /// Creates an [`IndentOptions`] representing 2 spaces.
    #[must_use]
    pub const fn two_spaces() -> Self {
        Self::spaces(2)
    }

    /// Creates an [`IndentOptions`] representing 1 tab.
    #[must_use]
    pub const fn one_tab() -> Self {
        Self::tabs(1)
    }
}

impl IndentOptions {
    /// Creates a new `IndentOptions` with the given indentation style and
    /// number of spaces.
    #[must_use]
    pub const fn new(style: IndentStyle, number: usize, line_endings: Option<LineEnding>) -> Self {
        Self {
            style,
            number,
            line_endings,
        }
    }

    /// Create an [`IndentOptions`] using spaces, auto-detected line endings, and the given indentation amount.
    #[must_use]
    pub const fn spaces(number: usize) -> Self {
        Self {
            style: IndentStyle::Spaces,
            number,
            line_endings: None,
        }
    }

    /// Create an [`IndentOptions`] using tabs, auto-detected line endings, and the given indentation amount.
    #[must_use]
    pub const fn tabs(number: usize) -> Self {
        Self {
            style: IndentStyle::Tabs,
            number,
            line_endings: None,
        }
    }
}

impl IndentOptions {
    /// Gets the [`IndentStyle`].
    #[must_use]
    pub const fn style(&self) -> IndentStyle {
        self.style
    }

    /// Gets the number / level of indentation.
    #[must_use]
    pub const fn number(&self) -> usize {
        self.number
    }

    /// Gets the line endings output should be joined with.
    #[must_use]
    pub const fn line_endings(&self) -> Option<LineEnding> {
        self.line_endings
    }

    /// Creates a [`String`] representing the full indentation.
    #[must_use]
    pub fn make_indent(&self) -> String {
        self.style.as_str().repeat(self.number)
    }

    /// Get the appropriate line ending string for these options and the given input.
    #[must_use]
    pub fn get_line_ending(&self, input: &str) -> &str {
        match self.line_endings {
            Some(line_ending) => line_ending.as_str(),
            None => LineEnding::detect(input).as_str(),
        }
    }

    /// Indents the given line using the options stored in this [`IndentOptions`]. If the given
    /// line already starts with the correct indentation, it is returned unmodified.
    #[must_use]
    pub fn indent_line(&self, input: &str) -> String {
        let mut output = input.to_string();
        let indent = self.make_indent();
        if output.starts_with(&indent) {
            output
        } else {
            format!("{}{}", indent, output)
        }
    }

    /// Indents the given line using the options stored in this [`IndentOptions`]. This version
    /// does not check whether the line already starts with the correct indentation, but rather
    /// blindly applies it.
    #[must_use]
    pub fn indent_line_unchecked(&self, input: &str) -> String {
        let indent = self.make_indent();
        format!("{}{}", indent, input)
    }
}
