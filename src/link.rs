use constants::{HTML_START_OPEN, HTML_END_OPEN, HTML_CLOSE};
use input::Input;
use section::Section;
use symbol::Symbol;

#[allow(dead_code)]
pub struct LinkMetadata {
    href: String,
    title: Option<String>,
}

#[allow(dead_code)]
impl LinkMetadata {
    fn new(href: &str, title: &str) -> LinkMetadata {
        let title = match title.len() > 0 {
            true => Some(title.to_string()),
            false => None,
        };
        let href = href.to_string();
        return LinkMetadata { href: href, title: title };
    }
    fn from_inline(input: &Input, section: &Section) -> Option<LinkMetadata> {
        let mut link_start: usize = section.end;
        for i in section.start..section.end {
            match input.symbols[i] {
                Symbol::Newline | Symbol::Space => {
                    if i == section.end {
                        return None;
                    } else {
                        continue;
                    }
                },
                _ => {
                    link_start = i;
                    break;
                },
            }
        }
        let mut link_end: usize = section.end;
        for i in link_start..section.end {
            match input.symbols[i] {
                Symbol::Newline | Symbol::Space => {
                    link_end = i;
                    break;
                },
                _ => continue,
            }
        }
        if link_start == link_end {
            return None;
        }
        let href = &input.string[link_start..link_end];
        let mut title: &str = "";
        if link_end < section.end {
            for i in link_end..section.end {
                match input.symbols[i] {
                    Symbol::Quote => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match input.find_next("\"", &subsection) {
                            None => "",
                            Some(n) => &input.string[i + 1..n],
                        };
                        break;
                    },
                    Symbol::Apostrophe => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match input.find_next("'", &subsection) {
                            None => "",
                            Some(n) => &input.string[i + 1..n],
                        };
                        break;
                    },
                    Symbol::LeftParenthsis => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match input.find_next(")", &subsection) {
                            None => "",
                            Some(n) => &input.string[i + 1..n],
                        };
                        break;
                    },
                    _ => continue,
                }
            }
        }
        return Some(LinkMetadata::new(href, title));
    }
}

#[allow(dead_code)]
pub struct Link {
    id: String,
    metadata: LinkMetadata
}

#[allow(dead_code)]
impl Link {
    fn new(id: &str, href: &str, title: &str) -> Link {
        let id = id.to_string();
        let metadata = LinkMetadata::new(href, title);
        return Link { id: id, metadata: metadata };
    }
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(HTML_START_OPEN);
        output.push_str("a href=\"");
        output.push_str(&self.metadata.href);
        output.push('"');
        match self.metadata.title {
            None => output.push_str(HTML_CLOSE),
            Some(ref title) => {
                output.push_str(" title=\"");
                output.push_str(title);
                output.push('"');
                output.push_str(HTML_CLOSE);
            }
        }
        output.push_str(&self.id);
        output.push_str(HTML_END_OPEN);
        output.push_str("a");
        output.push_str(HTML_CLOSE);
        return output;
    }
}

#[cfg(test)]
mod test {
    use super::Link;
    use super::LinkMetadata;
    use input::Input;
    use section::Section;

    #[test]
    fn simple_link() {
        let link = Link::new("link", "http://carlcolglazier.com/", "Sample Link.");
        assert_eq!(
            "<a href=\"http://carlcolglazier.com/\" title=\"Sample Link.\">link</a>",
            link.to_string()
        );
    }

    #[test]
    fn no_title() {
        let link = Link::new("link", "http://carlcolglazier.com/", "");
        assert_eq!("<a href=\"http://carlcolglazier.com/\">link</a>", link.to_string());
    }

    #[test]
    fn parse_link_metadata() {

        // Just the link.
        let input_str = "http://carlcolglazier.com";
        let input = Input::new(input_str);
        let section = Section::new(0, input_str.len());
        let link_metadata = match LinkMetadata::from_inline(&input, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(input_str, link_metadata.href);

        // Extra spaces.
        let extra_input_str = "      http://carlcolglazier.com         ";
        let input = Input::new(extra_input_str);
        let section = Section::new(0, extra_input_str.len());
        let link_metadata = match LinkMetadata::from_inline(&input, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(input_str, link_metadata.href);

        // Include a title.
        let title_input_str = "http://carlcolglazier.com 'Title'";
        let input = Input::new(title_input_str);
        let section = Section::new(0, title_input_str.len());
        let link_metadata = match LinkMetadata::from_inline(&input, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(input_str, link_metadata.href);
        assert_eq!(Some("Title".to_string()), link_metadata.title);
    }
}
