//! Reflo-rs (Like re-floors I guess?)
//!
//! Mod for mod ripoff of the seemingly excellent go library [reflow](https://github.com/muesli/reflow)

// TODO At some point I should probably update this with the features I'm actually using.
#![feature(
    associated_type_defaults,
    backtrace,
    inline_const,
    concat_idents,
    crate_visibility_modifier,
    default_free_fn,
    exclusive_range_pattern,
    half_open_range_patterns,
    let_else,
    once_cell,
    test,
    try_blocks
)]
// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rustdoc::all
)]
#![allow(
    unused,
    dead_code,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::unnecessary_wraps
)]

/// # `ansi` Module
///
/// Contains constants for ansi escape code sequence start bytes, as well as functions to whether a character is a terminating char.
pub mod ansi;

/// # `dedent` Module
///
/// Contains code to un-indent text, taking ansi escape codes into account.
pub mod dedent;

/// # `indent` Module
///
/// Contains code to indent text, taking ansi escape codes into account.
pub mod indent;

/// # `margin` Module
///
/// TODO: Describe module here.
pub mod margin;

/// # `padding` Module
///
/// TODO: Describe module here.
pub mod padding;

/// # `truncate` Module
///
/// TODO: Describe module here.
pub mod truncate;

/// # `wordwrap` Module
///
/// TODO: Describe module here.
pub mod wordwrap;

/// # `wrap` Module
///
/// TODO: Describe module here.
pub mod wrap;
