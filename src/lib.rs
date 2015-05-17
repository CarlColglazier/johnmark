//!A native markdown parser for Rust with zero dependencies.

mod symbol;
mod constants;

use symbol::Symbol;
use constants::*;

#[allow(dead_code)]
struct Input {
    symbols: Vec<Symbol>,
    string: String,
}

#[allow(dead_code)]
impl Input {

    /// Create a new input.
    fn new (input: &str) -> Input {
        let mut symbols: Vec<Symbol> = Vec::new();
        for character in input.chars() {
            symbols.push(Symbol::from_char(character));
        }
        symbols.push(Symbol::EndInput);
        return Input { symbols: symbols, string: input.to_string() };
    }

    /// Check if a special character at index has been escaped by '\\'
    fn is_encaped(&self, index: usize) -> bool {
        if index > self.string.len() || index < 1 {
            return false;
        }
        if self.symbols[index - 1] == Symbol::Escape {
            return true;
        } else {
            return false;
        }
    }

    /// Check if the section has anything inside.
    fn is_blank(&self, section: &Section) -> bool {
        for i in section.start..section.end {
            if !self.symbols[i].is_blank() {
                return false;
            }
        }
        return true;
    }

    /// Check if the inputted string slice is found at the given index.
    fn check_match(&self, key: &str, index: usize) -> bool {
        if index + key.len() > self.string.len() {
            return false;
        }
        return &self.string[index..index + key.len()] == key;
    }

    /// Find the next instance of a given str.
    fn find_next(&self, key: &str, section: &Section) -> Option<usize> {
        for i in section.start..section.end {
            if self.check_match(key, i) {
                return Some(i);
            }
        }
        return None;
    }

    /// Check if a section's string cannot be directly appended as valid HTML.
    /// For example, a `&` may need to be converted to `&amp;`.
    fn has_char_entity(&self, section: &Section) -> bool {
        for i in section.start..section.end {
            if self.symbols[i].is_char_entity() {
                return true;
            }
        }
        return false;
    }

    /// Split a paragraph into new sections on the newline character.
    fn split_lines(&self, paragraph: &Section) -> Vec<Section> {
        let mut section: Vec<Section> = Vec::new();
        let mut start: usize = 0;
        let mut next: usize = 0;
        for i in paragraph.start..paragraph.end {
            if i < next {
                continue;
            } else {
                next += 1;
            }
            if self.symbols[i] == Symbol::Newline {
                section.push(Section::new(start, i));
                next += self.sequence_length("\n", i) - 1;
                start = i + self.sequence_length("\n", i);
            }
        }
        if start < paragraph.end {
            section.push(Section::new(start, paragraph.end));
        }
        return section;
    }

    /// Check if this section is a fancy header.
    /// That is, if it is formatted as so `Header\n===`.
    fn is_fancy_header(&self, section: &Section) -> bool {
        let section_slice = &self.string[section.start..section.end];
        // If there is no newline character in this section,
        // it cannot be a fancy header.
        if section_slice.split('\n').count() < 2 {
            return false;
        }
        let last_line = match section_slice.split('\n').last() {
            None => return false,
            Some(l) => l,
        };
        // There needs to be at least two characters in the last line.
        if last_line.len() < 2 {
            return false;
        }
        let search_string = match self.symbols[section.end - last_line.len()] {
            Symbol::EqualsSign => "=",
            Symbol::Hyphen => "-",
            _ => return false,
        };
        if self.sequence_length(search_string, section.end - last_line.len()) == last_line.len() {
            return true;
        }
        return false;
    }

    /// Check how many times (if any) a character is repeated.
    fn sequence_length(&self, key: &str, index: usize) -> usize {
        let mut length: usize = 0;
        loop {
            if self.check_match(key, index + length * key.len()) {
                length += 1;
            } else {
                return length;
            }
        }
    }

    /// Headers such as 'Header\n===='
    fn parse_fancy_header(&self, section: &Section) -> String {
        let mut output = String::new();
        let last_line = match self.string[section.start..section.end].split('\n').last() {
            None => "",
            Some(l) => l,
        };
        let last_line_len = last_line.len();
        let content_section = Section::new(section.start, section.end - last_line_len - 1);
        let header_text = match self.symbols[section.end - last_line_len] {
            Symbol::EqualsSign => "h1",
            Symbol::Hyphen => "h2",
            _ => "h2" // This should not ever happen.
        };
        output.push_str(HTML_START_OPEN);
        output.push_str(header_text);
        output.push_str(HTML_CLOSE);
        output.push_str(&self.section_to_string(&content_section));
        output.push_str(HTML_END_OPEN);
        output.push_str(header_text);
        output.push_str(HTML_CLOSE);
        return output;
    }

    /// Headers such as '# Header'
    fn parse_number_sign_header(&self, section: &Section) -> String {
        let mut output = String::new();
        let header_depth = self.sequence_length("#", section.start);
        let header_text = match header_depth {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            4 => "h4",
            5 => "h5",
            6 | _ => "h6",
        };
        let section_start = match self.symbols[section.start + header_depth] {
            Symbol::Space => section.start + header_depth
                + self.sequence_length(" ", section.start + header_depth),
            _ => section.start + header_depth,
        };
        // TODO: Cut off trailing content.
        let section_end: usize = 0;
        let new_section = Section::new(section_start, section.end - section_end);
        output.push_str(HTML_START_OPEN);
        output.push_str(header_text);
        output.push_str(HTML_CLOSE);
        output.push_str(&self.section_to_string(&new_section));
        output.push_str(HTML_END_OPEN);
        output.push_str(header_text);
        output.push_str(HTML_CLOSE);
        return output;
    }

    /// Regular paragraphs
    fn parse_paragraph(&self, section: &Section) -> String {
        let mut output = String::new();
        output.push_str("<p>");
        output.push_str(&self.section_to_string(section));
        output.push_str("</p>");
        return output;
    }

    /// Blockquote paragraphs
    fn parse_blockquote(&self, section: &Section) -> String {
        let mut output = String::new();
        let new_section = Section::new(section.start + 1, section.end);
        output.push_str("<blockquote>");
        output.push_str(&self.parse_paragraph(&new_section));
        output.push_str("</blockquote>");
        return output;
    }

    /// For bold or italicized text.
    fn parse_emphasis(&self, section: &Section) -> Output {
        let mut output = String::new();
        let opening_length = self.sequence_length(ASTERISK, section.start);
        let opening_tag = match opening_length {
            1 => "<em>",
            2 => "<strong>",
            3 | _ => "<strong><em>",
        };
        let closing_tag = match opening_length {
            1 => "</em>",
            2 => "</strong>",
            3 | _ => "</em></strong>",
        };
        let search_key = match opening_length {
            1 => ASTERISK,
            2 => "**",
            _ => "***",
        };
        let subsection = Section::new(section.start + opening_length, section.end);
        let next_section: usize;
        match self.find_next(search_key, &subsection) {
            None => {
                let mut index = 0;
                while index < opening_length {
                    output.push_str(ASTERISK);
                    index += 1;
                }
                output.push_str(&self.section_to_string(&subsection));
                next_section = subsection.end;
            },
            Some(n) => {
                if opening_length > 3 {
                    let mut index = 0;
                    while index < opening_length - search_key.len() {
                        output.push_str(ASTERISK);
                        index += 1;
                    }
                }
                next_section = n + search_key.len();
                let subsection = Section::new(subsection.start, n);
                output.push_str(opening_tag);
                output.push_str(&self.section_to_string(&subsection));
                output.push_str(closing_tag);
            }
        }
        return Output::new(output, next_section);
    }

    // Parse inline code.
    fn parse_code(&self, section: &Section) -> Output {
        let mut output = String::new();
        let opening_length = self.sequence_length("`", section.start);
        let subsection = Section::new(section.start + opening_length, section.end);
        let next_section: usize;
        match self.find_next(&self.string[section.start..section.start + opening_length], &subsection) {
            None => {
                output.push_str(&self.string[section.start..section.start + opening_length]);
                output.push_str(&self.section_to_string(&subsection));
                next_section = subsection.end;
            },
            Some(n) => {
                next_section = n + opening_length;
                output.push_str("<code>");
                output.push_str(&self.string[subsection.start..n]);
                output.push_str("</code>");
            }
        }
        return Output::new(output, next_section);
    }

    /// Convert a block of code, set off by an indent.
    fn parse_code_block(&self, paragraph: &Section) -> String {
        let mut output = String::new();
        output.push_str("<pre><code>");
        let mut closed: bool = false;
        let mut section_count: usize = 0;
        for section in self.split_lines(paragraph) {
            match self.symbols[section.start] {
                Symbol::Tab => {
                    if section_count > 0 {
                        output.push('\n');
                    }
                    output.push_str(&self.string[section.start + 1..section.end]);
                },
                Symbol::Space => {
                    if self.sequence_length(" ", section.start) > 3 {
                        if section_count > 0 {
                            output.push('\n');
                        }
                        output.push_str(&self.string[section.start + 4..section.end]);
                    } else {
                        output.push_str("</code></pre>");
                        closed = true;
                        let subsection = Section::new(section.start, paragraph.end);
                        output.push_str(&self.parse_paragraph(&subsection));
                        break;
                    }
                },
                _ => {
                    output.push_str("</code></pre>");
                    closed = true;
                    let subsection = Section::new(section.start, paragraph.end);
                    output.push_str(&self.parse_paragraph(&subsection));
                    break;
                }
            }
            section_count += 1;
        }
        if !closed {
            output.push_str("</code></pre>");
        }
        return output;
    }
    /*
    fn convert_char_entities(&self, section: &Section) -> String {

    }
    */
    /// Convert a section to a string.
    fn section_to_string(&self, paragraph: &Section) -> String {
        let mut output = String::new();
        let mut next: usize = paragraph.start;
        for i in paragraph.start..paragraph.end {
            if i < next {
                continue;
            } else {
                next += 1;
            }
            match self.symbols[i] {
                Symbol::Asterisk => {
                    let subsection = Section::new(i, paragraph.end);
                    let parse_output = self.parse_emphasis(&subsection);
                    output.push_str(&parse_output.string);
                    next = parse_output.offset;
                },
                Symbol::Code => {
                    let subsection = Section::new(i, paragraph.end);
                    let parse_output = self.parse_code(&subsection);
                    output.push_str(&parse_output.string);
                    next = parse_output.offset;
                },
                _ => output.push(self.string.chars().nth(i).unwrap_or(' ')),
            }
        }
        return output;
    }
}

#[test]
fn new_input() {
    let input_str = "# Header.";
    let new_input = Input::new(input_str);
    assert_eq!(10, new_input.symbols.len());
    assert_eq!(9, new_input.string.len());
}

#[test]
fn is_encaped() {
    let input_str = "\\# Header. \\*";
    let new_input = Input::new(input_str);
    assert!(new_input.is_encaped(1));
    assert!(new_input.is_encaped(12));
    assert!(!new_input.is_encaped(8));
}

#[test]
fn has_char_entity() {
    let input_str = "This & that.";
    let new_input = Input::new(input_str);
    let new_section = Section::new(0, new_input.string.len());
    assert!(new_input.has_char_entity(&new_section));
    let input_str = "This and that.";
    let new_input = Input::new(input_str);
    let new_section = Section::new(0, new_input.string.len());
    assert!(!new_input.has_char_entity(&new_section));
}

#[test]
fn check_match() {
    let input_str = "Looking for: **";
    let new_input = Input::new(input_str);
    assert!(new_input.check_match("**", 13));
    assert!(!new_input.check_match("~~", 13));
    assert!(!new_input.check_match("**", 12));
}

#[test]
fn find_next() {
    let input_str = "** Looking for: **";
    let new_input = Input::new(input_str);
    let new_section = Section::new(3, new_input.string.len());
    assert_eq!(16, new_input.find_next("**", &new_section).unwrap());
}

#[test]
fn split_lines() {
    let input_str = "\tLine 1\nLine 2";
    let new_input = Input::new(input_str);
    let new_section = Section::new(3, new_input.string.len());
    assert_eq!(2, new_input.split_lines(&new_section).len());
}

#[test]
fn sequence_length() {
    let input_str = "Looking for: ******";
    let new_input = Input::new(input_str);
    assert_eq!(3, new_input.sequence_length("**", 13));
}

#[test]
fn is_fancy_header() {
    // Correct formatting.
    let input_str = "Header\n======";
    let new_input = Input::new(input_str);
    let paragraph = Section::new(0, input_str.len());
    assert!(new_input.is_fancy_header(&paragraph));
    // No newline. Should return false.
    let input_str = "Header ======";
    let new_input = Input::new(input_str);
    let paragraph = Section::new(0, input_str.len());
    assert!(!new_input.is_fancy_header(&paragraph));
    // Incorrect characters.
    let input_str = "Header\ngggggg";
    let new_input = Input::new(input_str);
    let paragraph = Section::new(0, input_str.len());
    assert!(!new_input.is_fancy_header(&paragraph));
    // Not a complete sequence.
    let input_str = "Header\n=====not==";
    let new_input = Input::new(input_str);
    let paragraph = Section::new(0, input_str.len());
    assert!(!new_input.is_fancy_header(&paragraph));
}

// In markdown, the input can easily be reduced down to small blocks.
// These blocks are separated be two newline characters.
#[allow(dead_code)]
#[derive(PartialEq)]
struct Section {
    start: usize,
    end: usize,
}

#[allow(dead_code)]
impl Section {
    fn new(start: usize, end: usize) -> Section {
        return Section { start: start, end: end };
    }

    fn from_input(input: &Input) -> Vec<Section> {
        let mut sections: Vec<Section> = Vec::new();
        let mut start: usize = 0;
        for i in 0..input.symbols.len() - 1 {
            if input.symbols[i] == Symbol::Newline && input.symbols[i + 1] == Symbol::Newline {
                sections.push(Section::new(start, i));
                start = i + 2;
            }
        }
        sections.push(Section::new(start, input.string.len()));
        return sections;
    }
}

#[test]
fn paragraph_from_input() {
    let input_str = "# Header.\n\nContent.";
    let new_input = Input::new(input_str);
    let paragraphs = Section::from_input(&new_input);
    assert_eq!(2, paragraphs.len());
    assert_eq!(0, paragraphs[0].start);
    assert_eq!(9, paragraphs[0].end);
    assert_eq!(11, paragraphs[1].start);
    assert_eq!(19, paragraphs[1].end);
}

#[allow(dead_code)]
struct Parser {
    input: Input,
    paragraphs: Vec<Section>,
}

#[allow(dead_code)]
impl Parser {
    fn new(input: &str) -> Parser {
        let new_input = Input::new(input);
        let paragraphs = Section::from_input(&new_input);
        return Parser { input: new_input, paragraphs: paragraphs };
    }
    fn convert(&self) -> String {
        let mut output = String::new();
        for paragraph in self.paragraphs.iter() {
            // First, check if this is a fancy header.
            // This needs to be its own process because it is not dependent on
            // the first character of the section.
            if self.input.is_fancy_header(paragraph) {
                output.push_str(&self.input.parse_fancy_header(paragraph));
                continue;
            }

            // Be sure to skip blank paragraphs.
            // They are not helping anyone.
            if self.input.is_blank(paragraph) {
                continue;
            }

            // Otherwise, let us figure out what kind of section this is.
            match self.input.symbols[paragraph.start] {
                Symbol::NumberSign => {
                    output.push_str(&self.input.parse_number_sign_header(paragraph));
                },
                // Inline HTML
                Symbol::LessThan => {
                    match self.input.find_next(HTML_CLOSE, paragraph) {
                        None => output.push_str(&self.input.parse_paragraph(paragraph)),
                        Some(_) => output.push_str(&self.input.string[paragraph.start..paragraph.end]),
                    }
                },
                // Code blocks.
                Symbol::Tab => {
                    output.push_str(&self.input.parse_code_block(&paragraph));
                },
                // Sometimes there are extra spaces that simply do not need
                // to be there. Other times, there four spaces are used to
                // indicate a code block.
                Symbol::Space => {
                    let spaces_number = self.input.sequence_length(" ", paragraph.start);
                    if  spaces_number > 3 {
                        // Code block.
                        output.push_str(&self.input.parse_code_block(&paragraph));
                    } else {
                        // Just some extra spaces.
                        let subsection = Section::new(paragraph.start + spaces_number, paragraph.end);
                        output.push_str(&self.input.parse_paragraph(&subsection));
                    }
                },
                Symbol::GreaterThan => {
                    output.push_str(&self.input.parse_blockquote(paragraph));
                }
                _ => output.push_str(&self.input.parse_paragraph(paragraph)),
            }
        }
        return output;
    }
}

#[allow(dead_code)]
struct Output {
    string: String,
    offset: usize,
}

#[allow(dead_code)]
impl Output {
    fn new(string: String, offset: usize) -> Output {
        return Output{ string: string, offset: offset };
    }
    fn from_str(string: &str, offset: usize) -> Output {
        return Output::new(string.to_string(), offset);
    }
}

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
