use super::markdown::linkregistry::LinkRegistry;
use super::markdown::parser::{InlineElement, ListItem, MarkdownElement, MarkdownParser};
use crate::prelude::*;

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
    let mut registry = LinkRegistry::new();
    registry.register_header_position("Test Header", 42);
    assert_eq!(registry.get_header_position("test-header"), Some(42));
}

#[test]
fn test_register_link_position_and_check() {
    let mut registry = LinkRegistry::new();
    registry.register_link_position("example-link", 10, 20, 5, false);
    assert_eq!(registry.check_for_link_at_position(12, 20), Some("example-link".to_string()));
    assert_eq!(registry.check_for_link_at_position(16, 20), None);
}

#[test]
fn test_get_id_from_header() {
    assert_eq!(LinkRegistry::get_id_from_header("Example Header"), "example-header");
    assert_eq!(LinkRegistry::get_id_from_header("Another_Test"), "anothertest");
}

// Markdown tests
#[test]
fn check_scrollbars_on_arrows_and_click() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x5EC4171CCE8C9EA9)
        Key.Pressed(Right,5)
        Paint('2. bottom scrollbar moved with keys')
        CheckHash(0xB63A094AC561BACF)
        Key.Pressed(Down,3)
        Paint('3. vertical scrollbar moved with keys')
        CheckHash(0x9B4BF0FBAFDA353F)
        Key.Pressed(Left,3)
        Key.Pressed(Right,20)
        Key.Pressed(Up,1)
        Paint('4. Scrollbars on bottom-right')
        CheckHash(0x32ABDD6A0506AB2)
        Mouse.Move(49,7)
        Mouse.Click(49,7,left)
        Mouse.Move(31,13)
        Mouse.Click(31,13,left)
        Mouse.Move(44,16)
        Paint('5. scroll bars moved with mouse')
        CheckHash(0xB7F2FEFDD0647BF4) 
";
    let m = markdown!(
        "'
    # My Markdown Example\r\n\r\n\
    Welcome to this **Markdown** example! This file *demonstrates* basic Markdown syntax.\r\n\r\n\
    ---\r\n\
    ## Back link\r\n\
    Some testing for links back \r\n\r\n\
    ## Table of Contents\r\n\r\n\
    1. [Introduction](#introduction)\r\n\
    \t1. ana\r\n\
    \t2. are\r\n\
    \t3. mere\r\n\
    2. [Features](#features)\r\n\
    \t1. gigi\r\n\
    \t\t1. vine\r\n\
    \t\t2. cere\r\n\
    3. [Conclusion](#conclusion)\r\n\r\n\
    ## Introduction\r\n\r\n\
    Markdown is a lightweight markup language for creating formatted text using a plain-text editor.\r\n\r\n\
    ### Why use Markdown?\r\n\r\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\r\n\
    - __Readable__: Looks great as plain text.\r\n\
    - **Flexible**: Converts to HTML and other formats.\r\n\r\n\
    ## Features\r\n\r\n\
    Here are some Markdown features:\r\n\r\n\
    ### Lists\r\n\r\n\
    #### Unordered\r\n\r\n\
    - Item 1\r\n\
    \t- Sub-item 1.1\r\n\
    \t- Sub-item 1.2\r\n\
    \t\t- Subitem 1.2.1\r\n\
    \t\t- Subitem 1.2.2\r\n\
    \t- Item 1.3\r\n\
    - Item 2\r\n\r\n\
    #### Code Blocks\r\n\r\n\
    Here is an example of inline code: `let x = 10;`\r\n\r\n\
    And here is a code block:\r\n\r\n\
    ```\r\n\
    fn main() {\r\n\
    \tprintln!(\"Hello, world!\");\r\n\
    }\r\n\
    ```\r\n\
    [Get me back](#back-link)\r\n\r\n\
    | Column 1 | Column 2 |\r\n\
    | - | --- |\r\n\
    | Cell 1, __Row 1__ | Cell 2, Row 1 |\r\n\
    | Cell 1, Row 2 | Cell 1, Row 2 |\r\n\r\n\
    ', d:c, flags:ScrollBars"
    );

    let mut a = App::debug(60, 20, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    w.add(m);
    a.add_window(w);
    a.run();
}

// complete here
#[test]
fn check_scrollbars_on_drag_and_weel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x93FD8A2AF6052839)
        Mouse.Wheel(5,5,down,5)
        Paint('2. Scroll weel down')
        CheckHash(0x64BD055E27582B91)
        Mouse.Wheel(5,5,up,5)
        Paint('3. scroll weel up')
        CheckHash(0x93FD8A2AF6052839)
        Mouse.Drag(5,5,5,3)
        Paint('4. drag mouse up (scroll down)')
        CheckHash(0xA70336B5C3B4EC86)
        Mouse.Drag(5,5,5,12)
        Paint('5. drag mouse down (scroll up)')
        CheckHash(0x93FD8A2AF6052839)
    ";
    let m = markdown!(
        "'
    # My Markdown Example\r\n\r\n\
    Welcome to this **Markdown** example! This file *demonstrates* basic Markdown syntax.\r\n\r\n\
    ---\r\n\
    ## Back link\r\n\
    Some testing for links back \r\n\r\n\
    ## Table of Contents\r\n\r\n\
    1. [Introduction](#introduction)\r\n\
    \t1. ana\r\n\
    \t2. are\r\n\
    \t3. mere\r\n\
    2. [Features](#features)\r\n\
    \t1. gigi\r\n\
    \t\t1. vine\r\n\
    \t\t2. cere\r\n\
    3. [Conclusion](#conclusion)\r\n\r\n\
    ## Introduction\r\n\r\n\
    Markdown is a lightweight markup language for creating formatted text using a plain-text editor.\r\n\r\n\
    ### Why use Markdown?\r\n\r\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\r\n\
    - __Readable__: Looks great as plain text.\r\n\
    - **Flexible**: Converts to HTML and other formats.\r\n\r\n\
    ## Features\r\n\r\n\
    Here are some Markdown features:\r\n\r\n\
    ### Lists\r\n\r\n\
    #### Unordered\r\n\r\n\
    - Item 1\r\n\
    \t- Sub-item 1.1\r\n\
    \t- Sub-item 1.2\r\n\
    \t\t- Subitem 1.2.1\r\n\
    \t\t- Subitem 1.2.2\r\n\
    \t- Item 1.3\r\n\
    - Item 2\r\n\r\n\
    #### Code Blocks\r\n\r\n\
    Here is an example of inline code: `let x = 10;`\r\n\r\n\
    And here is a code block:\r\n\r\n\
    ```\r\n\
    fn main() {\r\n\
    \tprintln!(\"Hello, world!\");\r\n\
    }\r\n\
    ```\r\n\
    [Get me back](#back-link)\r\n\r\n\
    | Column 1 | Column 2 |\r\n\
    | - | --- |\r\n\
    | Cell 1, __Row 1__ | Cell 2, Row 1 |\r\n\
    | Cell 1, Row 2 | Cell 1, Row 2 |\r\n\r\n\
    ', d:c, flags:ScrollBars"
    );

    let mut a = App::debug(40, 8, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    w.add(m);
    a.add_window(w);
    a.run();
}

#[test]
fn check_move_to_section_on_link_click() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x173C84B9116601C0)
        Mouse.Move(3,2)
        Mouse.Click(3,2,left)
        Paint('2. scroll bars moved with mouse')
        CheckHash(0x19DD90B0FD257FA9) 
    ";
    let m = markdown!(
        "'
    [Go to end](#end-link)\r\n\r\n\
    Some testing for links back \r\n\r\n\
    ## Table of Contents\r\n\r\n\
    1. [Introduction](#introduction)\r\n\
    \t1. ana\r\n\
    \t2. are\r\n\
    \t3. mere\r\n\
    2. [Features](#features)\r\n\
    \t1. gigi\r\n\
    \t\t1. vine\r\n\
    \t\t2. cere\r\n\
    3. [Conclusion](#conclusion)\r\n\r\n\
    ## Introduction\r\n\r\n\
    Markdown is a lightweight markup language for creating formatted text using a plain-text editor.\r\n\r\n\
    ### Why use Markdown?\r\n\r\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\r\n\
    - __Readable__: Looks great as plain text.\r\n\
    - **Flexible**: Converts to HTML and other formats.\r\n\r\n\
    ## Features\r\n\r\n\
    Here are some Markdown features:\r\n\r\n\
    ### Lists\r\n\r\n\
    #### Unordered\r\n\r\n\
    - Item 1\r\n\
    \t- Sub-item 1.1\r\n\
    \t- Sub-item 1.2\r\n\
    \t\t- Subitem 1.2.1\r\n\
    \t\t- Subitem 1.2.2\r\n\
    \t- Item 1.3\r\n\
    - Item 2\r\n\r\n\
    #### Code Blocks\r\n\r\n\
    Here is an example of inline code: `let x = 10;`\r\n\r\n\
    And here is a code block:\r\n\r\n\
    ```\r\n\
    fn main() {\r\n\
    \tprintln!(\"Hello, world!\");\r\n\
    }\r\n\
    ```\r\n\
    ## End Link\r\n\
    ', d:c, flags:ScrollBars"
    );

    let mut a = App::debug(40, 8, script).build().unwrap();
    let mut w = window!("Title,d:c,w:40,h:8,flags:Sizeable");
    w.add(m);
    a.add_window(w);
    a.run();
}

#[test]
fn check_inactive() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF5953A121DF629B6)
        Mouse.Move(3,2)
        Mouse.Click(3,2,left)
        Paint('2. scroll bars moved with mouse')
        CheckHash(0xF5953A121DF629B6) 
";
    let mut m = markdown!(
        "'
    [Go to end](#end-link)\r\n\r\n\
    Some testing for links back \r\n\r\n\
    ## Table of Contents\r\n\r\n\
    1. [Introduction](#introduction)\r\n\
    \t1. ana\r\n\
    \t2. are\r\n\
    \t3. mere\r\n\
    2. [Conclusion](#conclusion)\r\n\r\n\
    ## Introduction\r\n\r\n\
    Markdown is a lightweight markup language.\r\n\r\n\
    ### Why use Markdown?\r\n\r\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\r\n\
    - __Readable__: Looks great as plain text.\r\n\
    - **Flexible**: Converts to HTML and other formats.\r\n\r\n\
    #### Code Blocks\r\n\r\n\
    Here is an example of inline code: `let x = 10;`\r\n\r\n\
    And here is a code block:\r\n\r\n\
    ```\r\n\
    fn main() {\r\n\
    \tprintln!(\"Hello, world!\");\r\n\
    }\r\n\
    ```\r\n\
    ## End Link\r\n\
    ', d:c, flags:ScrollBars"
    );

    let mut a = App::debug(70, 30, script).build().unwrap();
    let mut w = window!("Title,d:c,w:70,h:30,flags:Sizeable");
    m.set_enabled(false);
    w.add(m);
    a.add_window(w);
    a.run();
}

#[test]
fn check_set_content() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF1F34BA26A32DF9C)
";
    let mut m = markdown!(
        "'
    [Go to end](#end-link)\r\n\r\n\
    Some testing for links back \r\n\r\n\
    ## Table of Contents\r\n\r\n\
    1. [Introduction](#introduction)\r\n\
    \t1. ana\r\n\
    \t2. are\r\n\
    \t3. mere\r\n\
    2. [Conclusion](#conclusion)\r\n\r\n\
    ## Introduction\r\n\r\n\
    Markdown is a lightweight markup language.\r\n\r\n\
    ### Why use Markdown?\r\n\r\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\r\n\
    - __Readable__: Looks great as plain text.\r\n\
    - **Flexible**: Converts to HTML and other formats.\r\n\r\n\
    #### Code Blocks\r\n\r\n\
    Here is an example of inline code: `let x = 10;`\r\n\r\n\
    And here is a code block:\r\n\r\n\
    ```\r\n\
    fn main() {\r\n\
    \tprintln!(\"Hello, world!\");\r\n\
    }\r\n\
    ```\r\n\
    ## End Link\r\n\
    ', d:c, flags:ScrollBars"
    );

    let mut a = App::debug(70, 30, script).build().unwrap();
    let mut w = window!("Title,d:c,w:70,h:30,flags:Sizeable");
    m.set_enabled(false);
    m.set_content("# Empty markdown");
    w.add(m);
    a.add_window(w);
    a.run();
}
