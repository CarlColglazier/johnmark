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

    pub fn slice_to_index(sections: &[Section]) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();
        for section in sections.iter() {
            for i in section.start..section.end + 1 {
                indexes.push(i);
            }
        }
        return indexes;
    }

    pub fn to_index(sections: &Vec<Section>) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();
        for section in sections.iter() {
            for i in section.start..section.end + 1 {
                indexes.push(i);
            }
        }
        return indexes;
    }
}
