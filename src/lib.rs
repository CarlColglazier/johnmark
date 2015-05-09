//!A native markdown parser for Rust with zero dependencies.

#[derive(PartialEq)]
enum ParseResult {
    False, // Not a match.
    True, // Match. Finish parsing.
    Continue, // Might be a match. Keep going.
}

#[derive(PartialEq)]
enum Symbol {
    LeftBracket, // [
    RightBracket, // ]
    LeftParenthsis, // (
    RightParenthsis, // )
    Asterisk, // *
    Underscore, // _
    Plus, // +
    Newline, // \n
    NumberSign, // #
    Blockquote, // >
    Unknown,
    EndInput,
}

impl Symbol {
    fn tokenize(input: char) -> Symbol {
        return match input {
            '[' => Symbol::LeftBracket,
            ']' => Symbol::RightBracket,
            '(' => Symbol::LeftParenthsis,
            ')' => Symbol::RightParenthsis,
            '*' => Symbol::Asterisk,
            '_' => Symbol::Underscore,
            '+' => Symbol::Plus,
            '\r' | '\n' => Symbol::Newline,
            '#' => Symbol::NumberSign,
            '>' => Symbol::Blockquote,
            _ => Symbol::Unknown,
        };
    }
    fn parse_str(input: &str) -> Vec<Symbol> {
        let mut tokens: Vec<Symbol> = Vec::new();
        for character in input.trim().chars() {
            tokens.push(Symbol::tokenize(character));
        }
        tokens.push(Symbol::EndInput);
        return tokens;
    }
}

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn new(start: usize, end: usize) -> Section {
        return Section{start: start, end: end};
    }
    fn parse(&self, input: &str, parsed_str: &Vec<Symbol>) -> String {
        let mut output = String::new();
        for i in self.start..self.end {
            match parsed_str[i] {
                Symbol::Newline => continue,
                Symbol::EndInput => break,
                _ => output.push(input.chars().nth(i).unwrap_or(' ')),
            }
        }
        return output;
    }
}

#[test]
fn test_section() {
    let input = "`code` block.";
    let parsed_str = Symbol::parse_str(input);
    let section = Section::new(0, parsed_str.len());
    assert_eq!("`code` block.", section.parse(input, &parsed_str));
}

#[derive(PartialEq)]
enum ParagraphType {
    Header,
    Paragraph,
    Blockquote,
    Error,
}

struct Paragraph {
    start: usize,
    end: usize,
    label: ParagraphType,
}

impl Paragraph {
    fn parse_next(next_symbol: &Symbol) -> ParseResult {
        match next_symbol {
            &Symbol::Newline => {
                return ParseResult::Continue;
            },
            &Symbol::EndInput => ParseResult::True,
            _ => return ParseResult::False,
        }
    }
    fn parse(symbols: &Vec<Symbol>, start: usize) -> Paragraph {
        let paragraph_type = match symbols[start] {
            Symbol::Blockquote => ParagraphType::Blockquote,
            Symbol::NumberSign => ParagraphType::Header,
            _ => ParagraphType::Paragraph,
        };
        for i in start..symbols.len() {
            let ref next_symbol = symbols[i];
            match Paragraph::parse_next(next_symbol) {
                ParseResult::False => continue,
                ParseResult::Continue => {
                    let ref next_next_symbol = symbols[i + 1];
                    match Paragraph::parse_next(next_next_symbol) {
                        ParseResult::False => continue,
                        ParseResult::True | ParseResult::Continue => {
                            return Paragraph {start: start, end: i + 1, label: paragraph_type };
                        },
                    }
                },
                ParseResult::True => {
                    return Paragraph { start: start, end: i, label: paragraph_type };
                },
            }
        };
        return Paragraph{ start: 0, end: 0, label: ParagraphType::Error };
    }
    fn parse_symbols(symbols: &Vec<Symbol>) -> Vec<Paragraph> {
        let mut paragarphs: Vec<Paragraph> = Vec::new();
        let mut i: usize = 0;
        loop {
            let next_paragraph = Paragraph::parse(&symbols, i);
            let last = next_paragraph.end;
            if next_paragraph.label != ParagraphType::Error {
                paragarphs.push(next_paragraph);
            }
            if last + 1 == symbols.len() {
                break;
            } else {
                i = last + 1;
            }
        }
        return paragarphs;
    }
    fn parse_paragraph(&self, input: &str, parsed_str: &Vec<Symbol>) -> String {
        let mut output = String::new();
        if self.label == ParagraphType::Paragraph {
            output.push_str("<p>");
            output.push_str(&Section::new(self.start, self.end).parse(input, parsed_str));

            output.push_str("</p>");
        } else if self.label == ParagraphType::Header {
            let mut header_weight: u8 = 0;
            for i in self.start..self.start + 5 {
                if parsed_str[i] == Symbol::NumberSign {
                    header_weight += 1;
                } else {
                    break;
                }
            }
            let header_str = match header_weight{
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                _ => "6",
            };
            output.push_str("<h");
            output.push_str(header_str);
            output.push_str(">");
            for i in self.start + header_weight as usize..self.end {
                if parsed_str[i] != Symbol::Newline {
                    output.push(input.chars().nth(i).unwrap_or(' '));
                }
            };
            output.push_str("</h");
            output.push_str(header_str);
            output.push_str(">");
        } else if self.label == ParagraphType::Blockquote {
            output.push_str("<blockquote>");
            let shorter_paragraph = Paragraph::parse(&parsed_str, self.start + 1);
            output.push_str(&shorter_paragraph.parse_paragraph(input, &parsed_str));
            output.push_str("</blockquote>");
        } else {
            output = String::new();
        }
        return output;
    }
}

#[test]
fn test_paragraph() {
    let parsed_str = Symbol::parse_str("Paragraph\n\n# New Paragraph");
    let parsed_paragraph = Paragraph::parse(&parsed_str, 0);
    assert_eq!(0, parsed_paragraph.start);
    assert_eq!(10, parsed_paragraph.end);
    let parsed_paragraphs = Paragraph::parse_symbols(&parsed_str);
    assert_eq!(2, parsed_paragraphs.len());
    assert!(ParagraphType::Paragraph == parsed_paragraphs[0].label);
    assert!(ParagraphType::Header == parsed_paragraphs[1].label);
}

/// Convert a string of markdown to HTML.
pub fn convert(input: &str) -> String {
    let parsed_str = Symbol::parse_str(input);
    let parsed_paragraphs = Paragraph::parse_symbols(&parsed_str);
    let mut output = String::new();
    for paragraph in parsed_paragraphs {
        output.push_str(&paragraph.parse_paragraph(input, &parsed_str));
    }
    return output;
}

#[test]
fn test_convert() {
    assert_eq!("<p>Paragraph</p><blockquote><p>2</p></blockquote><h2>Header</h2>",
        convert("Paragraph\n\n>2\n\n##Header"));
}

#[cfg(test)]
mod tests {
    use super::Symbol;

    #[test]
    fn test_parse() {
        assert_eq!(true, Symbol::tokenize('[') == Symbol::LeftBracket);
        assert_eq!(true, Symbol::tokenize(']') == Symbol::RightBracket);
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(4, Symbol::parse_str("*.*").len());
    }
}
