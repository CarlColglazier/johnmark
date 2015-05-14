#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Symbol {
    LeftBracket, // [
    RightBracket, // ]
    LeftParenthsis, // (
    RightParenthsis, // )
    Asterisk, // *
    Underscore, // _
    Hyphen,
    Plus, // +
    Newline, // \n
    NumberSign, // #
    Code, // `
    Escape, // \
    Ampersand, // &
    LessThan, // <
    GreaterThan, // >
    EqualsSign, // =
    Other,
    Space,
    EndInput,
}

#[allow(dead_code)]
impl Symbol {
    pub fn from_char(input: char) -> Symbol {
        return match input {
            '[' => Symbol::LeftBracket,
            ']' => Symbol::RightBracket,
            '(' => Symbol::LeftParenthsis,
            ')' => Symbol::RightParenthsis,
            '*' => Symbol::Asterisk,
            '_' => Symbol::Underscore,
            '-' => Symbol::Hyphen,
            '+' => Symbol::Plus,
            '\r' | '\n' => Symbol::Newline,
            '#' => Symbol::NumberSign,
            '<' => Symbol::LessThan,
            '>' => Symbol::GreaterThan,
            '`' => Symbol::Code,
            '=' => Symbol::EqualsSign,
            '\\' => Symbol::Escape,
            ' ' => Symbol::Space,
            _ => Symbol::Other,
        };
    }
}
