// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;
pub use options::{IndentOptions, IndentStyle, LineEnding};

/// Indent the given `input` string using the given [`options`](`IndentOptions`). If `checked`
/// is true, each line will be checked to see if it already contains the correct indentation,
/// and if so it will be returned unmodified, otherwise it will be indented regardless.
///
/// # Examples
/// ```
/// # use reflors::indent::{indent_str, IndentOptions};
/// let text = "One\nTwo\nThree";
/// let one_tab = indent_str(text, IndentOptions::one_tab(), false);
/// let two_spaces = indent_str(text, IndentOptions::two_spaces(), true);
/// let four_spaces = indent_str(text, IndentOptions::four_spaces(), false);
/// assert_eq!(one_tab, "\tOne\n\tTwo\n\tThree");
/// assert_eq!(two_spaces, "  One\n  Two\n  Three");
/// assert_eq!(four_spaces, "    One\n    Two\n    Three");
/// ```
#[must_use]
pub fn indent_str(input: &str, options: IndentOptions, checked: bool) -> String {
    let sep = "\n";
    let output = input
        .lines()
        .map(|line| {
            if checked {
                options.indent_line(line) + sep
            } else {
                options.indent_line_unchecked(line) + sep
            }
        })
        .collect::<String>();
    output[..output.len() - 1].to_string()
}

/// Second attempt at the `indent_str` function. This one is already looking much better, initial
/// testing shows that it is not only faster than both the checked and unchecked version of the
/// original, but it seems to also be faster than the in-place version as well. Unsure of how that
/// could possibly be the case?
#[must_use]
pub fn indent_str_v2(input: &str, options: IndentOptions) -> String {
    let indent = options.make_indent();
    let mut output = String::with_capacity(input.len() * 2);
    output.push_str(&indent);
    for ch in input.chars() {
        output.push(ch);
        if ch == '\n' {
            output.push_str(&indent);
        }
    }
    output
}

/// Indent the given `input` string using the given [`options`](`IndentOptions`) in place.
///
/// # Examples
/// ```
/// # use reflors::indent::{indent_str, IndentOptions};
/// let mut text = "One\nTwo\nThree".to_string();
/// indent_in_place(&mut text, IndentOptions::one_tab());
/// assert_eq!(text, "\tOne\n\tTwo\n\tThree");
/// ```
pub fn indent_in_place(input: &mut String, options: IndentOptions) {
    let indent = options.make_indent();
    input.insert_str(0, indent.as_str());
    // let replacement = format!("\n{}", indent);
    // input.replace('\n', replacement.as_str());
    let breaks = input
        .match_indices('\n')
        .map(|(i, s)| i)
        .rev()
        .collect::<Vec<_>>();
    input.reserve(breaks.len() * indent.len());
    for i in breaks {
        input.insert_str(i + 1, indent.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn basic_usage() {
        const INPUT: &str = "One\nTwo\nThree\nFour\nFive";
        let one_tab = IndentOptions::one_tab();
        let two_spaces = IndentOptions::two_spaces();
        let four_spaces = IndentOptions::four_spaces();
        let one_tab_result = indent_str(INPUT, one_tab, false);
        let two_spaces_result = indent_str(INPUT, two_spaces, true);
        let four_spaces_result = indent_str(INPUT, four_spaces, false);
        assert_eq!(one_tab_result, "\tOne\n\tTwo\n\tThree\n\tFour\n\tFive");
        assert_eq!(two_spaces_result, "  One\n  Two\n  Three\n  Four\n  Five");
        assert_eq!(
            four_spaces_result,
            "    One\n    Two\n    Three\n    Four\n    Five"
        );
    }

    #[test]
    fn in_place() {
        let mut text = "One\nTwo\nThree".to_string();
        indent_in_place(&mut text, IndentOptions::one_tab());
        assert_eq!(text, "\tOne\n\tTwo\n\tThree");
    }

    /// Initial testing gives these results:
    ///
    /// Using 100000 iterations...
    /// - `indent_str   (checked)` took 611.1361ms (6.111µs average)
    /// - `indent_str (unchecked)` took 476.1627ms (4.761µs average)
    /// - `indent_in_place       ` took 174.7235ms (1.747µs average)
    /// - `indent_str_v2         ` took 160.4168ms (1.604µs average)
    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn compare() {
        use std::string::ToString;
        use std::time::{Duration, Instant};
        const ITERS: usize = 100_000;
        const TEXT: &str = "One\nTwo\nThree";
        const EXPECTED: &str = "\tOne\n\tTwo\n\tThree";

        let strs = vec![TEXT; ITERS];
        let mut strings = strs.iter().map(ToString::to_string).collect::<Vec<_>>();
        let mut out_strs = vec![""; ITERS]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let opt = IndentOptions::one_tab();

        // ===================================================================================
        // indent_str (unchecked)
        let now = Instant::now();
        for i in 0..ITERS {
            out_strs[i] = indent_str(strs[i], opt, false);
        }
        let unchecked_elapsed = now.elapsed();

        for (i, text) in out_strs.iter().enumerate() {
            assert_eq!(
                text, EXPECTED,
                "unchecked: failure on index {} with text '{}'",
                i, text
            );
        }

        // ===================================================================================
        // indent_in_place
        let now = Instant::now();
        for s in &mut strings {
            indent_in_place(s, opt);
        }
        let in_place_elapsed = now.elapsed();

        for (i, text) in strings.iter().enumerate() {
            assert_eq!(
                text, EXPECTED,
                "in place: failure on index {} with text '{}'",
                i, text
            );
        }

        // reset output vector
        for s in &mut out_strs {
            *s = "".to_string();
        }

        // ===================================================================================
        // indent_str (checked)
        let now = Instant::now();
        for i in 0..ITERS {
            out_strs[i] = indent_str(strs[i], opt, true);
        }
        let checked_elapsed = now.elapsed();

        for (i, text) in out_strs.iter().enumerate() {
            assert_eq!(
                text, EXPECTED,
                "checked: failure on index {} with text '{}'",
                i, text
            );
        }

        // reset output vector
        for s in &mut out_strs {
            *s = "".to_string();
        }

        // ===================================================================================
        // indent_str_v2
        let now = Instant::now();
        for i in 0..ITERS {
            out_strs[i] = indent_str_v2(strs[i], opt);
        }
        let v2_elapsed = now.elapsed();

        for (i, text) in out_strs.iter().enumerate() {
            assert_eq!(
                text, EXPECTED,
                "checked: failure on index {} with text '{}'",
                i, text
            );
        }

        let checked_ave = checked_elapsed / ITERS as u32;
        let unchecked_ave = unchecked_elapsed / ITERS as u32;
        let in_place_ave = in_place_elapsed / ITERS as u32;
        let v2_ave = v2_elapsed / ITERS as u32;

        println!("Using {} iterations...", ITERS);
        println!(
            "\tindent_str   (checked) took {:?} ({:?} average)",
            checked_elapsed, checked_ave
        );
        println!(
            "\tindent_str (unchecked) took {:?} ({:?} average)",
            unchecked_elapsed, unchecked_ave
        );
        println!(
            "\tindent_in_place        took {:?} ({:?} average)",
            in_place_elapsed, in_place_ave
        );
        println!(
            "\tindent_str_v2          took {:?} ({:?} average)",
            v2_elapsed, v2_ave
        );
    }
}
