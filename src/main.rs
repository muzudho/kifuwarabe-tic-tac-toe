mod command_line;
mod piece;
mod position;
mod protocol;
mod search;
mod test;
mod view;

use crate::command_line::CommandLine;
use position::Position;
use search::Search;
use std;
use test::test;

fn main() {
    test();
    println!(
        "〇×ゲーム
    
    コマンド:
    `do 7` - 手番のプレイヤーが、 7 番地に印を付けます。
    `pos` - 局面表示。
    `position xfen 3/3/3 o` - 初期局面に設定。"
    );
    // let board = Board::default();

    // 初期局面
    let xfen = "xfen 3/3/3 o";
    let mut pos = if let Some(board) = Position::from_xfen(xfen) {
        board
    } else {
        panic!("xfen error. xfen={}", xfen);
    };

    // println!("xfen={}", xfen);
    // board.debug_write();

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

        if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = position::Position::from_xfen(rest) {
                    pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            pos.pos();
        } else if p.starts_with("do") {
            p.go_next_to("do ");
            // println!("Debug   | rest=|{}|", p.rest());
            if let Some(rest) = p.rest() {
                pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::default();
            let (address, mate) = search.go(&mut pos);
            if let Some(addr_val) = address {
                println!("info mate={}", mate);
                println!("bestmove {}", addr_val);
            } else {
                println!("resign");
            }
        } else {
            println!("Debug   | Command not found. {:?}", p);
        }
    }
}
