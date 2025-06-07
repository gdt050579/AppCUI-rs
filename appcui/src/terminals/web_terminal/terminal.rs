use crate::{
    prelude::{CharFlags, Color, ErrorKind, Key, KeyCode, KeyModifier, MouseButton, MouseWheelDirection, Size, Surface},
    system::Error,
    terminals::{KeyPressedEvent, MouseButtonDownEvent, MouseButtonUpEvent, MouseMoveEvent, MouseWheelEvent, SystemEvent, Terminal},
};
use std::sync::{mpsc::Sender, Arc, Mutex};
use wasm_bindgen::{convert::FromWasmAbi, prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGlBuffer, WebGlProgram,
    WebGlRenderingContext as GL, WheelEvent,
};

const CURSOR_COLOR: &str = "rgba(255, 255, 255, 0.5)";

struct TerminalDomConfig {
    cols: u32,
    rows: u32,
    font_family: String,
    font_size: u32,
    cell_w: u32,
    cell_h: u32,
}

struct WebGLResources {
    gl: GL,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    pos_attrib_location: u32,
    color_attrib_location: u32,
}

pub struct WebTerminal {
    gl: GL,
    size: Size,
    webgl_canvas: HtmlCanvasElement,
    text_canvas: HtmlCanvasElement,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    pos_attrib_location: u32,
    color_attrib_location: u32,
    event_queue: Arc<Mutex<Vec<SystemEvent>>>,
    font: String,
    cell_width_px: f32,
    cell_height_px: f32,
    clipboard_content: Arc<Mutex<Option<String>>>,
}

unsafe impl Send for WebTerminal {}
unsafe impl Sync for WebTerminal {}

impl WebTerminal {
    fn load_dom_config(document: &web_sys::Document) -> TerminalDomConfig {
        let font_size_val = Self::get_config(document, "terminal-font-size", 20);
        TerminalDomConfig {
            cols: Self::get_config(document, "terminal-cols", 211),
            rows: Self::get_config(document, "terminal-rows", 56),
            font_family: Self::get_config(document, "terminal-font", "Consolas Mono, monospace".to_string()),
            font_size: font_size_val,
            cell_w: Self::get_config(document, "terminal-cell-width", 9),
            cell_h: Self::get_config(document, "terminal-cell-height", font_size_val),
        }
    }

    fn initialize_canvases_from_dom(
        document: &web_sys::Document,
        config: &TerminalDomConfig,
    ) -> Result<(HtmlCanvasElement, HtmlCanvasElement), Error> {
        let webgl_canvas = Self::get_canvas(document, "canvas")?;
        let text_canvas = Self::get_canvas(document, "textCanvas")?;
        Self::init_canvas(&webgl_canvas, config.cols, config.rows, config.cell_w, config.cell_h, 1)?;
        Self::init_canvas(&text_canvas, config.cols, config.rows, config.cell_w, config.cell_h, 2)?;
        Ok((webgl_canvas, text_canvas))
    }

    fn initialize_webgl_resources(webgl_canvas: &HtmlCanvasElement, config: &TerminalDomConfig) -> Result<WebGLResources, Error> {
        let gl = Self::get_gl(webgl_canvas)?;
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.viewport(0, 0, (config.cols * config.cell_w) as i32, (config.rows * config.cell_h) as i32);

        let program = Self::init_program(&gl)?;
        let pos_attrib_location = gl.get_attrib_location(&program, "position") as u32;
        let color_attrib_location = gl.get_attrib_location(&program, "color") as u32;
        let buffer = gl
            .create_buffer()
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "Failed to create WebGL buffer".into()))?;

        Ok(WebGLResources {
            gl,
            program,
            buffer,
            pos_attrib_location,
            color_attrib_location,
        })
    }

    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let document = Self::document()?;
        document.set_title(builder.title.as_deref().unwrap_or("AppCUI Web Terminal"));

        let dom_config = Self::load_dom_config(&document);
        let (webgl_canvas, text_canvas) = Self::initialize_canvases_from_dom(&document, &dom_config)?;
        let webgl_resources = Self::initialize_webgl_resources(&webgl_canvas, &dom_config)?;

        let event_queue = Arc::new(Mutex::new(Vec::new()));
        let font_style = format!("{}px {}", dom_config.font_size, dom_config.font_family);

        let term = WebTerminal {
            gl: webgl_resources.gl,
            size: Size {
                width: dom_config.cols,
                height: dom_config.rows,
            },
            webgl_canvas,
            text_canvas,
            program: webgl_resources.program,
            buffer: webgl_resources.buffer,
            pos_attrib_location: webgl_resources.pos_attrib_location,
            color_attrib_location: webgl_resources.color_attrib_location,
            event_queue: event_queue.clone(),
            font: font_style,
            cell_width_px: dom_config.cell_w as f32,
            cell_height_px: dom_config.cell_h as f32,
            clipboard_content: Arc::new(Mutex::new(None)),
        };

        term.setup_input_listeners(&document, sender)
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
        js_sys::Reflect::set(&opts, &"alpha".into(), &true.into()).map_err(|e| {
            Error::new(
                ErrorKind::InitializationFailure,
                format!("Failed to configure WebGL context options: {e:?}"),
            )
        })?;
        canvas
            .get_context_with_context_options("webgl", &opts)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Error getting WebGL context: {e:?}")))?
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "WebGL not supported".into()))?
            .dyn_into::<GL>()
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Failed to cast context: {:?}", e)))
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

    fn create_mouse_event_handler<F>(&self, sender: Sender<SystemEvent>, event_mapper: F) -> Closure<dyn FnMut(MouseEvent)>
    where
        F: Fn(i32, i32, &MouseEvent) -> SystemEvent + 'static,
    {
        let queue = self.event_queue.clone();
        let canvas = self.webgl_canvas.clone();
        let cw = self.cell_width_px;
        let ch = self.cell_height_px;

        Closure::wrap(Box::new(move |event: MouseEvent| {
            let r = canvas.get_bounding_client_rect();
            let x = ((event.client_x() as f32 - r.left() as f32) / cw) as i32;
            let y = ((event.client_y() as f32 - r.top() as f32) / ch) as i32;

            let system_event = event_mapper(x, y, &event);

            if sender.send(system_event.clone()).is_ok() {
                if let Ok(mut q) = queue.lock() {
                    q.push(system_event);
                }
            }
        }))
    }

    fn create_wheel_event_handler<F>(&self, sender: Sender<SystemEvent>, event_mapper: F) -> Closure<dyn FnMut(WheelEvent)>
    where
        F: Fn(i32, i32, &WheelEvent) -> SystemEvent + 'static,
    {
        let queue = self.event_queue.clone();
        let canvas = self.webgl_canvas.clone();
        let cw = self.cell_width_px;
        let ch = self.cell_height_px;

        Closure::wrap(Box::new(move |event: WheelEvent| {
            let r = canvas.get_bounding_client_rect();
            let x = ((event.client_x() as f32 - r.left() as f32) / cw) as i32;
            let y = ((event.client_y() as f32 - r.top() as f32) / ch) as i32;

            let system_event = event_mapper(x, y, &event);

            if sender.send(system_event.clone()).is_ok() {
                if let Ok(mut q) = queue.lock() {
                    q.push(system_event);
                }
            }
            event.prevent_default();
        }))
    }

    fn setup_input_listeners(&self, document: &web_sys::Document, sender: Sender<SystemEvent>) -> Result<(), JsValue> {
        let target: &EventTarget = document.as_ref();

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
            let keyboard_sender = sender.clone();
            let queue = self.event_queue.clone();
            let key_closure: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new(move |event| {
                let k = event.key();
                let idx = WebTerminal::key_index(&k);
                // Prevent default browser actions for F1-F10 keys
                if (1..=10).contains(&idx) {
                    event.prevent_default();
                }

                let key_code = KeyCode::from(idx);
                let mut mods = KeyModifier::None;
                if event.alt_key() {
                    mods |= KeyModifier::Alt;
                }
                if event.ctrl_key() {
                    mods |= KeyModifier::Ctrl;
                }
                if event.shift_key() {
                    mods |= KeyModifier::Shift;
                }

                let character = if k.len() == 1 { k.chars().next().unwrap_or('\0') } else { '\0' };

                let sys_event = SystemEvent::KeyPressed(KeyPressedEvent {
                    key: Key::new(key_code, mods),
                    character,
                });

                if keyboard_sender.send(sys_event.clone()).is_ok() {
                    if let Ok(mut q) = queue.lock() {
                        q.push(sys_event);
                    }
                }
            }));
            attach::<KeyboardEvent>(target, "keydown", key_closure)?;
        }

        // MOUSE MOVE
        let mouse_move_closure = self.create_mouse_event_handler(sender.clone(), |x, y, _event| {
            SystemEvent::MouseMove(MouseMoveEvent {
                x,
                y,
                button: MouseButton::None,
            })
        });
        attach::<MouseEvent>(target, "mousemove", mouse_move_closure)?;

        // MOUSE DOWN
        let mouse_down_closure = self.create_mouse_event_handler(sender.clone(), |x, y, event| {
            let button = match event.button() {
                0 => MouseButton::Left,
                1 => MouseButton::Center,
                2 => MouseButton::Right,
                _ => MouseButton::None,
            };
            SystemEvent::MouseButtonDown(MouseButtonDownEvent { x, y, button })
        });
        attach::<MouseEvent>(target, "mousedown", mouse_down_closure)?;

        // MOUSE UP
        let mouse_up_closure = self.create_mouse_event_handler(sender.clone(), |x, y, event| {
            let button = match event.button() {
                0 => crate::input::MouseButton::Left,
                1 => crate::input::MouseButton::Center,
                2 => crate::input::MouseButton::Right,
                _ => crate::input::MouseButton::None,
            };
            SystemEvent::MouseButtonUp(MouseButtonUpEvent { x, y, button })
        });
        attach::<MouseEvent>(target, "mouseup", mouse_up_closure)?;

        // MOUSE WHEEL
        let wheel_closure = self.create_wheel_event_handler(sender, |x, y, event| {
            let direction = if event.delta_y() < 0.0 {
                MouseWheelDirection::Up
            } else {
                MouseWheelDirection::Down
            };
            SystemEvent::MouseWheel(MouseWheelEvent { x, y, direction })
        });
        attach::<WheelEvent>(target, "wheel", wheel_closure)?;

        Ok(())
    }

    fn render_background(&self, surface: &Surface) {
        let canvas_width = self.webgl_canvas.width() as f32;
        let canvas_height = self.webgl_canvas.height() as f32;
        let cell_width = self.cell_width_px;
        let cell_height = self.cell_height_px;

        let width = surface.size.width as i32;
        let height = surface.size.height as i32;
        let mut vertices: Vec<f32> = Vec::new();

        for global_y in 0..height {
            for global_x in 0..width {
                if let Some(cell) = &surface.char(global_x, global_y) {
                    if cell.background == Color::Transparent {
                        // web_sys::console::log_1(&format!("Skipping transparent cell at ({}, {})", global_x, global_y).into());
                        continue;
                    }
                    let pos_x = global_x as f32 * cell_width;
                    let pos_y = global_y as f32 * cell_height;
                    let bg_color = self.color_to_rgba(cell.background);

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

        // clear entire canvas
        context.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        context.save();
        context.set_font(self.font.as_str());
        context.set_text_baseline("top");
        context.set_text_align("center");

        let width = surface.size.width as i32;
        let height = surface.size.height as i32;
        for global_y in 0..height {
            for global_x in 0..width {
                if let Some(cell) = &surface.char(global_x, global_y) {
                    if cell.foreground == Color::Transparent {
                        // web_sys::console::log_1(&format!("Skipping transparent cell at ({}, {})", global_x, global_y).into());
                        continue;
                    }
                    let pos_x = global_x as f64 * cell_width;
                    let pos_y = global_y as f64 * cell_height;

                    let foreground = self.color_to_rgba(cell.foreground);
                    let css_color = format!(
                        "rgba({},{},{},{})",
                        (foreground[0] * 255.0) as u8,
                        (foreground[1] * 255.0) as u8,
                        (foreground[2] * 255.0) as u8,
                        foreground[3],
                    );
                    context.set_fill_style_str(&css_color);
                    context.fill_text(&cell.code.to_string(), pos_x.into(), pos_y.into())?;

                    if cell.flags.contains(CharFlags::Underline) {
                        context.begin_path();
                        context.set_stroke_style_str(&css_color);
                        let x_start = pos_x + 1.0;
                        let x_end = pos_x + cell_width - 1.0;
                        let y_line = pos_y + cell_height - 1.0;
                        context.move_to(x_start, y_line);
                        context.line_to(x_end, y_line);
                        context.stroke();
                        // restore fill style for next character
                        context.set_fill_style_str(&css_color);
                    }
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

        context.set_fill_style_str(CURSOR_COLOR);
        context.fill_rect(cursor_x * cell_width, cursor_y * cell_height, cell_width, cell_height);

        Ok(())
    }
}

impl Terminal for WebTerminal {
    fn on_resize(&mut self, new_size: Size) {
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
        self.render_background(surface);
        if let Err(e) = self.render_text(surface) {
            web_sys::console::log_1(&format!("Error rendering text: {e:?}").into());
        }

        if let Err(e) = self.render_cursor(surface) {
            web_sys::console::log_1(&format!("Error rendering cursor: {e:?}").into());
        }
    }

    fn get_size(&self) -> crate::prelude::Size {
        self.size
    }

    fn get_clipboard_text(&self) -> Option<String> {
        self.clipboard_content.lock().unwrap().clone()
    }

    fn has_clipboard_text(&self) -> bool {
        self.clipboard_content.lock().unwrap().is_some()
    }

    fn query_system_event(&mut self) -> Option<SystemEvent> {
        let mut queue = self.event_queue.lock().unwrap();
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }

    fn set_clipboard_text(&mut self, text: &str) {
        let text_is_empty = text.is_empty();
        {
            let mut local_clipboard = self.clipboard_content.lock().unwrap();
            if text_is_empty {
                *local_clipboard = None;
            } else {
                *local_clipboard = Some(text.to_string());
            }
        }

        if !text_is_empty {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let text_owned = text.to_string();
                let promise = clipboard.write_text(&text_owned);
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(promise).await {
                        Ok(_) => {
                            web_sys::console::log_1(&format!("Successfully wrote to browser clipboard: '{text_owned}'").into());
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Failed to write to browser clipboard: {e:?}").into());
                        }
                    }
                });
            } else {
                web_sys::console::warn_1(&"Window object not available for clipboard operation.".into());
            }
        }
    }
}
