use appcui::prelude::*;
use std::fs;

fn main() -> Result<(), appcui::system::Error> {
    let file_path = "examples\\exemple.md";
    //let content = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());

    let mut a = App::new().build()?;

    let app_theme = Some(Themes::DarkGray);
    if let Some(theme) = app_theme {
        App::set_theme(Theme::new(theme));
    };
    let mut w = window!("Test,d:c,w:50,h:15,flags:sizeable");
    let mut m = markdown!("
    '
    # My Markdown Example\n\n\
    Welcome to this **Markdown** example! This file *demonstrates* basic Markdown syntax.\n\n\
    ---\n\
    ## Back link\n\
    Some testing for links back \n\n\
    ## Table of Contents\n\n\
    1. [Introduction](#introduction) \n\
    \t1. ana\n\
    \t2. are\n\
    \t3. mere\n\
    2. [Features](#features)\n\
    \t1. gigi\n\
    \t\t1. vine\n\
    \t\t2. cere\n\
    3. [Conclusion](#conclusion)\n\n\
    ## Introduction\n\n\
    Markdown is a lightweight markup language for creating formatted text using a plain-text editor.\n\n\
    ### Why use Markdown?\n\n\
    - **Easy to learn**: Simple syntax `__bold__` A **ceva**.\n\
    - __Readable__: Looks great as plain text.\n\
    - **Flexible**: Converts to HTML and other formats.\n\n\
    ## Features\n\n\
    Here are some Markdown features:\n\n\
    ### Lists\n\n\
    #### Unordered\n\n\
    - Item 1\n\
    \t- Sub-item 1.1\n\
    \t- Sub-item 1.2\n\
    \t\t- Subitem 1.2.1\n\
    \t\t- Subitem 1.2.2\n\
    \t- Item 1.3\n\
    - Item 2\n\n\
    #### Code Blocks\n\n\
    Here is an example of inline code: `let x = 10;`\n\n\
    And here is a code block:\n\n\
    ```\n\
    fn main() {
        println!(\"Hello, world!\");
    }
    ```\n\
    [Get me back](#back-link)\n\n\
    | Column 1 | Column 2|\n\
    | - | --- |\n\
    | Cell 1, __Row 1__ | Cell 2, Row 1 |\n\
    | Cell 1, Row 2 | Cell 1, Row 2 |\n
    ',d:c,flags:ScrollBars");

    
    //m = Markdown::new(&content,Layout::new("d: c"),markdown::Flags::ScrollBars,);
    
    w.add(m);
    a.add_window(w);
    a.run();
    Ok(())
}
