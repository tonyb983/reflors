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
    // This is a library so there's going to be a lot of unused
    unused,
    // I will remove this later on, but for now it's less pointlessly annoying
    dead_code,
    // I hate this lint
    clippy::module_inception,
    // I also hate this lint
    clippy::module_name_repetitions,
    // I am undecided on this lint
    clippy::unnecessary_wraps
)]

mod error;
pub use error::{Error, Result};

/// # `ansi` Module
///
/// Contains constants for ansi escape code sequence start bytes, as well as functions to whether a character is a terminating char.
///
/// #### Status: Mostly complete, need to make a decision about [`Writer`](`crate::ansi::writer::Writer`) and the best way to handle it.
///
/// Source: [muesli/reflow/ansi](https://github.com/muesli/reflow/ansi/)
pub mod ansi;

/// # `dedent` Module
///
/// Contains code to un-indent text, taking ansi escape codes into account.
///
/// #### Status: Complete, maybe add more tests to make sure it works on all inputs and weed out any edge cases.
///
/// Source: [muesli/reflow/dedent](https://github.com/muesli/reflow/dedent/)
pub mod dedent;

/// # `indent` Module
///
/// Contains code to indent text.
///
/// #### Status: Mostly complete, except for the below item.
/// TODO: The original golang version has indentation implemented as another `Writer`, but I'm not sure if that's necessary.
///
/// Source: [muesli/reflow/indent](https://github.com/muesli/reflow/indent/)
pub mod indent;

/// # `margin` Module
///
/// TODO: Describe module here.
///
/// Source: [muesli/reflow/margin](https://github.com/muesli/reflow/margin/)
pub mod margin;

/// # `padding` Module
///
/// TODO: Describe module here.
///
/// Source: [muesli/reflow/padding](https://github.com/muesli/reflow/padding/)
pub mod padding;

/// # `truncate` Module
///
/// TODO: Describe module here.
///
/// Source: [muesli/reflow/truncate](https://github.com/muesli/reflow/truncate/)
pub mod truncate;

/// # `wordwrap` Module
///
/// TODO: Describe module here.
///
/// Source: [muesli/reflow/wordwrap](https://github.com/muesli/reflow/wordwrap/)
pub mod wordwrap;

/// # `wrap` Module
///
/// TODO: Describe module here.
///
/// Source: [muesli/reflow/wrap](https://github.com/muesli/reflow/wrap/)
pub mod wrap;
