use crate::adapters::parser::HtmlParser;

pub struct AppContext {
    pub parser: HtmlParser,
}

impl Default for AppContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            parser: HtmlParser::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_context_parser() {
        let context = AppContext::new();
        assert_eq!(
            context
                .parser
                .parse_links("<a href=\"https://example.com\">Example</a>")
                .unwrap(),
            vec!["https://example.com".to_string()]
        );
    }
}
