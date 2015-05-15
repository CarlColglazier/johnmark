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
    Tab,
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
            '\t' => Symbol::Tab,
            '#' => Symbol::NumberSign,
            '<' => Symbol::LessThan,
            '>' => Symbol::GreaterThan,
            '`' => Symbol::Code,
            '=' => Symbol::EqualsSign,
            '\\' => Symbol::Escape,
            ' ' => Symbol::Space,
            '&' => Symbol::Ampersand,
            _ => Symbol::Other,
        };
    }
    pub fn is_char_entity(&self) -> bool {
        match self {
            &Symbol::Ampersand => return true,
            &Symbol::LeftBracket => return true,
            &Symbol::RightBracket => return true,
            &Symbol::LessThan => return true,
            &Symbol::GreaterThan => return true,
            _ => return false,
        }
    }
    pub fn is_blank(&self) -> bool {
        match self {
            &Symbol::Space => return true,
            &Symbol::Tab => return true,
            &Symbol::Newline => return true,
            _ => return false,
        }
    }
}
