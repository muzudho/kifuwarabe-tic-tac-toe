//! このファイルがプログラムの入り口とか、スタート地点みたいなもんだぜ☆（＾～＾）プログラムのエントリー・ポイントと言う☆（＾～＾）
#[macro_use]
extern crate lazy_static;

mod command_line_parser;
mod log;
mod position;
mod protocol;
mod search;
mod test;
mod view;

use command_line_parser::CommandLineParser;
use log::Log;
use position::Position;
use search::Search;
use std;
use test::test;

fn main() {
    // しょっぱなにプログラムが壊れてないかテストしているぜ☆（＾～＾）
    // こんなとこに書かない方がいいが、テストを毎回するのが めんどくさいんで 実行するたびにテストさせているぜ☆（＾～＾）
    test();
    // 説明を出そうぜ☆（＾～＾）
    Log::println(
        "きふわらべの〇×ゲーム

コマンド:
`do 7`     - 手番のプレイヤーが、 7 番地に印を付けます。
`go`       - コンピューターが次の1手を示します。
`info-off` - info出力なし。
`info-on`  - info出力あり(既定)。
`pos`      - 局面表示。
`position xfen 3/3/3 o moves 5 1 2 8 4 6 3 7 9` - 初期局面と棋譜を入力。
`undo`     - 1手戻します。
`xfen`     - 現局面のxfen文字列表示。
",
    );

    // 初期局面
    let mut pos = Position::default();
    let mut info_enable = true;

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
        let mut p = CommandLineParser::new(&line);

        // 本当は よく使うコマンド順に並べた方が高速だが、先に見つけた方が選ばれるので後ろの方を漏らしやすくて むずかしいし、
        // だから、アルファベット順に並べた方が見やすいぜ☆（＾～＾）
        if p.starts_with("do") {
            p.go_next_to("do ");
            if let Some(rest) = p.rest() {
                pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::new(pos.friend, pos.pieces_num, info_enable);
            let (addr, result) = search.go(&mut pos);
            if let Some(addr) = addr {
                Log::println(&format!("info result={:?}", result));
                Log::println(&format!("bestmove {}", addr));
            } else {
                Log::println("resign");
            }
        } else if p.starts_with("info-off") {
            info_enable = false;
        } else if p.starts_with("info-on") {
            info_enable = true;
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
            Log::println(&format!("{}", pos.to_xfen()));
        } else {
            Log::println(&format!("Debug   | Command not found. {:?}", p));
        }
    }
}
