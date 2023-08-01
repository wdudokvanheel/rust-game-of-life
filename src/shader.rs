use glium::{Display, Program};
use glium::glutin::surface::WindowSurface;

pub fn create_shader_program(display: &Display<WindowSurface>) -> Program {
    let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;
    uniform sampler2D tex;
    uniform vec2 screensize;
    uniform vec2 boardsize;

    void main() {
        vec2 pos = gl_FragCoord.xy / screensize;
        pos.y = 1.0 - pos.y;
        if(texture(tex, pos).r > 0){
            color = vec4(0.4, 0.01, 0.24, 1);
        }
        else{
            color = vec4(0.96, 0.96, 0.96, 1);
        }
    }
    "#;

    let program = Program::from_source(display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();
    return program;
}
