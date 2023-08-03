use glium::{Display, Program};
use glium::glutin::surface::WindowSurface;
use glium::program::ProgramCreationInput;

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
        vec4 foreground = vec4(0.075,0.298,0.812, 1.0);
        vec4 background = vec4(0.114,0.114,0.114, 1);
        vec4 grid = background * 1.3;

        vec2 pos = gl_FragCoord.xy / screensize;
        pos.y = 1.0 - pos.y;
        if(texture(tex, pos).r > 0){
            color = foreground;
        }
        else{
            color = background;
        }

        vec2 cell_size = screensize / boardsize;

        if (cell_size.x > 1 && cell_size.y > 1){
            vec2 mod = mod(pos * screensize, cell_size);

            if (abs(mod.x) < 1 || abs(mod.y) < 1) {
                color = grid;
            }
        }
    }
    "#;

    return Program::new(display, ProgramCreationInput::SourceCode {
        vertex_shader: vertex_shader_src,
        fragment_shader: fragment_shader_src,
        geometry_shader: None,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        transform_feedback_varyings: None,
        outputs_srgb: true,
        uses_point_size: false,
    }).unwrap();
}
