use crate::{
    backend::WebTerminal,
    prelude::Color,
    system::{Error, ErrorKind},
};
use web_sys::{WebGlProgram, WebGlRenderingContext as GL, WebGlShader};

pub(super) const VERTEX_SHADER_SOURCE: &str = r#"
    attribute vec2 position;
    attribute vec4 color;
    varying vec4 v_color;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_color = color;
    }
"#;

pub(super) const FRAGMENT_SHADER_SOURCE: &str = r#"
    precision mediump float;
    varying vec4 v_color;
    void main() {
        gl_FragColor = v_color;
    }
"#;

impl WebTerminal {
    pub(super) fn init_program(gl: &GL) -> Result<WebGlProgram, Error> {
        let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, VERTEX_SHADER_SOURCE)
            .map_err(|msg| Error::new(ErrorKind::InitializationFailure, format!("Vertex shader error: {}", msg)))?;
        let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)
            .map_err(|msg| Error::new(ErrorKind::InitializationFailure, format!("Fragment shader error: {}", msg)))?;
        link_program(gl, &vert_shader, &frag_shader)
            .map_err(|msg| Error::new(ErrorKind::InitializationFailure, format!("Program linking error: {}", msg)))
    }

    pub(super) fn color_to_rgba(&self, color: Color) -> [f32; 4] {
        match color {
            Color::Black => [0.0, 0.0, 0.0, 1.0],
            Color::DarkBlue => [0.0, 0.0, 127.5, 1.0],
            Color::DarkGreen => [0.0, 127.5, 0.0, 1.0],
            Color::Teal => [0.0, 127.5, 127.5, 1.0],
            Color::DarkRed => [127.5, 0.0, 0.0, 1.0],
            Color::Magenta => [127.5, 0.0, 127.5, 1.0],
            Color::Olive => [127.5, 127.5, 0.0, 1.0],
            Color::Silver => [191.25, 191.25, 191.25, 1.0],
            Color::Gray => [127.5, 127.5, 127.5, 1.0],
            Color::Blue => [0.0, 0.0, 255.0, 1.0],
            Color::Green => [0.0, 255.0, 0.0, 1.0],
            Color::Aqua => [0.0, 255.0, 255.0, 1.0],
            Color::Red => [255.0, 0.0, 0.0, 1.0],
            Color::Pink => [255.0, 0.0, 255.0, 1.0],
            Color::Yellow => [255.0, 255.0, 0.0, 1.0],
            Color::White => [255.0, 255.0, 255.0, 1.0],
            Color::Transparent => [0.0, 0.0, 0.0, 0.0],
            Color::RGB(r, g, b) => {
                let r = r as f32 / 255.0;
                let g = g as f32 / 255.0;
                let b = b as f32 / 255.0;
                [r, g, b, 1.0]
            }
        }
    }
}

pub(super) fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Unable to create shader object".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error compiling shader".to_string()))
    }
}

pub(super) fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| "Unable to create program".to_string())?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error linking program".to_string()))
    }
}
