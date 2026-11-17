use appcui::prelude::*;

fn main() -> Result<(), appcui::system::Error> {
    let choice = std::env::args().nth(1).unwrap_or_else(|| "classic".to_string());
    let (title, window_type, background, flags) = match choice.as_str() {
        "classic" => ("Classic", window::Type::Classic, window::Background::Normal, window::Flags::None),
        "rounded" => ("Rounded", window::Type::Rounded, window::Background::Normal, window::Flags::None),
        "panel" => ("Panel", window::Type::Panel, window::Background::Normal, window::Flags::None),
        "error" => ("Error", window::Type::Classic, window::Background::Error, window::Flags::None),
        "warning" => ("Warning", window::Type::Classic, window::Background::Warning, window::Flags::None),
        "notification" => (
            "Notification",
            window::Type::Classic,
            window::Background::Notification,
            window::Flags::None,
        ),
        "sizeable" => ("Sizeable", window::Type::Classic, window::Background::Normal, window::Flags::Sizeable),
        "no-close" => (
            "NoCloseButton",
            window::Type::Classic,
            window::Background::Normal,
            window::Flags::NoCloseButton,
        ),
        "fixed-position" => (
            "FixedPosition",
            window::Type::Classic,
            window::Background::Normal,
            window::Flags::FixedPosition,
        ),
        _ => {
            eprintln!("Choose classic, rounded, panel, error, warning, notification, sizeable, no-close, or fixed-position.");
            std::process::exit(2);
        }
    };

    App::new()
        .size(Size::new(32, 9))
        .color_schema(false)
        .window(move || Window::with_type(title, layout!("a:c,w:30,h:7"), flags, window_type, background))
        .run()
}
