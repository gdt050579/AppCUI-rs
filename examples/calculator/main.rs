mod expression_parser;

use expression_parser::ExpressionParser;
use appcui::prelude::*;

fn evaluate_expression(expression: &str) -> String {
    let mut expression_parser = ExpressionParser::new(expression);
    match expression_parser.parse_expression() {
        Ok(result) => result.to_string(),
        Err(_) => "Invalid Expression".to_string(),
    }
}

#[Window(events=ButtonEvents+TextFieldEvents )]
struct MyWin {
    b_0: Handle<Button>,
    b_1: Handle<Button>,
    b_2: Handle<Button>,
    b_3: Handle<Button>,
    b_4: Handle<Button>,
    b_5: Handle<Button>,
    b_6: Handle<Button>,
    b_7: Handle<Button>,
    b_8: Handle<Button>,
    b_9: Handle<Button>,
    b_add: Handle<Button>,
    b_sub: Handle<Button>,
    b_mul: Handle<Button>,
    b_div: Handle<Button>,
    b_eq: Handle<Button>,
    b_dot: Handle<Button>,
    b_clear: Handle<Button>,
    display: Handle<TextField>,
}

impl MyWin {
    fn new() -> Self {
        let mut w = Self {
            base: window!("Calculator, d:c, w:47, h:17"),
            b_0: Handle::None,
            b_1: Handle::None,
            b_2: Handle::None,
            b_3: Handle::None,
            b_4: Handle::None,
            b_5: Handle::None,
            b_6: Handle::None,
            b_7: Handle::None,
            b_8: Handle::None,
            b_9: Handle::None,
            b_add: Handle::None,
            b_sub: Handle::None,
            b_mul: Handle::None,
            b_div: Handle::None,
            b_eq: Handle::None,
            b_dot: Handle::None,
            b_clear: Handle::None,
            display: Handle::None,
        };

        w.b_7 = w.add(button!("7, x:5,  y:4, w:8"));
        w.b_8 = w.add(button!("8, x:14, y:4, w:8"));
        w.b_9 = w.add(button!("9, x:23, y:4, w:8"));
        w.b_div = w.add(button!("/, x:32, y:4, w:8"));

        w.b_4 = w.add(button!("4, x:5,  y:6, w:8"));
        w.b_5 = w.add(button!("5, x:14, y:6, w:8"));
        w.b_6 = w.add(button!("6, x:23, y:6, w:8"));
        w.b_mul = w.add(button!("*, x:32, y:6, w:8"));

        w.b_1 = w.add(button!("1, x:5,  y:8, w:8"));
        w.b_2 = w.add(button!("2, x:14, y:8, w:8"));
        w.b_3 = w.add(button!("3, x:23, y:8, w:8"));
        w.b_sub = w.add(button!("- , x:32, y:8, w:8"));

        w.b_0 = w.add(button!("0, x:5,  y:10, w:8"));
        w.b_dot = w.add(button!("., x:14, y:10, w:8"));
        w.b_eq = w.add(button!("'=', x:23, y:10, w:8"));
        w.b_add = w.add(button!("+, x:32, y:10, w:8"));

        w.b_clear = w.add(button!("C, x:5, y:12, w:35"));

        w.display = w.add(textfield!("0, x:5, y:1, w: 35, h:2, flags:ProcessEnter"));

        w
    }

    fn append_to_display(&mut self, value: &str) {
        let d = self.display;
        if let Some(display) = self.control_mut(d) {
            let current_text = display.text().to_string();
            let new_text = if current_text == "0" {
                value.to_string()
            } else {
                current_text + value
            };
            display.set_text(&new_text);
        }
    }
}

impl TextFieldEvents for MyWin {
    fn on_validate(&mut self, _handle: Handle<TextField>, _text: &str) -> EventProcessStatus {
        let d = self.display;
        let result = evaluate_expression(&_text);
        if let Some(display) = self.control_mut(d) {
            display.set_text(&result);
        }
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for MyWin {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if let Some(button) = self.control(handle) {
            let content = button.caption().to_string();
            match content.as_str() {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" | "+" | "*" | "/" | "." => {
                    self.append_to_display(&content);
                }
                "=" => {
                    let d = self.display;
                    if let Some(display) = self.control_mut(d) {
                        let input_text = display.text().to_string();
                        let result = evaluate_expression(&input_text);
                        display.set_text(&result);
                    }
                }
                "C" => {
                    let d = self.display;
                    if let Some(display) = self.control_mut(d) {
                        display.set_text("0");
                    }
                }
                _ => {}
            }
        }
        EventProcessStatus::Processed
    }
}

fn main() -> Result<(), appcui::system::Error> {
    let mut app = App::new().build()?;
    app.add_window(MyWin::new());
    app.run();
    Ok(())
}
