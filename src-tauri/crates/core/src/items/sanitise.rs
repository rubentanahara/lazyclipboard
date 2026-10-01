use ammonia::Builder;
use std::collections::HashSet;

const NON_FORMATTING_TAGS: [&str; 4] = ["div", "span", "p", "br"];

pub struct Sanitised {
    pub html: String,
    pub has_formatting: bool,
}

pub fn sanitise(html: &str) -> Sanitised {
    let html_without_scripts = Builder::default().clean(html).to_string();
    let structure_only = Builder::default()
        .tags(HashSet::from(NON_FORMATTING_TAGS))
        .clean(html)
        .to_string();
    Sanitised {
        has_formatting: html_without_scripts != structure_only,
        html: html_without_scripts,
    }
}
