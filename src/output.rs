#[allow(dead_code)]
pub struct Output {
    pub string: String,
    pub offset: usize,
}

#[allow(dead_code)]
impl Output {
    pub fn new(string: String, offset: usize) -> Output {
        return Output{ string: string, offset: offset };
    }
    pub fn from_str(string: &str, offset: usize) -> Output {
        return Output::new(string.to_string(), offset);
    }
}
