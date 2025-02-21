use super::markdown::linkheaderregistry::LinkHeaderRegistry;
use super::markdown::parser::{InlineElement, ListItem, MarkdownElement, MarkdownParser};

// MarkdownParser Tests
#[test]
fn test_parse_header() {
    let input = "# Header";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::Header(content, level) => {
            assert_eq!(content, "Header");
            assert_eq!(*level, 1);
        }
        _ => panic!("Expected a Header element"),
    }
}

#[test]
fn test_parse_multiple_headers() {
    let input = "# Header 1\n## Header 2";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 2);
    match &parsed[0] {
        MarkdownElement::Header(content, level) => {
            assert_eq!(content, "Header 1");
            assert_eq!(*level, 1);
        }
        _ => panic!("Expected a Header element"),
    }
    match &parsed[1] {
        MarkdownElement::Header(content, level) => {
            assert_eq!(content, "Header 2");
            assert_eq!(*level, 2);
        }
        _ => panic!("Expected a Header element"),
    }
}

#[test]
fn test_parse_paragraph() {
    let input = "This is a paragraph.";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::Paragraph(inline_elements) => {
            assert_eq!(inline_elements.len(), 1);
            match &inline_elements[0] {
                InlineElement::Text(content) => assert_eq!(content, "This is a paragraph."),
                _ => panic!("Expected a Text element"),
            }
        }
        _ => panic!("Expected a Paragraph element"),
    }
}

#[test]
fn test_parse_unordered_list() {
    let input = "- Item 1\n- Item 2\n- Item 3";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::UnorderedList(items) => {
            assert_eq!(items.len(), 3);
            match &items[0] {
                ListItem::Simple(inline_elements) => {
                    assert_eq!(inline_elements.len(), 1);
                    match &inline_elements[0] {
                        InlineElement::Text(content) => assert_eq!(content, "Item 1"),
                        _ => panic!("Expected a Text element"),
                    }
                }
                _ => panic!("Expected a Simple ListItem"),
            }
        }
        _ => panic!("Expected an UnorderedList element"),
    }
}

#[test]
fn test_parse_ordered_list() {
    let input = "1. Item 1\n2. Item 2\n3. Item 3";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::OrderedList(items) => {
            assert_eq!(items.len(), 3);
            match &items[0] {
                ListItem::Simple(inline_elements) => {
                    assert_eq!(inline_elements.len(), 1);
                    match &inline_elements[0] {
                        InlineElement::Text(content) => assert_eq!(content, "Item 1"),
                        _ => panic!("Expected a Text element"),
                    }
                }
                _ => panic!("Expected a Simple ListItem"),
            }
        }
        _ => panic!("Expected an OrderedList element"),
    }
}

#[test]
fn test_parse_table() {
    let input = "| Header 1 | Header 2 |\n|----------|----------|\n| Row 1, Cell 1 | Row 1, Cell 2 |\n| Row 2, Cell 1 | Row 2, Cell 2 |";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::Table(table) => {
            assert_eq!(table.headers.len(), 2);
            assert_eq!(table.rows.len(), 2);
            assert_eq!(table.rows[0].len(), 2);
            match &table.rows[0][0][0] {
                InlineElement::Text(content) => assert_eq!(content, "Row 1, Cell 1"),
                _ => panic!("Expected a Text element"),
            }
        }
        _ => panic!("Expected a Table element"),
    }
}

#[test]
fn test_parse_inline_elements() {
    let input = "This is **bold** and this is *italic* and [a link](http://example.com).";
    let parsed = MarkdownParser::parse(input);
    assert_eq!(parsed.len(), 1);
    match &parsed[0] {
        MarkdownElement::Paragraph(inline_elements) => {
            assert_eq!(inline_elements.len(), 7);
            match &inline_elements[1] {
                InlineElement::Bold(content) => assert_eq!(content, "bold"),
                _ => panic!("Expected a Bold element"),
            }
            match &inline_elements[3] {
                InlineElement::Italic(content) => assert_eq!(content, "italic"),
                _ => panic!("Expected an Italic element"),
            }
            match &inline_elements[5] {
                InlineElement::Link(text, url) => {
                    assert_eq!(text, "a link");
                    assert_eq!(url, "http://example.com");
                }
                _ => panic!("Expected a Link element"),
            }
        }
        _ => panic!("Expected a Paragraph element"),
    }
}

// LinkHeaderRegistry Tests
#[test]
fn test_register_and_get_header_position() {
    let mut registry = LinkHeaderRegistry::new();
    registry.register_header_position("Test Header", 42);
    assert_eq!(registry.get_header_position("test-header"), Some(42));
}

#[test]
fn test_register_link_position_and_check() {
    let mut registry = LinkHeaderRegistry::new();
    registry.register_link_position("example-link", 10, 20, 5);
    assert_eq!(registry.check_for_link_at_position(12, 20), Some("example-link".to_string()));
    assert_eq!(registry.check_for_link_at_position(16, 20), None);
}

#[test]
fn test_get_id_from_header() {
    assert_eq!(LinkHeaderRegistry::get_id_from_header("Example Header"), "example-header");
    assert_eq!(LinkHeaderRegistry::get_id_from_header("Another_Test"), "anothertest");
}
