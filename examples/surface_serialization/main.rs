use appcui::prelude::*;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_content = fs::read_to_string("examples/surface_serialization/raw.txt")?;

    let surface = Surface::from_string(&raw_content, Size::new(240, 128));
    let mut app = App::new().single_window().build()?;

    let srf_path = Path::new("examples/surface_serialization/output.srf");
    surface.save(srf_path)?;
    println!("Surface saved to: {}", srf_path.display());

    let loaded_surface = Surface::from_file(srf_path).map_err(|e| format!("Failed to load surface: {}", e))?;
    println!("Surface loaded from: {}", srf_path.display());

    let mut win = Window::new(
        "Surface Serialization Demo",
        Layout::new("d:c,w:100%,h:100%"),
        window::Flags::NoCloseButton,
    );

    let mut canvas = Canvas::new(loaded_surface.size(), Layout::new("l:0,t:0,r:0,b:0"), canvas::Flags::ScrollBars);
    let drawing_surface = canvas.drawing_surface_mut();
    drawing_surface.draw_surface(0, 0, &loaded_surface);

    win.add(canvas);

    app.add_window(win);
    app.run();

    Ok(())
}
