use super::helpers::{compile_shader, link_program};
use crate::{
    prelude::{ErrorKind, Size, Surface},
    system::Error,
    terminals::{SystemEvent, Terminal},
};
use std::sync::{mpsc::Sender, Arc, Mutex};
use wasm_bindgen::{convert::FromWasmAbi, prelude::*, JsCast};
use web_sys::{
    window, CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGlBuffer, WebGlProgram,
    WebGlRenderingContext as GL, WheelEvent,
};

pub struct WebTerminal {
    gl:                    GL,
    size:                  Size,
    webgl_canvas:          HtmlCanvasElement,
    text_canvas:           HtmlCanvasElement,
    program:               WebGlProgram,
    buffer:                WebGlBuffer,
    pos_attrib_location:   u32,
    color_attrib_location: u32,
    event_queue:           Arc<Mutex<Vec<SystemEvent>>>,
    font:                  String,
    cell_width_px:         f32,
    cell_height_px:        f32,
}

unsafe impl Send for WebTerminal {}
unsafe impl Sync for WebTerminal {}

impl WebTerminal {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        // Grab the document and set the window title
        let document = Self::document()?;
        document.set_title(builder.title.as_deref().unwrap_or("AppCUI Web Terminal"));

        // Pull configuration (cols, rows, font, sizes) from <div id="terminal-*">
        let cols = Self::get_config(&document, "terminal-cols", 211);
        let rows = Self::get_config(&document, "terminal-rows", 56);
        let font_family = Self::get_config(&document, "terminal-font", "Consolas Mono, monospace".to_string());
        let font_size = Self::get_config(&document, "terminal-font-size", 20);
        let cell_w = Self::get_config(&document, "terminal-cell-width", 9);
        let cell_h = Self::get_config(&document, "terminal-cell-height", font_size);

        // Fetch canvases and initialize
        let webgl_canvas = Self::get_canvas(&document, "canvas")?;
        let text_canvas = Self::get_canvas(&document, "textCanvas")?;
        Self::init_canvas(&webgl_canvas, cols, rows, cell_w, cell_h, 1)?;
        Self::init_canvas(&text_canvas, cols, rows, cell_w, cell_h, 2)?;

        // Initialize WebGL state
        let gl = Self::get_gl(&webgl_canvas)?;
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.viewport(0, 0, (cols * cell_w) as i32, (rows * cell_h) as i32);

        // Compile + link shader
        let program = Self::init_program(&gl)?;
        let pos_loc = gl.get_attrib_location(&program, "position") as u32;
        let col_loc = gl.get_attrib_location(&program, "color") as u32;
        let buffer = gl
            .create_buffer()
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "Failed to create WebGL buffer".into()))?;

        let queue = Arc::new(Mutex::new(Vec::new()));
        let font = format!("{}px {}", font_size, font_family);

        let term = WebTerminal {
            gl,
            size: Size { width: cols, height: rows },
            webgl_canvas,
            text_canvas,
            program,
            buffer,
            pos_attrib_location: pos_loc,
            color_attrib_location: col_loc,
            event_queue: queue.clone(),
            font,
            cell_width_px: cell_w as f32,
            cell_height_px: cell_h as f32,
        };

        // Hook up event listeners
        term.setup_input_listeners(sender)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Failed to initialize input listeners: {:?}", e)))?;
        Ok(term)
    }

    fn get_config<T: std::str::FromStr>(document: &web_sys::Document, id: &str, default: T) -> T {
        document
            .get_element_by_id(id)
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<T>().ok())
            .unwrap_or(default)
    }

    fn init_canvas(canvas: &HtmlCanvasElement, cols: u32, rows: u32, cell_w: u32, cell_h: u32, z_index: u32) -> Result<(), Error> {
        let width = cols * cell_w;
        let height = rows * cell_h;
        canvas.set_width(width);
        canvas.set_height(height);

        let style = canvas.style();
        for &(prop, ref val) in &[
            ("width", format!("{}px", width)),
            ("height", format!("{}px", height)),
            ("background-color", "transparent".into()),
            ("position", "absolute".into()),
            ("z-index", z_index.to_string()),
        ] {
            style
                .set_property(prop, val)
                .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Failed to set {}: {:?}", prop, e)))?;
        }
        Ok(())
    }

    fn document() -> Result<web_sys::Document, Error> {
        window()
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "No window found".into()))?
            .document()
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "No document found".into()))
    }

    fn get_canvas(document: &web_sys::Document, id: &str) -> Result<HtmlCanvasElement, Error> {
        document
            .get_element_by_id(id)
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, format!("No element with id '{}'", id)))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| Error::new(ErrorKind::InitializationFailure, format!("Element '{}' is not a canvas", id)))
    }

    fn get_gl(canvas: &HtmlCanvasElement) -> Result<GL, Error> {
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(&opts, &"alpha".into(), &true.into()).unwrap();
        canvas
            .get_context_with_context_options("webgl", &opts)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Error getting WebGL context: {:?}", e)))?
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "WebGL not supported".into()))?
            .dyn_into::<GL>()
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Failed to cast context: {:?}", e)))
    }

    fn init_program(gl: &GL) -> Result<WebGlProgram, Error> {
        let vs_src = r#"
            attribute vec2 position;
            attribute vec4 color;
            varying vec4 v_color;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                v_color = color;
            }
        "#;
        let fs_src = r#"
            precision mediump float;
            varying vec4 v_color;
            void main() {
                gl_FragColor = v_color;
            }
        "#;
        let vert = compile_shader(gl, GL::VERTEX_SHADER, vs_src)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Vertex shader error: {}", e)))?;
        let frag = compile_shader(gl, GL::FRAGMENT_SHADER, fs_src)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Fragment shader error: {}", e)))?;
        link_program(gl, &vert, &frag).map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Program linking error: {}", e)))
    }

    fn key_index(k: &str) -> u8 {
        match k {
            "F1" => 1,
            "F2" => 2,
            "F3" => 3,
            "F4" => 4,
            "F5" => 5,
            "F6" => 6,
            "F7" => 7,
            "F8" => 8,
            "F9" => 9,
            "F10" => 10,
            "F11" => 11,
            "F12" => 12,
            "Enter" => 13,
            "Escape" => 14,
            "Insert" => 15,
            "Delete" => 16,
            "Backspace" => 17,
            "Tab" => 18,
            "ArrowLeft" | "Left" => 19,
            "ArrowUp" | "Up" => 20,
            "ArrowDown" | "Down" => 21,
            "ArrowRight" | "Right" => 22,
            "PageUp" => 23,
            "PageDown" => 24,
            "Home" => 25,
            "End" => 26,
            " " | "Space" | "Spacebar" => 27,
            _ => {
                if let Some(ch) = k.chars().next().filter(|_| k.len() == 1) {
                    match ch {
                        'A'..='Z' => ch as u8 - b'A' + 28,
                        'a'..='z' => ch.to_ascii_uppercase() as u8 - b'A' + 28,
                        '0'..='9' => ch as u8 - b'0' + 54,
                        _ => 0,
                    }
                } else {
                    0
                }
            }
        }
    }

    fn setup_input_listeners(&self, sender: Sender<SystemEvent>) -> Result<(), JsValue> {
        let document = window().unwrap().document().unwrap();
        let target: &EventTarget = document.as_ref();

        // Generic helper to attach a Closure<dyn FnMut(E)> listener
        fn attach<E: 'static + JsCast>(target: &EventTarget, event_name: &str, c: Closure<dyn FnMut(E)>) -> Result<(), JsValue>
        where
            E: FromWasmAbi,
        {
            target.add_event_listener_with_callback(event_name, c.as_ref().unchecked_ref())?;
            c.forget();
            Ok(())
        }

        // KEYBOARD
        {
            let sender = sender.clone();
            let queue = self.event_queue.clone();
            let key_closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new(move |event| {
                let k = event.key();
                let idx = WebTerminal::key_index(&k);
                if (1..=10).contains(&idx) {
                    event.prevent_default();
                }
                let key_code = crate::input::KeyCode::from(idx);
                let mut mods = crate::input::KeyModifier::None;
                if event.alt_key() {
                    mods |= crate::input::KeyModifier::Alt;
                }
                if event.ctrl_key() {
                    mods |= crate::input::KeyModifier::Ctrl;
                }
                if event.shift_key() {
                    mods |= crate::input::KeyModifier::Shift;
                }
                let character = k.chars().next().filter(|c| c.is_alphanumeric()).unwrap_or('\0');
                let sys = SystemEvent::KeyPressed(crate::terminals::KeyPressedEvent {
                    key: crate::input::Key::new(key_code, mods),
                    character,
                });
                let _ = sender.send(sys.clone());
                if let Ok(mut q) = queue.lock() {
                    q.push(sys);
                }
            }));
            attach::<KeyboardEvent>(target, "keydown", key_closure)?;
        }

        // MOUSE MOVE
        {
            let sender = sender.clone();
            let queue = self.event_queue.clone();
            let canvas = self.webgl_canvas.clone();
            let cw = self.cell_width_px;
            let ch = self.cell_height_px;
            let mv: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |e| {
                let r = canvas.get_bounding_client_rect();
                let x = ((e.client_x() as f32 - r.left() as f32) / cw) as i32;
                let y = ((e.client_y() as f32 - r.top() as f32) / ch) as i32;
                let sys = SystemEvent::MouseMove(crate::terminals::MouseMoveEvent {
                    x,
                    y,
                    button: crate::input::MouseButton::None,
                });
                let _ = sender.send(sys.clone());
                if let Ok(mut q) = queue.lock() {
                    q.push(sys);
                }
            }));
            attach::<MouseEvent>(target, "mousemove", mv)?;
        }

        // MOUSE DOWN
        {
            let sender = sender.clone();
            let queue = self.event_queue.clone();
            let canvas = self.webgl_canvas.clone();
            let cw = self.cell_width_px;
            let ch = self.cell_height_px;
            let md: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |e| {
                let r = canvas.get_bounding_client_rect();
                let x = ((e.client_x() as f32 - r.left() as f32) / cw) as i32;
                let y = ((e.client_y() as f32 - r.top() as f32) / ch) as i32;
                let b = match e.button() {
                    0 => crate::input::MouseButton::Left,
                    1 => crate::input::MouseButton::Center,
                    2 => crate::input::MouseButton::Right,
                    _ => crate::input::MouseButton::None,
                };
                let sys = SystemEvent::MouseButtonDown(crate::terminals::MouseButtonDownEvent { x, y, button: b });
                let _ = sender.send(sys.clone());
                if let Ok(mut q) = queue.lock() {
                    q.push(sys);
                }
            }));
            attach::<MouseEvent>(target, "mousedown", md)?;
        }

        // MOUSE UP
        {
            let sender = sender.clone();
            let queue = self.event_queue.clone();
            let canvas = self.webgl_canvas.clone();
            let cw = self.cell_width_px;
            let ch = self.cell_height_px;
            let mu: Closure<dyn FnMut(MouseEvent)> = Closure::wrap(Box::new(move |e| {
                let r = canvas.get_bounding_client_rect();
                let x = ((e.client_x() as f32 - r.left() as f32) / cw) as i32;
                let y = ((e.client_y() as f32 - r.top() as f32) / ch) as i32;
                let b = match e.button() {
                    0 => crate::input::MouseButton::Left,
                    1 => crate::input::MouseButton::Center,
                    2 => crate::input::MouseButton::Right,
                    _ => crate::input::MouseButton::None,
                };
                let sys = SystemEvent::MouseButtonUp(crate::terminals::MouseButtonUpEvent { x, y, button: b });
                let _ = sender.send(sys.clone());
                if let Ok(mut q) = queue.lock() {
                    q.push(sys);
                }
            }));
            attach::<MouseEvent>(target, "mouseup", mu)?;
        }

        // MOUSE WHEEL
        {
            let sender = sender.clone();
            let queue = self.event_queue.clone();
            let canvas = self.webgl_canvas.clone();
            let cw = self.cell_width_px;
            let ch = self.cell_height_px;
            let wh: Closure<dyn FnMut(WheelEvent)> = Closure::wrap(Box::new(move |e| {
                let r = canvas.get_bounding_client_rect();
                let x = ((e.client_x() as f32 - r.left() as f32) / cw) as i32;
                let y = ((e.client_y() as f32 - r.top() as f32) / ch) as i32;
                let dir = if e.delta_y() < 0.0 {
                    crate::input::MouseWheelDirection::Up
                } else {
                    crate::input::MouseWheelDirection::Down
                };
                let sys = SystemEvent::MouseWheel(crate::terminals::MouseWheelEvent { x, y, direction: dir });
                let _ = sender.send(sys.clone());
                if let Ok(mut q) = queue.lock() {
                    q.push(sys);
                }
                e.prevent_default();
            }));
            attach::<WheelEvent>(target, "wheel", wh)?;
        }

        Ok(())
    }

    fn render_background(&self, surface: &Surface) {
        let canvas_width = self.webgl_canvas.width() as f32;
        let canvas_height = self.webgl_canvas.height() as f32;
        let cell_width = self.cell_width_px;
        let cell_height = self.cell_height_px;

        let clip_left = surface.clip.left as i32;
        let clip_top = surface.clip.top as i32;
        let clip_right = surface.clip.right as i32;
        let clip_bottom = surface.clip.bottom as i32;

        let mut vertices: Vec<f32> = Vec::new();

        let mut min_px_x = i32::MAX;
        let mut min_px_y = i32::MAX;
        let mut max_px_x = 0;
        let mut max_px_y = 0;

        for global_y in clip_top..=clip_bottom {
            for global_x in clip_left..=clip_right {
                let cell_x = global_x - surface.origin.x;
                let cell_y = global_y - surface.origin.y;
                if let Some(cell) = &surface.char(cell_x, cell_y) {
                    let pos_x = global_x as f32 * cell_width;
                    let pos_y = global_y as f32 * cell_height;
                    let bg_color = cell.background.to_rgba();

                    min_px_x = min_px_x.min(pos_x as i32);
                    min_px_y = min_px_y.min(pos_y as i32);
                    max_px_x = max_px_x.max((pos_x + cell_width) as i32);
                    max_px_y = max_px_y.max((pos_y + cell_height) as i32);

                    let x_ndc = 2.0 * (pos_x / canvas_width) - 1.0;
                    let y_ndc = 1.0 - 2.0 * (pos_y / canvas_height);
                    let w_ndc = 2.0 * (cell_width / canvas_width);
                    let h_ndc = 2.0 * (cell_height / canvas_height);

                    vertices.extend_from_slice(&[
                        x_ndc,
                        y_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                        x_ndc,
                        y_ndc - h_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                        x_ndc + w_ndc,
                        y_ndc - h_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                    ]);
                    vertices.extend_from_slice(&[
                        x_ndc,
                        y_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                        x_ndc + w_ndc,
                        y_ndc - h_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                        x_ndc + w_ndc,
                        y_ndc,
                        bg_color[0],
                        bg_color[1],
                        bg_color[2],
                        bg_color[3],
                    ]);
                }
            }
        }

        let scissor_x = min_px_x.max(0);
        let scissor_y = min_px_y.max(0);
        let scissor_width = (max_px_x - min_px_x).max(0).min(canvas_width as i32 - scissor_x);
        let scissor_height = (max_px_y - min_px_y).max(0).min(canvas_height as i32 - scissor_y);

        // web_sys::console::log_1(&format!("Scissor rect: ({}, {}), {}x{}", scissor_x, scissor_y, scissor_width, scissor_height).into());

        self.gl.enable(GL::SCISSOR_TEST);
        self.gl.scissor(scissor_x, scissor_y, scissor_width, scissor_height);

        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertices);
            self.gl
                .buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_array, GL::DYNAMIC_DRAW);
        }

        self.gl.use_program(Some(&self.program));

        let stride = (6 * std::mem::size_of::<f32>()) as i32;
        self.gl.enable_vertex_attrib_array(self.pos_attrib_location);
        self.gl
            .vertex_attrib_pointer_with_i32(self.pos_attrib_location, 2, GL::FLOAT, false, stride, 0);
        self.gl.enable_vertex_attrib_array(self.color_attrib_location);
        self.gl.vertex_attrib_pointer_with_i32(
            self.color_attrib_location,
            4,
            GL::FLOAT,
            false,
            stride,
            (2 * std::mem::size_of::<f32>()) as i32,
        );

        let vertex_count = (vertices.len() / 6) as i32;
        self.gl.draw_arrays(GL::TRIANGLES, 0, vertex_count);

        self.gl.disable(GL::SCISSOR_TEST);
    }

    fn render_text(&self, surface: &Surface) -> Result<(), JsValue> {
        let context = self
            .text_canvas
            .get_context("2d")?
            .ok_or("2d context not available")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let canvas_width = self.text_canvas.width() as f64;
        let canvas_height = self.text_canvas.height() as f64;
        let cell_width = self.cell_width_px as f64;
        let cell_height = self.cell_height_px as f64;

        // context.clear_rect(
        //     0.0 + surface.clip.left as f64 * cell_width,
        //     0.0 + surface.clip.top as f64 * cell_height,
        //     (surface.clip.right - surface.clip.left) as f64 * cell_width,
        //     (surface.clip.bottom - surface.clip.top) as f64 * cell_height,
        // );

        // context.clear_rect(
        //     (surface.clip.left as f64 * cell_width) as f64,
        //     (surface.clip.top as f64 * cell_height) as f64,
        //     ((surface.clip.right - surface.clip.left) as f64 * cell_width) as f64,
        //     ((surface.clip.bottom - surface.clip.top) as f64 * cell_height) as f64,
        // );

        context.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        context.save();
        context.set_font(self.font.as_str());
        context.set_text_baseline("top");
        context.set_text_align("center");

        let clip_left = surface.clip.left;
        let clip_top = surface.clip.top;
        let clip_right = surface.clip.right;
        let clip_bottom = surface.clip.bottom;

        for global_y in clip_top..=clip_bottom {
            for global_x in clip_left..=clip_right {
                let cell_x = global_x - surface.origin.x as i32;
                let cell_y = global_y - surface.origin.y as i32;
                if let Some(cell) = &surface.char(cell_x, cell_y) {
                    let pos_x = global_x as f64 * cell_width;
                    let pos_y = global_y as f64 * cell_height;
                    let foreground = cell.foreground.to_rgba();
                    let css_color = format!(
                        "rgba({},{},{},{})",
                        (foreground[0] * 255.0) as u8,
                        (foreground[1] * 255.0) as u8,
                        (foreground[2] * 255.0) as u8,
                        foreground[3],
                    );
                    context.set_fill_style(&JsValue::from_str(&css_color));
                    context.fill_text(&cell.code.to_string(), pos_x.into(), pos_y.into())?;
                }
            }
        }

        context.restore();
        Ok(())
    }

    fn render_cursor(&self, surface: &Surface) -> Result<(), JsValue> {
        if !surface.cursor.is_visible() {
            return Ok(());
        }

        let context = self
            .text_canvas
            .get_context("2d")?
            .ok_or("2d context not available")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        let canvas_width = self.text_canvas.width() as f64;
        let canvas_height = self.text_canvas.height() as f64;
        let num_cols = surface.size.width as f64;
        let num_rows = surface.size.height as f64;
        let cell_width = canvas_width / num_cols;
        let cell_height = canvas_height / num_rows;

        let cursor_x = surface.cursor.x as f64;
        let cursor_y = surface.cursor.y as f64;

        context.set_fill_style(&JsValue::from_str("rgba(255,255,255,0.5)"));
        context.fill_rect(cursor_x * cell_width, cursor_y * cell_height, cell_width, cell_height);

        Ok(())
    }
}

impl Terminal for WebTerminal {
    fn on_resize(&mut self, new_size: crate::prelude::Size) {
        self.webgl_canvas.set_width(new_size.width);
        self.webgl_canvas.set_height(new_size.height);
        self.text_canvas.set_width(new_size.width);
        self.text_canvas.set_height(new_size.height);
        self.gl.viewport(0, 0, new_size.width as i32, new_size.height as i32);
    }

    fn is_single_threaded(&self) -> bool {
        false
    }

    fn update_screen(&mut self, surface: &Surface) {
        // web_sys::console::log_1(
        //     &format!(
        //         "Update screen: origin {:?}, base_origin: {:?}, clip: {:?}, base_clip: {:?}, right_most: {}, bottom_most: {}",
        //         surface.origin, surface.base_origin, surface.clip, surface.base_clip, surface.right_most, surface.bottom_most
        //     )
        //     .into(),
        // );
        self.render_background(surface);
        if let Err(e) = self.render_text(surface) {
            web_sys::console::log_1(&format!("Error rendering text: {:?}", e).into());
        }

        if let Err(e) = self.render_cursor(surface) {
            web_sys::console::log_1(&format!("Error rendering cursor: {:?}", e).into());
        }
    }

    fn get_size(&self) -> crate::prelude::Size {
        self.size
    }

    fn get_clipboard_text(&self) -> Option<String> {
        None
    }

    fn has_clipboard_text(&self) -> bool {
        false
    }

    fn query_system_event(&mut self) -> Option<SystemEvent> {
        let mut queue = self.event_queue.lock().unwrap();
        if !queue.is_empty() {
            let event = Some(queue.remove(0));
            // web_sys::console::log_1(&format!("Event: {:?}", event).into());
            event
        } else {
            None
        }
    }

    fn set_clipboard_text(&mut self, _text: &str) {}
}
