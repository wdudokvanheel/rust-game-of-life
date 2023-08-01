use crate::direction::Direction;

pub enum Pattern {
    Glider,
    Beacon,
    Toad,
    GliderGun,
    Acorn
}

impl Pattern {
    pub fn grid(&self) -> Vec<Vec<u8>> {
        match *self {
            Pattern::Glider => vec![
                vec![0, 1, 0],
                vec![0, 0, 1],
                vec![1, 1, 1],
            ],
            Pattern::Beacon => vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
            ],
            Pattern::Toad => vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 1, 1, 0, 0],
            ],
            Pattern::GliderGun => vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            Pattern::Acorn => vec![
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 1, 1, 1],
            ],
        }
    }

    pub fn rotated_grid(&self, direction: Direction) -> Vec<Vec<u8>> {
        let grid = self.grid();
        let rows = grid.len();
        let cols = grid[0].len();
        let mut rotated = vec![];

        match direction {
            Direction::North => rotated = grid,
            Direction::East => {
                rotated = vec![vec![0; rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        rotated[j][rows - i - 1] = grid[i][j];
                    }
                }
            }
            Direction::South => {
                rotated = vec![vec![0; cols]; rows];
                for i in 0..rows {
                    for j in 0..cols {
                        rotated[rows - i - 1][cols - j - 1] = grid[i][j];
                    }
                }
            }
            Direction::West => {
                rotated = vec![vec![0; rows]; cols];
                for i in 0..rows {
                    for j in 0..cols {
                        rotated[cols - j - 1][i] = grid[i][j];
                    }
                }
            }
        }
        rotated
    }
}
