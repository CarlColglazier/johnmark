use input::Input;
use symbol::Symbol;


// In markdown, the input can easily be reduced down to small blocks.
// These blocks are separated be two newline characters.
#[allow(dead_code)]
#[derive(PartialEq)]
pub struct Section {
    pub start: usize,
    pub end: usize,
}

#[allow(dead_code)]
impl Section {
    pub fn new(start: usize, end: usize) -> Section {
        return Section { start: start, end: end };
    }

    pub fn from_input(input: &Input) -> Vec<Section> {
        let mut sections: Vec<Section> = Vec::new();
        let mut start: usize = 0;
        for i in 0..input.symbols.len() - 1 {
            if start > i {
                continue;
            }
            if input.symbols[i] == Symbol::Newline {
                match input.symbols[i + 1] {
                    Symbol::Newline => {
                        sections.push(Section::new(start, i));
                        start = i + 2;
                    },
                    Symbol::NumberSign => {
                        // Make sure not to go over the index length.
                        if i + 2 < input.string.len() {
                            if input.symbols[i + 2] == Symbol::Space {
                                sections.push(Section::new(start, i));
                                start = i + 1;
                            }
                        }
                    },
                    Symbol::GreaterThan => {
                        sections.push(Section::new(start, i));
                        start = i + 1;
                    }
                    _ => {
                        continue;
                    }
                }
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
