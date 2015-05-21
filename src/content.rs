use section::Section;
use symbol::Symbol;
use paragraph::Paragraph;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum LineType {
    Blank,
    Paragraph,
    Header,
    Blockquote,
    Code,
    Null,
}

#[allow(dead_code)]
pub struct Content {
    pub string: String,
    pub symbols: Vec<Symbol>,
}

#[allow(dead_code)]
impl Content {

    /// INITIALIZATION //

    /// Create a new input.
    pub fn from_str(input: &str) -> Content {
        let string = input.to_string();
        let symbols = Symbol::from_str(input);
        return Content { symbols: symbols, string: string };
    }

    // PUBLIC FUNCTIONS //

    pub fn convert(&self) -> String {
        let mut output = String::new();
        for paragraph in self.sections().iter() {
            match paragraph.kind {
                LineType::Blockquote => {
                    output.push_str("<blockquote>");
                    for i in Section::to_index(&paragraph.lines) {
                        match self.string.chars().nth(i) {
                            None => continue,
                            Some(s) => output.push(s),
                        }
                    }
                    output.push_str("</blockquote>");
                },
                LineType::Code => {
                    output.push_str("<pre><code>");
                    for i in Section::to_index(&paragraph.lines) {
                        match self.string.chars().nth(i) {
                            None => continue,
                            Some(s) => output.push(s),
                        }
                    }
                    output.push_str("</code></pre>");
                },
                LineType::Paragraph => {
                    let mut line_slice: &[Section] = &paragraph.lines[..];

                    // Check for fancy headers.
                    if paragraph.lines.len() > 1 {
                        let start_index = paragraph.lines[1].start;

                        // 'Foo\n=='
                        if self.symbols[start_index] == Symbol::EqualsSign {
                            let seq_len = self.sequence_length(Symbol::EqualsSign, start_index);
                            if seq_len > 1 && seq_len == paragraph.lines[1].end - paragraph.lines[1].start {
                                output.push_str("<h1>");
                                output.push_str(&self.string[paragraph.lines[0].start..paragraph.lines[0].end]);
                                output.push_str("</h1>");
                                if paragraph.lines.len() > 2 {
                                    line_slice = &paragraph.lines[2..paragraph.lines.len()];
                                } else {
                                    continue;
                                }
                            }

                        // 'Foo\n--'
                        } else if self.symbols[start_index] == Symbol::Hyphen {
                            let seq_len = self.sequence_length(Symbol::Hyphen, start_index);
                            if seq_len > 1 && seq_len == paragraph.lines[1].end - paragraph.lines[1].start {
                                output.push_str("<h2>");
                                output.push_str(&self.string[paragraph.lines[0].start..paragraph.lines[0].end]);
                                output.push_str("</h2>");
                                if paragraph.lines.len() > 2 {
                                    line_slice = &paragraph.lines[2..paragraph.lines.len()];
                                } else {
                                    continue;
                                }
                            }
                        }
                    }
                    output.push_str(&self.convert_paragraph(line_slice));
                }
                LineType::Header => {
                    if paragraph.lines.len() > 1 {
                        panic!("Header contains more than one line.")
                    }
                    output.push_str(&self.convert_header(&paragraph.lines[0]));
                },

                // This should not happen.
                _ => continue,
            }
        }
        return output;
    }

    // CONVERSION FUNCTIONS //

    fn parse_section(&self, indexes: &[usize]) -> String {
        let mut output = String::new();
        let mut next = indexes[0];
        for index in indexes {
            let i = *index;
            if i < next {
                continue;
            } else {
                next += 1;
            }
            match self.symbols[i] {

                // Emphasis characters.
                Symbol::Asterisk => {
                    let search_symbols: &[Symbol];
                    let length = self.sequence_length(Symbol::Asterisk, i);
                    if length > 3 {
                        search_symbols = &self.symbols[i..i + 3];
                    } else {
                        search_symbols = &self.symbols[i..i + length];
                    }

                    // Emphasis characters need to be attached to something.
                    // For example `*foo*` is valid; however, `* foo *` is not.
                    if self.symbols[i + length].is_blank() {
                        output.push('*');
                    } else {
                        match self.find_next_slice(search_symbols, i + length) {
                            None => {
                                let mut index = 0;
                                while index < length {
                                    output.push('*');
                                    index += 1;
                                }
                                next = i + length;
                            },
                            Some(int) => {
                                if length > 3 {
                                    let mut index = 0;
                                    while index < length - 3 {
                                        output.push('*');
                                        index += 1;
                                    }
                                }
                                if self.symbols[int - 1].is_blank() {
                                    output.push('*')
                                } else {
                                    match length {
                                        1 => output.push_str("<em>"),
                                        2 => output.push_str("<strong>"),
                                        _ => output.push_str("<strong><em>"),
                                    }

                                    // TODO: Check this section for code blocks.
                                    let subsection = &indexes[i + length..int];
                                    output.push_str(&self.parse_section(subsection));
                                    match length {
                                        1 => output.push_str("</em>"),
                                        2 => output.push_str("</strong>"),
                                        _ => output.push_str("</em></strong>"),
                                    }
                                    next = int + search_symbols.len();
                                }
                            },
                        }
                    }
                },

                // Inlines code such as '`code`' can only be broken by an equal number of
                // backtick characters.
                Symbol::Code => {
                    let length = self.sequence_length(Symbol::Code, i);
                    let search_symbols = &self.symbols[i..i + length];
                    match self.find_next_slice(search_symbols, i + length) {
                        None => {
                            let mut index = 0;
                            while index < length {
                                output.push('`');
                                index += 1;
                            }
                            next = i + length;
                        },
                        Some(int) => {
                            output.push_str("<code>");
                            output.push_str(&self.string[i + length..int]);
                            output.push_str("</code>");
                            next = int + search_symbols.len();
                        },
                    }
                },
                _ => {
                    match self.string.chars().nth(i) {
                        None => continue,
                        Some(s) => output.push(s),
                    }
                }
            }
        }
        return output;
    }

    fn convert_paragraph(&self, lines: &[Section]) -> String {
        let mut output = String::new();
        output.push_str("<p>");
        output.push_str(&self.parse_section(&Section::slice_to_index(lines)[..]));
        output.push_str("</p>");
        return output;
    }

    fn convert_header(&self, line: &Section) -> String {
        let mut output = String::new();
        let depth = self.sequence_length(Symbol::NumberSign, line.start);
        let opening_tag = match depth {
            5 => "<h5>",
            4 => "<h4>",
            3 => "<h3>",
            2 => "<h2>",
            1 => "<h1>",
            _ => "<h6>",
        };
        output.push_str(opening_tag);
        let subsection = self.strip_paragraph_line(line.start + depth, line.end);

        // TODO: Parse inside headers.
        output.push_str(&self.string[subsection.start..subsection.end]);
        let closing_tag = match depth {
            5 => "</h5>",
            4 => "</h4>",
            3 => "</h3>",
            2 => "</h2>",
            1 => "</h1>",
            _ => "</h6>",
        };
        output.push_str(closing_tag);
        return output;
    }

    // HELPER FUNCTIONS //

    /// Check if the inputted string slice is found at the given index.
    fn check_match(&self, key: &Symbol, index: usize) -> bool {
        if index > self.string.len() {
            return false;
        }
        return &self.symbols[index] == key;
    }

    fn find_next(&self, symbol: Symbol, index: usize) -> Option<usize> {
        if index > self.string.len() {
            return None;
        }
        for i in index..self.string.len() {
            if self.symbols[i] == symbol {
                return Some(i);
            }
        }
        return None;
    }

    fn find_next_slice(&self, symbol: &[Symbol], index: usize) -> Option<usize> {
        for i in index + symbol.len()..self.string.len() {
            if &self.symbols[i..i + symbol.len()] == symbol {
                return Some(i);
            }
        }
        return None;
    }

    /// Check how many times (if any) a character is repeated.
    fn sequence_length(&self, key: Symbol, index: usize) -> usize {
        let mut length: usize = 0;
        loop {
            if self.check_match(&key, index + length) {
                length += 1;
            } else {
                return length;
            }
        }
    }

    fn is_blank(&self, start: usize, end: usize) -> bool {
        for i in start..end {
            if !self.symbols[i].is_blank() {
                return false;
            }
        }
        return true;
    }

    fn find_line_type(&self, start: usize, end: usize) -> LineType {
        if self.is_blank(start, end) {
            return LineType::Blank;
        }
        let offset = match self.symbols[start] {
            Symbol::Space => self.sequence_length(Symbol::Space, start),
            Symbol::Tab => return LineType::Code,
            _ => 0,
        };
        if offset > 3 {
            return LineType::Code;
        }
        match self.symbols[start + offset] {
            Symbol::NumberSign => {
                if self.symbols[start + offset + self.sequence_length(Symbol::NumberSign, start + offset)]
                    == Symbol::Space {
                        return LineType::Header;
                    } else {
                        return LineType::Paragraph;
                    }
            },
            Symbol::GreaterThan => return LineType::Blockquote,
            Symbol::Tab => return LineType::Code,
            _ => return LineType::Paragraph,
        }
    }

    fn sections(&self) -> Vec<Paragraph> {
         let mut paragraphs: Vec<Paragraph> = Vec::new();
         let mut lines: Vec<Section> = Vec::new();
         let mut last_line_type = LineType::Null;
         let mut index: usize = 0;
         loop {
             let next_newline = match self.find_next(Symbol::Newline, index) {
                 None => {
                     if index < self.string.len() {
                         let line_type = self.find_line_type(index, self.string.len());
                         if line_type == LineType::Blank {
                             break;
                         }
                         let stripped_line = self.strip_line(line_type, index, self.string.len());
                         if line_type == last_line_type {
                             lines.push(stripped_line);
                             paragraphs.push(Paragraph::new(lines, last_line_type));
                             lines = Vec::new();
                         } else {
                             if lines.len() > 0 {
                                 paragraphs.push(Paragraph::new(lines, last_line_type));
                                 lines = Vec::new();
                             }
                             lines.push(stripped_line);
                             paragraphs.push(Paragraph::new(lines, line_type));
                             lines = Vec::new();
                         }
                     }
                     break;
                 },
                 Some(int) => int,
             };

             // Skip extra newline characters at the start.
             if next_newline - index == 0 {
                 index += self.sequence_length(Symbol::Newline, index);
                 continue;
             }
             let newline_count = self.sequence_length(Symbol::Newline, next_newline);
             let line_type = self.find_line_type(index, next_newline);
             let stripped_line = self.strip_line(line_type, index, next_newline);
             if line_type == last_line_type {
                 lines.push(Section::new(index, next_newline));
                 index = next_newline + 1;
             } else {
                 match last_line_type {

                     // This is the first line.
                     LineType::Null => {
                         if line_type != LineType::Blank {
                             last_line_type = line_type;
                             lines.push(stripped_line);
                         }
                         index = next_newline + 1;
                     },

                     LineType::Blockquote => {
                         match line_type {

                             LineType::Blank => {
                                 paragraphs.push(Paragraph::new(lines, LineType::Blockquote));
                                 lines = Vec::new();
                                 last_line_type = LineType::Null;
                                 index = next_newline + 1;
                             },

                             // Headers break blockquotes.
                             LineType::Header => {
                                 paragraphs.push(Paragraph::new(lines, LineType::Blockquote));
                                 lines = Vec::new();
                                 lines.push(stripped_line);
                                 paragraphs.push(Paragraph::new(lines, LineType::Header));
                                 lines = Vec::new();
                                 last_line_type = LineType::Null;
                                 index = next_newline + 1;
                                 continue;
                             },

                             // Paragraph and code lines continue a blockquote.
                             _ => {
                                 lines.push(stripped_line);
                                 last_line_type = line_type;
                                 index = next_newline + 1;
                             }
                         }
                     },

                     LineType::Paragraph => {
                         match line_type {

                             LineType::Blank => {
                                 paragraphs.push(Paragraph::new(lines, LineType::Paragraph));
                                 lines = Vec::new();
                                 last_line_type = LineType::Null;
                                 index = next_newline + 1;
                             },

                             // Headers break paragraphs.
                             LineType::Header => {
                                 paragraphs.push(Paragraph::new(lines, LineType::Paragraph));
                                 lines = Vec::new();
                                 lines.push(stripped_line);
                                 paragraphs.push(Paragraph::new(lines, LineType::Header));
                                 lines = Vec::new();
                                 last_line_type = LineType::Null;
                                 index = next_newline + 1;
                                 continue;
                             },

                             //  Blockquotes break paragraphs.
                             LineType::Blockquote => {
                                 paragraphs.push(Paragraph::new(lines, LineType::Paragraph));
                                 lines = Vec::new();
                                 lines.push(stripped_line);
                                 last_line_type = line_type;
                                 index = next_newline + 1;
                                 continue;
                             },

                             // Lines of code do not break paragraphs.
                             _ => {
                                 lines.push(stripped_line);
                             },
                         }
                     }

                     // Anything that is not a line of code will break a line of code.
                     LineType::Code => {
                         last_line_type = line_type;
                         paragraphs.push(Paragraph::new(lines, LineType::Code));
                         lines = Vec::new();
                         lines.push(stripped_line);
                         index = next_newline + 1;
                         continue;
                     },
                     _ => {
                         last_line_type = line_type;
                         index = next_newline + 1;
                     }
                 }
             }
             if newline_count > 1 {
                 paragraphs.push(Paragraph::new(lines, last_line_type));
                 last_line_type = LineType::Null;
                 lines = Vec::new();
                 index = next_newline + newline_count;
                 continue;
             }

         }

         // Add the line and paragraph if that has not already been done.
         if lines.len() > 0 {
             paragraphs.push(Paragraph::new(lines, last_line_type));
         }
         return paragraphs;
     }

     fn strip_line(&self, kind: LineType, start: usize, end: usize) -> Section {
         return match kind {
             LineType::Paragraph => self.strip_paragraph_line(start, end),
             _ => self.strip_paragraph_line(start, end),
         };
     }

     /// Remove extra space in a line.
     fn strip_paragraph_line(&self, start: usize, end: usize) -> Section {
         let mut line_start = start;
         for i in start..end {
             if !self.symbols[i].is_blank() {
                 line_start = i;
                 break;
             }
         }
         return Section::new(line_start, end);
     }
}
