use section::Section;
use content::LineType;

#[allow(dead_code)]
pub struct Line {
    pub kind: LineType,
    pub section: Section,
}

#[allow(dead_code)]
impl Line {
    pub fn new(kind: LineType, start: usize, end: usize) -> Line {
        return Line { kind: kind, section: Section::new(start, end) };
    }
}
