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

// TODO: Headers.
/*
#[allow(dead_code)]
struct Header {
    weight: u8,
    offset: usize,
}

impl Header {
    #[allow(dead_code)]
    fn parse(symbols: Vec<Symbol>, start: usize) -> Option<Header> {
        let mut position: (usize, usize, usize, usize);
        for i in start..symbols.len() {
            let ref next_symbol = symbols[i+ 1];
            match next_symbol {
                &Symbol::Newline | &Symbol::EndInput => return None,
                _ => continue,
            }
        }
        return None;
    }
}
x
#[test]
fn test_header() {
    let parsed_str = parse_str("# Header\n");
    assert!(Header::parse(parsed_str, 0).is_none());
}
*/

/*
struct Link {
    a: String,
    href: String,
}
*/

/// Convert a string of markdown to HTML.
pub fn convert(input: &str) -> String {
    let parsed_str = Symbol::parse_str(input);
    let parsed_paragraphs = Paragraph::parse_symbols(&parsed_str);
    //let mut offset: usize = 0;
    let mut output = String::new();
    for paragraph in parsed_paragraphs {
        let opening_tag: &'static str;
        let closing_tag: &'static str;
        let offset: u8;
        if paragraph.label == ParagraphType::Paragraph {
            opening_tag = "<p>";
            closing_tag = "</p>";
            offset = 0;
        } else if paragraph.label == ParagraphType::Blockquote {
            opening_tag = "<blockquote>";
            closing_tag = "</blockquote>";
            offset = 1;
        } else {
            opening_tag = "";
            closing_tag = "";
            offset = 0;
        }
        output.push_str(opening_tag);
        for i in paragraph.start + offset as usize..paragraph.end {
            match input.chars().nth(i) {
                None => continue,
                Some(o) => {
                    if o != '\n' {
                        output.push(o)
                    }
                },
            }
        }
        output.push_str(closing_tag)
    }
    return output;
}

#[test]
fn test_convert() {
    assert_eq!("<p>Paragraph</p><blockquote>2</blockquote>", convert("Paragraph\n\n>2"));
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
