use std::thread;
use std::time::Duration;

use board::Board;

mod board;

const SLEEP_TIME: Duration = Duration::from_millis(500);

fn main() {
    let mut board = Board::new();
    board.set_cell(3, 3, true);
    board.set_cell(1, 1, true);
    board.set_cell(4, 3, true);
    board.set_cell(5, 3, true);
    board.set_cell(6, 3, true);
    board.set_cell(5, 5, true);
    board.set_cell(5, 6, true);

    loop {
        let _ = clearscreen::clear();
        board = perform_generation(&mut board);
        print_board(&board);
        thread::sleep(SLEEP_TIME);
    }
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

    if neighbours >= 3 {
        return true;
    }
    return false;
}

fn print_board(board: &Board) {
    println!();
    for row in board.cells.iter() {
        for cell in row.iter() {
            print!(" {}", fmt_board_piece(cell))
        }
        println!()
    }
}

fn fmt_board_piece(value: &bool) -> char {
    match value {
        true => 'O',
        false => '.'
    }
}

