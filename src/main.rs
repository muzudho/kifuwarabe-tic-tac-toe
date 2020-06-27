mod board;
mod piece;
mod view;

use board::Board;

fn main() {
    let board = Board::default();
    board.println();
}
