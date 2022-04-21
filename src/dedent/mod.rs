use std::io::Write;

use bytes::BufMut;

use crate::{Error, Result};

/// Detects the indentation level that is shared amongst all lines in the
/// given text and removes it.
///
/// ## Errors
pub fn dedent_str(input: &str) -> Result<String> {
    let indent = detect_indent(input)?;
    if indent == 0 {
        return Ok(input.to_string());
    }

    dedent_impl(input, indent)
}

/// Detects the indentation level that is shared amongst all lines in the
/// given text and removes it.
///
/// ## Errors
pub fn dedent_string(input: String) -> Result<String> {
    let indent = detect_indent(input.as_str())?;
    if indent == 0 {
        return Ok(input);
    }

    dedent_impl(&input, indent)
}

/// Detects the indentation level that is shared amongst all lines in the
/// given text and removes it.
///
/// ## Errors
pub fn dedent_in_place(input: &mut String) -> Result<()> {
    let indent = detect_indent(input.as_str())?;
    if indent == 0 {
        return Ok(());
    }

    *input = dedent_impl(input, indent)?;
    Ok(())
}

/// Detects indentation level, removes it, and writes the result to the buffer given.
///
/// ## Errors
/// - `crate::Error::Io(std::io::Error)` if an I/O error occurs.
pub fn dedent_to<W: std::io::Write>(input: &str, out: &mut W) -> Result<()> {
    let output = dedent_str(input)?;
    out.write_all(output.as_bytes())?;
    Ok(())
}

fn detect_indent(input: &str) -> Result<usize> {
    let mut current = 0usize;
    let mut min = 0usize;
    let mut append = true;

    for ch in input.chars() {
        match ch {
            ' ' | '\t' => {
                if append {
                    current += 1;
                }
            }
            '\n' => {
                current = 0;
                append = true;
            }
            _ => {
                if current > 0 && (min == 0 || current < min) {
                    min = current;
                    current = 0;
                }
                append = false;
            }
        }
    }

    Ok(min)
}

fn dedent_impl(input: &str, indent: usize) -> Result<String> {
    use std::io::Write;

    let mut omitted = 0usize;
    let mut omit = true;
    let mut buf = String::with_capacity(input.len());

    for ch in input.chars() {
        match ch {
            ' ' | '\t' => {
                if omit {
                    if omitted < indent {
                        omitted += 1;
                        continue;
                    }
                    omit = false;
                }
                buf.push(ch);
            }
            '\n' => {
                omitted = 0;
                omit = true;
                buf.push(ch);
            }
            _ => buf.push(ch),
        }
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_spaces() {
        const TEXT: &str = r#"    Blah blah blah blah blah blah blah blah 
     blah blah blah blah blah blah 
      blah blah blah blah blah blah blah blah 
    blah blah blah blah blah 
        blah blah blah blah blah blah "#;
        let output = dedent_str(TEXT).unwrap();
        let output2 = dedent_string(TEXT.to_string()).unwrap();
        assert_eq!(
            output,
            r#"Blah blah blah blah blah blah blah blah 
 blah blah blah blah blah blah 
  blah blah blah blah blah blah blah blah 
blah blah blah blah blah 
    blah blah blah blah blah blah "#
        );
        assert_eq!(
            output2,
            r#"Blah blah blah blah blah blah blah blah 
 blah blah blah blah blah blah 
  blah blah blah blah blah blah blah blah 
blah blah blah blah blah 
    blah blah blah blah blah blah "#
        );
        assert_eq!(output, output2);
    }

    #[test]
    fn basic_tabs() {
        #[rustfmt::skip]
        const TEXT: &str = "\tBlah blah blah blah blah blah blah blah\n\tblah blah blah blah blah blah\n\tblah blah blah blah blah blah blah blah\n\tblah blah blah blah blah\n\tblah blah blah blah blah blah";
        const EXPECTED: &str = "Blah blah blah blah blah blah blah blah\nblah blah blah blah blah blah\nblah blah blah blah blah blah blah blah\nblah blah blah blah blah\nblah blah blah blah blah blah";
        let output = dedent_str(TEXT).unwrap();
        let output2 = dedent_string(TEXT.to_string()).unwrap();
        assert_eq!(output, EXPECTED);
        assert_eq!(output2, EXPECTED);
        assert_eq!(output, output2);
    }
}
