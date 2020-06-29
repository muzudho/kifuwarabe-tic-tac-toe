mod command_line;
mod piece;
mod position;
mod protocol;
mod result;
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
`position xfen 3/3/3 o moves 5 1 2 8 4 6 3 7 9` - 初期局面と棋譜を入力。
`xfen` - 現局面のxfen文字列表示。
"
    );
    // let board = Board::default();

    // 初期局面
    let xfen = "xfen 3/3/3 o";
    // 一手詰め局面
    // let xfen = "xfen oxo/xox/3 o";
    // 二手詰め局面
    // let xfen = "xfen oxo/xo1/3 x";
    // ３手詰め局面
    // let xfen = "xfen oxo/x2/3 o";
    // 必死の２手局面
    // let xfen = "xfen 2o/1o1/xxo x";
    // 受けの１手局面
    // let xfen = "xfen 3/1ox/1o1 x";
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

        // よく使うコマンド順に並べた方が高速だが、先に見つけた方が選ばれるので後ろの方を漏らしやすいし、アルファベット順に並べた方が見やすいぜ☆（＾～＾）
        if p.starts_with("do") {
            p.go_next_to("do ");
            // println!("Debug   | rest=|{}|", p.rest());
            if let Some(rest) = p.rest() {
                pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::default();
            let (address, mate) = search.go(&mut pos);
            if let Some(addr_val) = address {
                if let Some(mate_val) = mate {
                    println!("info mate={}", mate_val);
                }
                println!("bestmove {}", addr_val);
            } else {
                println!("resign");
            }
        } else if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = position::Position::from_xfen(rest) {
                    pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            pos.pos();
        } else if p.starts_with("undo") {
            pos.undo();
        } else if p.starts_with("xfen") {
            println!("{}", pos.to_xfen());
        } else {
            println!("Debug   | Command not found. {:?}", p);
        }
    }
}
