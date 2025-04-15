use super::helpers::{compile_shader, link_program};
use crate::{
    prelude::{ErrorKind, Size, Surface},
    system::Error,
    terminals::{SystemEvent, Terminal},
};
use std::sync::{
    mpsc::Sender,
    {Arc, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, EventTarget, HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGlBuffer, WebGlProgram,
    WebGlRenderingContext as GL, WheelEvent,
};

pub struct WebTerminal {
    gl:                     GL,
    pub size:               Size,
    pub webgl_canvas:       HtmlCanvasElement,
    pub text_canvas:        HtmlCanvasElement,
    program:                WebGlProgram,
    buffer:                 WebGlBuffer,
    pos_attrib_location:    u32,
    color_attrib_location:  u32,
    pub(super) event_queue: Arc<Mutex<Vec<SystemEvent>>>,
    pub font:               String,
    cell_width_px:          f32,
    cell_height_px:         f32,
}

unsafe impl Send for WebTerminal {}
unsafe impl Sync for WebTerminal {}

impl WebTerminal {
    pub(crate) fn new(builder: &crate::system::Builder, sender: Sender<SystemEvent>) -> Result<Self, Error> {
        let document = Self::document()?;

        if let Some(title) = &builder.title {
            document.set_title(&title);
        } else {
            document.set_title("AppCUI Web Terminal");
        }

        let cols = document
            .get_element_by_id("terminal-cols")
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(211);

        let rows = document
            .get_element_by_id("terminal-rows")
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(56);

        let font = document
            .get_element_by_id("terminal-font")
            .and_then(|el| el.text_content())
            .unwrap_or("Consolas Mono, monospace".to_string());

        let font_size = document
            .get_element_by_id("terminal-font-size")
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(20);

        let cell_width_px = document
            .get_element_by_id("terminal-cell-width")
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(9);
        let cell_height_px = document
            .get_element_by_id("terminal-cell-height")
            .and_then(|el| el.text_content())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(font_size);

        let webgl_canvas = Self::get_canvas(&document, "canvas")?;
        let text_canvas = Self::get_canvas(&document, "textCanvas")?;

        let canvas_width = cols * cell_width_px;
        let canvas_height = rows * cell_height_px;

        let gl = Self::get_webgl_context(&webgl_canvas)?;

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        webgl_canvas
            .style()
            .set_property("background-color", "transparent")
            .expect("Failed to set canvas background color");
        text_canvas
            .style()
            .set_property("background-color", "transparent")
            .expect("Failed to set text_canvas background color");

        webgl_canvas.set_width(canvas_width);
        webgl_canvas.set_height(canvas_height);
        text_canvas.set_width(canvas_width);
        text_canvas.set_height(canvas_height);

        text_canvas
            .style()
            .set_property("background-color", "transparent")
            .expect("Failed to set canvas background color");

        webgl_canvas
            .style()
            .set_property("width", &format!("{}px", canvas_width))
            .expect("Failed to set canvas CSS width");
        webgl_canvas
            .style()
            .set_property("height", &format!("{}px", canvas_height))
            .expect("Failed to set canvas CSS height");

        text_canvas
            .style()
            .set_property("width", &format!("{}px", canvas_width))
            .expect("Failed to set canvas CSS width");
        text_canvas
            .style()
            .set_property("height", &format!("{}px", canvas_height))
            .expect("Failed to set canvas CSS height");

        webgl_canvas
            .style()
            .set_property("position", "absolute")
            .expect("Failed to set canvas CSS position");
        webgl_canvas.style().set_property("z-index", "1").expect("Failed to set canvas z-index");
        text_canvas
            .style()
            .set_property("position", "absolute")
            .expect("Failed to set canvas CSS position");
        text_canvas.style().set_property("z-index", "2").expect("Failed to set canvas z-index");

        gl.viewport(0, 0, canvas_width as i32, canvas_height as i32);

        let program = Self::init_program(&gl)?;
        let pos_attrib_location = gl.get_attrib_location(&program, "position") as u32;
        let color_attrib_location = gl.get_attrib_location(&program, "color") as u32;

        let buffer = gl
            .create_buffer()
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "Failed to create WebGL buffer!".to_string()))?;

        let queue = Arc::new(Mutex::new(Vec::new()));
        let font = format!("{font_size}px {font}");

        let terminal = Self {
            gl,
            webgl_canvas,
            text_canvas,
            program,
            buffer,
            pos_attrib_location,
            color_attrib_location,
            event_queue: queue,
            size: Size { width: cols, height: rows },
            font,
            cell_width_px: cell_width_px as f32,
            cell_height_px: cell_height_px as f32,
        };

        terminal
            .setup_input_listeners(sender)
            .map_err(|_| Error::new(ErrorKind::InitializationFailure, "Failed to initialize input listeners!".to_string()))?;

        Ok(terminal)
    }

    fn document() -> Result<web_sys::Document, Error> {
        let window = window().ok_or_else(|| {
            Error::new(
                ErrorKind::InitializationFailure,
                "Failed to initialize web terminal! No window found.".to_string(),
            )
        })?;
        window.document().ok_or_else(|| {
            Error::new(
                ErrorKind::InitializationFailure,
                "Failed to initialize web terminal! No document found.".to_string(),
            )
        })
    }

    fn get_canvas(document: &web_sys::Document, id: &str) -> Result<HtmlCanvasElement, Error> {
        document
            .get_element_by_id(id)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Failed to initialize web terminal! No element with id '{id}' found."),
                )
            })?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| {
                Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Failed to cast element '{id}' to a HtmlCanvasElement."),
                )
            })
    }

    fn get_webgl_context(canvas: &HtmlCanvasElement) -> Result<GL, Error> {
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(&opts, &"alpha".into(), &true.into()).unwrap();
        canvas
            .get_context_with_context_options("webgl", &opts)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Error getting WebGL context: {e:?}")))?
            .ok_or_else(|| Error::new(ErrorKind::InitializationFailure, "WebGL not supported".to_string()))?
            .dyn_into::<GL>()
            .map_err(|e| {
                Error::new(
                    ErrorKind::InitializationFailure,
                    format!("Failed to cast context to WebGlRenderingContext: {e:?}"),
                )
            })
    }

    fn init_program(gl: &GL) -> Result<WebGlProgram, Error> {
        let vertex_shader_source = r#"
            attribute vec2 position;
            attribute vec4 color;
            varying vec4 v_color;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                v_color = color;
            }
        "#;

        let fragment_shader_source = r#"
            precision mediump float;
            varying vec4 v_color;
            void main() {
                gl_FragColor = v_color;
            }
        "#;

        let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, vertex_shader_source)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Vertex shader error: {e}")))?;
        let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, fragment_shader_source)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Fragment shader error: {e}")))?;
        link_program(gl, &vertex_shader, &fragment_shader)
            .map_err(|e| Error::new(ErrorKind::InitializationFailure, format!("Program linking error: {e}")))
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

    pub fn setup_input_listeners(&self, sender: Sender<SystemEvent>) -> Result<(), JsValue> {
        let document = window().ok_or("No window")?.document().ok_or("No document")?;
        let doc_target: &EventTarget = document.as_ref();

        let cell_width = self.webgl_canvas.width() as f32 / (211 as f32);
        let cell_height = self.webgl_canvas.height() as f32 / (56 as f32);

        let sender_key = sender.clone();
        let event_queue = self.event_queue.clone();

        let keydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key_str = event.key();
            if matches!(key_str.as_str(), "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10") {
                event.prevent_default();
            }

            let index: u8 = match key_str.as_str() {
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
                "A" | "a" => 28,
                "B" | "b" => 29,
                "C" | "c" => 30,
                "D" | "d" => 31,
                "E" | "e" => 32,
                "F" | "f" => 33,
                "G" | "g" => 34,
                "H" | "h" => 35,
                "I" | "i" => 36,
                "J" | "j" => 37,
                "K" | "k" => 38,
                "L" | "l" => 39,
                "M" | "m" => 40,
                "N" | "n" => 41,
                "O" | "o" => 42,
                "P" | "p" => 43,
                "Q" | "q" => 44,
                "R" | "r" => 45,
                "S" | "s" => 46,
                "T" | "t" => 47,
                "U" | "u" => 48,
                "V" | "v" => 49,
                "W" | "w" => 50,
                "X" | "x" => 51,
                "Y" | "y" => 52,
                "Z" | "z" => 53,
                "0" => 54,
                "1" => 55,
                "2" => 56,
                "3" => 57,
                "4" => 58,
                "5" => 59,
                "6" => 60,
                "7" => 61,
                "8" => 62,
                "9" => 63,
                _ => 0,
            };

            let key_code = crate::input::KeyCode::from(index);

            let mut modifiers = crate::input::KeyModifier::None;
            if event.alt_key() {
                modifiers |= crate::input::KeyModifier::Alt;
            }
            if event.ctrl_key() {
                modifiers |= crate::input::KeyModifier::Ctrl;
            }
            if event.shift_key() {
                modifiers |= crate::input::KeyModifier::Shift;
            }

            let character = if key_str.chars().count() == 1 {
                let c = key_str.chars().next().unwrap();
                if c.is_alphanumeric() {
                    c
                } else {
                    '\0'
                }
            } else {
                '\0'
            };
            let sys_event = SystemEvent::KeyPressed(crate::terminals::KeyPressedEvent {
                key: crate::input::Key::new(key_code, modifiers),
                character,
            });
            // web_sys::console::log_1(&format!("Key event: {:?}", sys_event).into());

            let _ = sender_key.send(sys_event);
            if let Ok(mut q) = event_queue.lock() {
                q.push(sys_event);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        doc_target.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())?;
        keydown.forget();

        let canvas_target: &EventTarget = self.webgl_canvas.as_ref();
        let canvas = self.webgl_canvas.clone();
        let event_queue = self.event_queue.clone();

        let sender_mouse = sender.clone();
        let mousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = canvas.get_bounding_client_rect();
            let canvas_x = event.client_x() as f32 - rect.left() as f32;
            let canvas_y = event.client_y() as f32 - rect.top() as f32;

            let sys_event = SystemEvent::MouseMove(crate::terminals::MouseMoveEvent {
                x:      (canvas_x as f32 / cell_width) as i32,
                y:      (canvas_y as f32 / cell_height) as i32,
                button: crate::input::MouseButton::None,
            });

            let _ = sender_mouse.send(sys_event);
            if let Ok(mut q) = event_queue.lock() {
                q.push(sys_event);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas_target.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())?;
        mousemove.forget();

        let canvas = self.webgl_canvas.clone();
        let sender_mousedown = sender.clone();
        let event_queue = self.event_queue.clone();
        let mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = canvas.get_bounding_client_rect();
            let canvas_x = event.client_x() as f32 - rect.left() as f32;
            let canvas_y = event.client_y() as f32 - rect.top() as f32;

            let button = match event.button() {
                0 => crate::input::MouseButton::Left,
                1 => crate::input::MouseButton::Center,
                2 => crate::input::MouseButton::Right,
                _ => crate::input::MouseButton::None,
            };
            let sys_event = SystemEvent::MouseButtonDown(crate::terminals::MouseButtonDownEvent {
                x: (canvas_x as f32 / cell_width) as i32,
                y: (canvas_y as f32 / cell_height) as i32,
                button,
            });

            let _ = sender_mousedown.send(sys_event);
            if let Ok(mut q) = event_queue.lock() {
                q.push(sys_event);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas_target.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref())?;
        mousedown.forget();

        let sender_mouseup = sender.clone();
        let event_queue = self.event_queue.clone();
        let canvas = self.webgl_canvas.clone();

        let mouseup = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = canvas.get_bounding_client_rect();
            let canvas_x = event.client_x() as f32 - rect.left() as f32;
            let canvas_y = event.client_y() as f32 - rect.top() as f32;

            let button = match event.button() {
                0 => crate::input::MouseButton::Left,
                1 => crate::input::MouseButton::Center,
                2 => crate::input::MouseButton::Right,
                _ => crate::input::MouseButton::None,
            };
            let sys_event = SystemEvent::MouseButtonUp(crate::terminals::MouseButtonUpEvent {
                x: (canvas_x as f32 / cell_width) as i32,
                y: (canvas_y as f32 / cell_height) as i32,
                button,
            });

            let _ = sender_mouseup.send(sys_event);
            if let Ok(mut q) = event_queue.lock() {
                q.push(sys_event);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas_target.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())?;
        mouseup.forget();

        let sender_wheel = sender.clone();
        let event_queue = self.event_queue.clone();
        let canvas = self.webgl_canvas.clone();

        let wheel = Closure::wrap(Box::new(move |event: WheelEvent| {
            let delta_y = event.delta_y();
            let direction = if delta_y < 0.0 {
                crate::input::MouseWheelDirection::Up
            } else {
                crate::input::MouseWheelDirection::Down
            };
            let rect = canvas.get_bounding_client_rect();
            let canvas_x = event.client_x() as f32 - rect.left() as f32;
            let canvas_y = event.client_y() as f32 - rect.top() as f32;

            let sys_event = SystemEvent::MouseWheel(crate::terminals::MouseWheelEvent {
                x: (canvas_x as f32 / cell_width) as i32,
                y: (canvas_y as f32 / cell_height) as i32,
                direction,
            });

            let _ = sender_wheel.send(sys_event);
            if let Ok(mut q) = event_queue.lock() {
                q.push(sys_event);
            }
            event.prevent_default();
        }) as Box<dyn FnMut(WheelEvent)>);
        canvas_target.add_event_listener_with_callback("wheel", wheel.as_ref().unchecked_ref())?;
        wheel.forget();

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
