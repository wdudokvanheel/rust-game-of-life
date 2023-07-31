use std::thread;
use std::time::Duration;

use board::Board;

mod board;

const SLEEP_TIME: Duration = Duration::from_millis(500);

fn main() {
    let mut board = Board::new();
    board.set_cell(32, 31, true);
    board.set_cell(33, 31, true);
    board.set_cell(31, 31, true);
    board.set_cell(33, 32, true);

    let mut generation = 1;

    let _ = clearscreen::clear();
    print_board(&board, generation);
    thread::sleep( Duration::from_millis(1000));

    loop {
        let _ = clearscreen::clear();
        board = perform_generation(&mut board);
        generation += 1;
        print_board(&board, generation);
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

    if neighbours == 3 {
        return true;
    }
    return false;
}

fn print_board(board: &Board, generation: u32) {
    println!();
    for row in board.cells.iter() {
        for cell in row.iter() {
            print!(" {}", fmt_board_piece(cell))
        }
        println!()
    }
    println!("Generation: {}", generation)
}

fn fmt_board_piece(value: &bool) -> char {
    match value {
        true => 'O',
        false => '.'
    }
}

