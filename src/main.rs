mod command_line;
mod piece;
mod position;
mod protocol;
mod view;

use crate::command_line::CommandLine;
use position::Position;
use std;

fn main() {
    println!(
        "〇×ゲーム
    
    コマンド:
    `do 7` - 手番のプレイヤーが、 7 番地に印を付けます。
    `pos` - 局面表示。"
    );
    // let board = Board::default();

    // TODO テスト
    let xfen = "xfen xox/3/o1o x";
    // let xfen = "xfen x2/3/3 x";
    // let xfen = "xfen 1x1/3/3 x";
    // let xfen = "xfen 2x/3/3 x";
    // let xfen = "xfen 3/x2/3 x";
    // let xfen = "xfen 3/1x1/3 x";
    let mut board = if let Some(board) = Position::from_xfen(xfen) {
        board
    } else {
        panic!("xfen error. xfen={}", xfen);
    };

    print!("xfen={}", xfen);
    board.debug_write();

    // [Ctrl]+[C] でループを終了
    loop {
        let mut line: String = String::new();
        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            // エラー番号は適当に近くの行番号でも振っとけだぜ☆（＾～＾）ちょっぴり散らばる☆（＾～＾）
            Err(e) => panic!(format!("(Err.32)  Failed to read line. / {}", e)),
        };

        // コマンドライン☆（＾～＾） p は parser の意味で使ってるぜ☆（＾～＾）
        let mut p = CommandLine::new(&line);

        if p.starts_with("pos") {
            board.pos();
        } else if p.starts_with("do") {
            p.go_next_to("do ");
            println!("Debug   | rest=|{}|", p.rest());
            board.do_(p.rest());
        } else {
            println!("Debug   | Command not found. {:?}", p);
        }
    }
}
