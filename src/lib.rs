//!A native markdown parser for Rust with zero dependencies.

mod symbol;
mod constants;
mod input;
mod section;
mod output;
mod parser;

use symbol::Symbol;
use constants::*;
use input::Input;
use section::Section;
use output::Output;
use parser::Parser;

/// Convert a string of markdown to HTML.
///
/// # Examples
///
/// ```
/// let input_str = "Header\n======\n\nContent";
/// assert_eq!("<h1>Header</h1><p>Content</p>", johnmark::convert(input_str));
/// ```
pub fn convert(input: &str) -> String {
    let parser = Parser::new(input);
    return parser.convert();
}
