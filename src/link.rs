//! Hyperlinks.

use constants::{HTML_START_OPEN, HTML_END_OPEN, HTML_CLOSE};
use content::Content;
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
    fn from_inline(content: &Content, section: &Section) -> Option<LinkMetadata> {
        let mut link_start: usize = section.end;
        for i in section.start..section.end {
            match content.symbols[i] {
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
            match content.symbols[i] {
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
        let href = &content.string[link_start..link_end];
        let mut title: &str = "";
        if link_end < section.end {
            for i in link_end..section.end {
                match content.symbols[i] {
                    Symbol::Quote => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match content.find_next("\"", &subsection) {
                            None => "",
                            Some(n) => &content.string[i + 1..n],
                        };
                        break;
                    },
                    Symbol::Apostrophe => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match content.find_next("'", &subsection) {
                            None => "",
                            Some(n) => &content.string[i + 1..n],
                        };
                        break;
                    },
                    Symbol::LeftParenthsis => {
                        let subsection = Section::new(i + 1, section.end);
                        title = match content.find_next(")", &subsection) {
                            None => "",
                            Some(n) => &content.string[i + 1..n],
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
    /*
    fn from_inline(content: &content, section: &Section) -> Option<Link> {
        let mut id_start = section.start;
        for i in section.start..section.end {
            if content.symbols[i] == Symbol::LeftBracket {
                break;
            }
            id_start += 1;
        }
        let id_end = match content.find_next("](", section) {
            None => return None,
            Some(i) => i,
        };
        let id = &content.string[id_start..id_end];

    }
    */
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
    use content::content;
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
        let content_str = "http://carlcolglazier.com";
        let content = Content::from_str(content_str);
        let section = Section::new(0, content_str.len());
        let link_metadata = match LinkMetadata::from_inline(&content, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(content_str, link_metadata.href);

        // Extra spaces.
        let extra_content_str = "      http://carlcolglazier.com         ";
        let content = Content::from_str(extra_content_str);
        let section = Section::new(0, extra_content_str.len());
        let link_metadata = match LinkMetadata::from_inline(&content, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(content_str, link_metadata.href);

        // Include a title.
        let title_content_str = "http://carlcolglazier.com 'Title'";
        let content = Content::from_str(title_content_str);
        let section = Section::new(0, title_content_str.len());
        let link_metadata = match LinkMetadata::from_inline(&content, &section) {
            None => panic!("Parsing error."),
            Some(m) => m,
        };
        assert_eq!(content_str, link_metadata.href);
        assert_eq!(Some("Title".to_string()), link_metadata.title);
    }
}
