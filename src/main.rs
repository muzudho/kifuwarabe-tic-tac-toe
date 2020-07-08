//! このファイルがプログラムの入り口とか、スタート地点みたいなもんだぜ☆（＾～＾）プログラムのエントリー・ポイントと言う☆（＾～＾）

#[macro_use]
extern crate lazy_static;

mod command_line_parser;
mod computer_player;
mod log;
mod look_and_model;
mod performance_measurement;
mod position;
mod test;
mod uxi_protocol;
mod win_lose_judgment;

use command_line_parser::CommandLineParser;
use log::Log;
use look_and_model::{GameResult, Piece, Position, Search};
use std;
use test::test_win_lose_judgement;

fn main() {
    // しょっぱなにプログラムが壊れてないかテストしているぜ☆（＾～＾）
    // こんなとこに書かない方がいいが、テストを毎回するのが めんどくさいんで 実行するたびにテストさせているぜ☆（＾～＾）
    Log::writeln("狂った街、東京！！");
    Log::println("おはようさん、世界！！");

    Log::println(&format!("Nought=|{}|", Piece::Nought));
    Log::println(&format!("Cross =|{}|", Piece::Cross));
    Log::println(&format!("Win   =|{}|", GameResult::Win));
    Log::println(&format!("Draw  =|{}|", GameResult::Draw));
    Log::println(&format!("Lose  =|{}|", GameResult::Lose));

    let mut pos = Position::default();
    Log::println(&pos.pos());
    // ぜったい None が返ってこない仕様のときは .unwrap() でヌル・チェックを飛ばせだぜ☆（＾～＾）
    Log::println(&Position::result(GameResult::Win, Some(Piece::Nought)).unwrap());

    let search = Search::new(pos.friend, pos.pieces_num, true);
    Log::println(&format!("pv=|{}|", search.pv(&pos)));
    Log::println(&Search::info_header(&pos));
    // 適当な内容を入れて、入れ物として、入れた中身を見せてくれるか、チェックしろだぜ☆（＾～＾）
    Log::println(&search.info_forward(123, &pos, 1, Some("Hello!")));
    Log::println(&search.info_forward_leaf(456, &pos, 1, GameResult::Win, Some("Hello!")));
    Log::println(&search.info_backward(789, &pos, 1, GameResult::Win, Some("Hello!")));

    pos.do_move(1);
    Log::println(&pos.pos());
    // [Next 2 move(s) | Go x]
    //
    //         +---+---+---+
    //         |   |   |   | マスを選んでください。例 `do 7`
    //         +---+---+---+
    //         |   |   |   |    7 8 9
    //         +---+---+---+    4 5 6
    //         | o |   |   |    1 2 3
    //         +---+---+---+
    pos.undo_move();
    Log::println(&pos.pos());
    // [Next 1 move(s) | Go o]
    //
    //         +---+---+---+
    //         |   |   |   | マスを選んでください。例 `do 7`
    //         +---+---+---+
    //         |   |   |   |    7 8 9
    //         +---+---+---+    4 5 6
    //         |   |   |   |    1 2 3
    //         +---+---+---+
    Log::println(&format!("opponent={}", pos.opponent()));

    let mut p = CommandLineParser::new("Go to the Moon!");
    Log::println(&format!("Go to   =|{}|", p.starts_with("Go to")));
    // Go to   =|True|
    Log::println(&format!("Goto    =|{}|", p.starts_with("Goto")));
    // Goto    =|False|
    Log::println(&format!("p.starts=|{}|", p.starts));
    // p.starts=|0|
    Log::println(&format!(
        "p.rest  =|{}|",
        if let Some(rest) = p.rest() { rest } else { "" }
    ));
    // p.rest  =|Go to the Moon!|
    p.go_next_to("Go to");
    Log::println(&format!("p.starts=|{}|", p.starts));
    // p.starts=|5|
    Log::println(&format!(
        "p.rest  =|{}|",
        if let Some(rest) = p.rest() { rest } else { "" }
    ));
    // p.rest  =| the Moon!|

    test_win_lose_judgement();

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
`uxi`      - 'uxiok tic-tac-toe {protocol-version}' を返します。
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
                Log::println(&format!("info result={:?} nps={}", result, search.nps()));
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
                if let Some(pos_val) = look_and_model::Position::from_xfen(rest) {
                    pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            Log::println(&pos.pos());
        } else if p.starts_with("undo") {
            pos.undo();
        } else if p.starts_with("uxi") {
            Log::println("uxiok tic-tac-toe v20200704.0.0");
        } else if p.starts_with("xfen") {
            Log::println(&format!("{}", pos.to_xfen()));
        } else {
            Log::println(&format!("Debug   | Invalid command=|{:?}|", p));
        }
    }
}
