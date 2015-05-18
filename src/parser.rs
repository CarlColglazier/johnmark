use constants::HTML_CLOSE;
use input::Input;
use section::Section;
use symbol::Symbol;

#[allow(dead_code)]
pub struct Parser {
    pub input: Input,
    pub paragraphs: Vec<Section>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(input: &str) -> Parser {
        let new_input = Input::new(input);
        let paragraphs = Section::from_input(&new_input);
        return Parser { input: new_input, paragraphs: paragraphs };
    }
    pub fn convert(&self) -> String {
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
                // Links.
                /*
                Symbol::LeftBracket => {
                    match self.input.find_next("]:", paragraph) {
                        None => output.push_str(&self.input.parse_paragraph(paragraph)),

                        // Reference links.
                        Some(_) => output.push_str(&self.input.parse_reference_link(paragraph)),
                    }
                },
                */
                Symbol::GreaterThan => {
                    output.push_str(&self.input.parse_blockquote(paragraph));
                }
                _ => output.push_str(&self.input.parse_paragraph(paragraph)),
            }
        }
        return output;
    }
}
