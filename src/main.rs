#![windows_subsystem = "windows"]
#[macro_use]
extern crate glium;

use std::time::Instant;

use glium::{Display, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::index::PrimitiveType::TrianglesList;
use glium::texture::RawImage2d;
use glium::uniforms::{EmptyUniforms, MagnifySamplerFilter, MinifySamplerFilter, Sampler, UniformsStorage};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Icon, Window, WindowButtons};

use board::Board;
use shader::create_shader_program;

use crate::board::BOARD_SIZE;
use crate::direction::Direction;
use crate::direction::Direction::East;
use crate::pattern::Pattern;
use crate::vertex::Vertex;

mod board;
mod vertex;
mod shader;
mod pattern;
mod direction;

const LOGIC_UPDATE_TIME: f32 = 1000f32;
const WINDOW_SIZE: i16 = 1024i16;

fn main() {
    let mut board = Board::new();
    let center = BOARD_SIZE / 2;

    board.place_rotated_pattern(Pattern::get_random_pattern(), center, center, East);

    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = create_window_display(&event_loop);
    let scale_factor = window.scale_factor();
    let window_size = (WINDOW_SIZE as f64 * scale_factor, WINDOW_SIZE as f64 * scale_factor);
    let program = create_shader_program(&display);

    // VBO to render a screen filling rectangle
    let vertex_buffer = create_rect_vbo(&display);
    let indices = NoIndices(TrianglesList);

    let data = create_data_from_board(&board);

    let image = RawImage2d::from_raw_rgb(data, (BOARD_SIZE as u32, BOARD_SIZE as u32));
    let texture = Texture2d::new(&display, image).unwrap();

    let mut last_update_time = Instant::now();
    let mut elapsed: i128 = LOGIC_UPDATE_TIME as i128;
    let mut running = false;
    let mut speed: f32 = 1f32;

    let mut mouse_dragging = false;
    let mut mouse_erase = false;
    let mut mouse_position = (0f64, 0f64);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                    ..
                } => {
                    match key {
                        VirtualKeyCode::Escape => {
                            *control_flow = ControlFlow::Exit;
                        }
                        VirtualKeyCode::Space => {
                            running = !running;
                        }
                        VirtualKeyCode::R => {
                            running = false;
                            board = Board::new();
                            board.place_rotated_pattern(Pattern::get_random_pattern(), center,
                                                        center, Direction::get_random_direction());
                        }
                        VirtualKeyCode::C => {
                            running = false;
                            board = Board::new();
                        }
                        VirtualKeyCode::D => {
                            if !running {
                                board = perform_generation(&mut board);
                            }
                        }
                        VirtualKeyCode::Key1 => {
                            speed = 1f32;
                        }
                        VirtualKeyCode::Key2 => {
                            speed = 1f32 / 2f32;
                        }
                        VirtualKeyCode::Key3 => {
                            speed = 1f32 / 4f32;
                        }
                        VirtualKeyCode::Key4 => {
                            speed = 1f32 / 8f32;
                        }
                        VirtualKeyCode::Key5 => {
                            speed = 1f32 / 16f32;
                        }
                        VirtualKeyCode::Key6 => {
                            speed = 1f32 / 32f32;
                        }
                        _ => (),
                    }
                }
                WindowEvent::MouseInput {
                    state, button, ..
                } => {
                    match (button, state) {
                        (MouseButton::Left, ElementState::Pressed) => {
                            mouse_dragging = true;
                            mouse_erase = false;
                            if !running {
                                set_cell_at_cursor(window_size, mouse_position, &mut board,
                                                   !mouse_erase);
                            }
                        }
                        (MouseButton::Right, ElementState::Pressed) => {
                            mouse_dragging = true;
                            mouse_erase = true;
                            if !running {
                                set_cell_at_cursor(window_size, mouse_position, &mut board,
                                                   !mouse_erase);
                            }
                        }
                        (MouseButton::Left | MouseButton::Right, ElementState::Released) => {
                            mouse_dragging = false;
                        }
                        _ => ()
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    mouse_position = (position.x, position.y);
                    if mouse_dragging {
                        if !running {
                            set_cell_at_cursor(window_size, mouse_position, &mut board,
                                               !mouse_erase);
                        }
                    }
                }
                _ => ()
            },

            Event::RedrawEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                // Update time elapsed
                let now = Instant::now();
                elapsed += now.duration_since(last_update_time).as_millis() as i128;
                last_update_time = now;

                // Create uniform values for the shader
                let sampler = texture.sampled()
                    .magnify_filter(MagnifySamplerFilter::Nearest)
                    .minify_filter(MinifySamplerFilter::Nearest);

                let uniforms = uniform! {
                    screensize: [window_size.0 as f32, window_size.1 as f32],
                    boardsize: [BOARD_SIZE as f32, BOARD_SIZE as f32],
                    tex: sampler,
                };

                let logic_time: i128 = (LOGIC_UPDATE_TIME * speed).round() as i128;

                while elapsed > logic_time {
                    if running {
                        board = perform_generation(&mut board);
                    }
                    elapsed -= logic_time;
                }
                update_texture(&texture, &board);
                draw_frame(&display, &program, &vertex_buffer, &indices, &uniforms);
                let title = format!("Game of Life :: Speed {} :: Generation {} :: Population {} {}",
                                    speed_to_string(speed), board.generation, board.population,
                                    sim_state_to_string(running));
                window.set_title(&title);
            }
            _ => (),
        }
    });
}

fn sim_state_to_string(running: bool) -> String {
    if running {
        return "".to_string();
    }
    return ":: Paused".to_string();
}

fn speed_to_string(speed: f32) -> String {
    if speed == 0f32 {
        return "0x".to_string();
    }
    let inverse = 1.0 / speed;
    format!("{:.0}x", inverse)
}

fn set_cell_at_cursor(
    window_size: (f64, f64),
    mouse_position: (f64, f64),
    board: &mut Board, draw: bool) {
    let (x, y) = mouse_position;

    if x < 0f64 || y < 0f64 || x > window_size.0 || y > window_size.1 {
        return;
    }

    let cell_size = window_size.0 / (BOARD_SIZE as f64);
    let x = (x.floor() / cell_size as f64).floor() as usize;
    let y = (y.floor() / cell_size as f64).floor() as usize;

    board.set_cell(x, y, draw);
}


fn perform_generation(board: &mut Board) -> Board {
    let mut new_board = Board::new();

    for (y, row) in board.cells.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            new_board.set_cell(x, y, update_cell(board, x, y));
        }
    }
    new_board.generation = board.generation + 1;
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
    let image = image::load(std::io::Cursor::new(&include_bytes!("../assets/icon.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let icon = Icon::from_rgba(image.as_raw().to_owned(), image.width(), image.height()).unwrap();

    let window_builder = winit::window::WindowBuilder::new()
        .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
        .with_inner_size(LogicalSize::new(1024, 1024))
        .with_window_icon(Some(icon))
        .with_resizable(false);


    glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .with_title("Game of Life")
        .build(&event_loop)
}
