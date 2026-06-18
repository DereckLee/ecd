//! ecd — fast CLI for detecting text file character encodings.
//!
//! See the [repository](https://github.com/dereck/ecd) for usage and supported encodings.

pub mod cli;
pub mod detect;
pub mod encodings;
pub mod error;
pub mod output;
pub mod walk;

pub use cli::Cli;
