use crate::direction::Direction;
use crate::pattern::Pattern;

pub const BOARD_SIZE: usize = 128;

const NEIGHBOUR_CELLS: [[i32; 2]; 8] = [
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1]
];

pub struct Board {
    pub generation: i128,
    pub cells: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            generation: 1,
            cells: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn place_pattern(&mut self, pattern: Pattern, x: usize, y: usize) {
        let pattern_grid = pattern.grid();
        self.set_cells(pattern_grid, x, y);
    }

    pub fn place_rotated_pattern(&mut self, pattern: Pattern, x: usize, y: usize, direction: Direction) {
        let pattern_grid = pattern.rotated_grid(direction);
        self.set_cells(pattern_grid, x, y);
    }

    pub fn set_cells(&mut self, cells: Vec<Vec<u8>>, x: usize, y: usize) {
        let y = y - (cells.len() / 2);
        let x = x - (cells.get(0).unwrap().len() / 2);

        for (pattern_y, row) in cells.iter().enumerate() {
            for (pattern_x, &cell) in row.iter().enumerate() {
                let board_x = pattern_x + x;
                let board_y = pattern_y + y;

                if board_x < BOARD_SIZE && board_y < BOARD_SIZE {
                    self.cells[board_y][board_x] = cell == 1;
                }
            }
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return;
        }

        self.cells[y][x] = value;
    }

    pub fn is_cell_occupied(&self, x: usize, y: usize) -> bool {
        if y >= BOARD_SIZE || x >= BOARD_SIZE {
            panic!("Index out of bounds");
        }
        self.cells[y][x]
    }

    pub fn get_active_neighbours(&self, x: i32, y: i32) -> i8 {
        if y >= BOARD_SIZE as i32 || x >= BOARD_SIZE as i32 || y < 0 || x < 0 {
            panic!("Index out of bounds");
        }

        let mut active: i8 = 0;

        for neighbour in NEIGHBOUR_CELLS {
            let mut x = x + neighbour[0];
            let mut y = y + neighbour[1];

            // Wrap around the board
            if x >= BOARD_SIZE as i32 {
                x -= BOARD_SIZE as i32;
            }
            if x < 0 {
                x += BOARD_SIZE as i32;
            }

            if y >= BOARD_SIZE as i32 {
                y -= BOARD_SIZE as i32;
            }
            if y < 0 {
                y += BOARD_SIZE as i32;
            }

            if self.is_cell_occupied(x as usize, y as usize) {
                active += 1;
            }
        }

        return active;
    }
}
