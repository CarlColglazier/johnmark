//!A native markdown parser for Rust with zero dependencies.

mod symbol;
mod content;
mod section;
mod paragraph;
mod line;

use content::Content;

/// Convert a string of markdown to HTML.
///
/// # Examples
///
/// ```
/// let input_str = "Header\n======\n\nContent";
/// assert_eq!("<h1>Header</h1><p>Content</p>", johnmark::convert(input_str));
/// ```
pub fn convert(input: &str) -> String {
    let parser = Content::from_str(input);
    return parser.convert();
}
