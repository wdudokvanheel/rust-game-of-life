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
    pub cells: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[false; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
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
            let x = x + neighbour[0];
            let y = y + neighbour[1];

            if x < 0 || x >= BOARD_SIZE as i32 || y < 0 || y >= BOARD_SIZE as i32 {
                continue;
            }

            if self.is_cell_occupied(x as usize, y as usize) {
                active += 1;
            }
        }

        return active;
    }
}
