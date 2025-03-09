use appcui::prelude::*;
#[Window(events = MarkdownEvents)]
struct my_win {}

impl my_win {
    fn new () -> Self {
        let mut w = Self { base: window!("Markdown,d:c,w:50,h:15,flags:sizeable") };
        let m = markdown!(
            "'''Rust is a modern systems programming language that emphasizes memory safety, concurrency, and performance.\r\n\
            \r\n\
            # Rust Programming Language\r\n\
            \r\n\
            ## Table of Contents\r\n\
            - [Introduction](#introduction)\r\n\
            \t- [What Makes Rust Unique?](#what-makes-rust-unique)\r\n\
            - [Features](#features)\r\n\
            \t- [Ownership and Borrowing](#ownership-and-borrowing)\r\n\
            \t- [Concurrency](#concurrency)\r\n\
            \r\n\
            ## Introduction\r\n\
            \r\n\
            Rust is a statically typed language designed to eliminate common programming errors at compile time while delivering high performance.\r\n\
            \r\n\
            ### What Makes Rust Unique?\r\n\
            \r\n\
            - **Memory Safety**: Rust's ownership model prevents null pointer dereferences and data races.\r\n\
            - **Concurrency**: Built-in support for safe, concurrent programming.\r\n\
            - **Performance**: Delivers speed comparable to C/C++.\r\n\
            - **Modern Syntax**: Offers clear, expressive code that is easy to maintain.\r\n\
            \r\n\
            ## Features\r\n\
            \r\n\
            Rust provides several advanced features that set it apart:\r\n\
            \r\n\
            ### Ownership and Borrowing\r\n\
            \r\n\
            Rust enforces strict rules for how memory is accessed and managed, ensuring that bugs like use-after-free and data races are caught at compile time.\r\n\
            \r\n\
            ### Concurrency\r\n\
            \r\n\
            Rust's design promotes safe concurrency, enabling multithreaded programming without the typical pitfalls of shared mutable state.\r\n\
            \r\n\
            Inline code example: `let x = 10;`\r\n\
            \r\n\
            Block code example:\r\n\
            \r\n\
            ```\r\n\
            fn main() {\r\n\
            \tprintln!(\"Hello, world!\");\r\n\
            }\r\n\
            ```\r\n\
            [Link EXTERN](https://google.com)\r\n\
            \r\n\
            | Feature           | Description                                                          |\r\n\
            | ----------------- | -------------------------------------------------------------------- |\r\n\
            | Memory Safety     | Prevents null pointers and data races through ownership rules.       |\r\n\
            | Concurrency       | Enables safe multithreading with minimal runtime overhead.           |\r\n\
            | Performance       | Optimized for high-performance, low-level systems programming.       |\r\n\
            | Expressive Syntax | Modern syntax that enhances code clarity and maintainability.         |\r\n\
            \r\n\
            [Rust Programming Language](#rust-programming-language)\r\n\
            ''',d: c,flags: ScrollBars,lsm:10,tsm:1"
        );
        w.add(m);
        w
    }
}
impl MarkdownEvents for my_win {
    fn on_external_link(&mut self, _handle: Handle<Markdown>, link: &str) -> EventProcessStatus {
        if let Some(md) = self.control_mut(_handle) {
            md.set_content(&format!("Go to {}", link));
        }
        EventProcessStatus::Processed
    }
}
fn main() -> Result<(), appcui::system::Error> {
    let mut a = App::new().build()?;
    a.add_window(my_win::new());
    a.run();
    Ok(())
}
