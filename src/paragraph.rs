use section::Section;
use content::LineType;

#[allow(dead_code)]
pub struct Paragraph {
    pub lines: Vec<Section>,
    pub kind: LineType
}

#[allow(dead_code)]
impl Paragraph {
    pub fn new(lines: Vec<Section>, kind: LineType) -> Paragraph {
        return Paragraph { lines: lines, kind: kind };
    }
}
