#[macro_use]
extern crate glium;

use std::time::Instant;

use glium::{Display, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;
use glium::texture::RawImage2d;
use glium::uniforms::{EmptyUniforms, MagnifySamplerFilter, MinifySamplerFilter, Sampler, UniformsStorage};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::Window;

use board::Board;
use shader::create_shader_program;

use crate::board::BOARD_SIZE;
use crate::direction::Direction::{East, South};
use crate::patterns::Pattern;
use crate::vertex::Vertex;

mod board;
mod vertex;
mod shader;
mod patterns;
mod direction;

const LOGIC_UPDATE_TIME: u128 = 100;

fn main() {
    let mut board = Board::new();
    let center = BOARD_SIZE / 2;

    let gun = Pattern::GliderGun;
    board.place_rotated_pattern(gun, center, center, East);

    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = create_window_display(&event_loop);
    let program = create_shader_program(&display);

    let vertex_buffer = create_rect_vbo(&display);
    let indices = NoIndices(TrianglesList);

    let width: u32 = BOARD_SIZE as u32;
    let height: u32 = BOARD_SIZE as u32;

    let data = create_data_from_board(&board);

    let image = RawImage2d::from_raw_rgb(data, (width, height));
    let texture = Texture2d::new(&display, image).unwrap();

    let mut last_update_time = Instant::now();
    let mut elapsed = 0u128;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },

            Event::RedrawEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                // Update time elapsed
                let now = Instant::now();
                elapsed += now.duration_since(last_update_time).as_millis();
                last_update_time = now;

                // Create uniform values for the shader
                let sampler = texture.sampled()
                    .magnify_filter(MagnifySamplerFilter::Nearest)
                    .minify_filter(MinifySamplerFilter::Nearest);

                let uniforms = uniform! {
                    screensize: [2048f32, 2048f32],
                    boardsize: [BOARD_SIZE as f32, BOARD_SIZE as f32],
                    tex: sampler,
                };

                while elapsed > LOGIC_UPDATE_TIME {
                    board = perform_generation(&mut board);
                    elapsed -= LOGIC_UPDATE_TIME;
                }
                update_texture(&texture, &board);
                draw_frame(&display, &program, &vertex_buffer, &indices, &uniforms);
            }
            _ => (),
        }
    });
}

fn perform_generation(board: &mut Board) -> Board {
    let mut new_board = Board::new();

    for (y, row) in board.cells.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            new_board.set_cell(x, y, update_cell(board, x, y));
        }
    }
    return new_board;
}

fn update_cell(board: &Board, x: usize, y: usize) -> bool {
    let active = board.is_cell_occupied(x, y);
    let neighbours = board.get_active_neighbours(x as i32, y as i32);

    if active {
        if neighbours != 2 && neighbours != 3 {
            return false;
        }
        return true;
    }

    if neighbours == 3 {
        return true;
    }
    return false;
}

fn create_data_from_board(board: &Board) -> Vec<u8> {
    let mut data = Vec::new();
    for row in board.cells.iter() {
        for cell in row.iter() {
            if *cell {
                data.extend_from_slice(&[255u8, 0, 0]);
            } else {
                data.extend_from_slice(&[0u8, 0, 0]);
            }
        }
    }

    return data;
}

fn update_texture(texture: &Texture2d, board: &Board) {
    let width: u32 = BOARD_SIZE as u32;
    let height: u32 = BOARD_SIZE as u32;

    texture.main_level().write(glium::Rect {
        left: 0,
        bottom: 0,
        width,
        height,
    }, RawImage2d::from_raw_rgb(create_data_from_board(board), (width, height)));
}

fn draw_frame(display: &Display<WindowSurface>, program: &Program, vertex_buffer: &VertexBuffer<Vertex>,
              indices: &NoIndices, uniforms: &UniformsStorage<Sampler<Texture2d>,
        UniformsStorage<[f32; 2], UniformsStorage<[f32; 2], EmptyUniforms>>>) {
    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(vertex_buffer, indices, &program, uniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();
}

fn create_rect_vbo(display: &Display<WindowSurface>) -> VertexBuffer<Vertex> {
    VertexBuffer::new(display, &get_rect_vertices()).unwrap()
}

fn get_rect_vertices() -> Vec<Vertex> {
    vec![
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
    ]
}

fn create_window_display(event_loop: &EventLoop<()>) -> (Window, Display<WindowSurface>) {
    let window_builder = winit::window::WindowBuilder::new()
        .with_resizable(false);

    glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .with_title("Game of Life")
        .with_inner_size(2048, 2048)
        .build(&event_loop)
}
