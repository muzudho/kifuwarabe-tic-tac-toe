mod piece;
mod position;
mod protocol;
mod view;

use position::Position;

fn main() {
    // let board = Board::default();

    // TODO テスト
    let xfen = "xfen xox/3/o1o x";
    // let xfen = "xfen x2/3/3 x";
    // let xfen = "xfen 1x1/3/3 x";
    // let xfen = "xfen 2x/3/3 x";
    // let xfen = "xfen 3/x2/3 x";
    // let xfen = "xfen 3/1x1/3 x";
    let board = if let Some(board) = Position::from_xfen(xfen) {
        board
    } else {
        panic!("xfen error. xfen={}", xfen);
    };

    print!("xfen={}", xfen);
    board.debug_write();
    board.println();
}
