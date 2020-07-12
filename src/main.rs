//! このファイルがプログラムの入り口とか、スタート地点みたいなもんだぜ☆（＾～＾）プログラムのエントリー・ポイントと言う☆（＾～＾）

extern crate chrono;
extern crate lazy_static;
extern crate regex;

mod command_line_parser;
mod computer_player;
mod look_and_model;
mod performance_measurement;
mod position;
mod test;
mod uxi_protocol;
mod win_lose_judgment;

use casual_logger::{Level, Log, LOGGER};
use command_line_parser::CommandLineParser;
use look_and_model::{GameResult, Piece, Position, Search};
use std;
use std::{thread, time};
use test::test_win_lose_judgement;

pub trait LogExt {
    fn println(s: &str);
}
impl LogExt for Log {
    /// 標準出力に文字列を出力するとともに、ログ・ファイルにも Info レベルで記録するぜ☆（＾～＾）
    fn println(s: &str) {
        println!("{}", s);
        Log::infoln(s);
    }
}

fn main() {
    let remove_num = if let Ok(mut logger) = LOGGER.lock() {
        // Do not call 'Log::xxxxx()' in this.
        //
        // Set file name.
        //
        // All: 'tic-tac-toe-2020-07-11.log.toml'
        // Prefix: 'tic-tac-toe'
        // StartDate: '-2020-07-11' automatically.
        // Suffix: '.log' - To be safe, include a word that clearly states that you can delete the file.
        // Extention: '.toml'
        //
        // If you don't like the .toml extension, leave the suffix empty and the .log extension.
        logger.set_file_name("tic-tac-toe", ".log", ".toml");

        logger.retention_days = 2;
        // The higher this level, the more will be omitted.
        //
        // |<-- Low Level ------------------------- High level -->|
        // |<-- High priority ------------------- Low priority -->|
        // | Fatal < Error < Warn < Notice < Info < Debug < Trace |
        logger.level = Level::Trace;
        // Remove old log files. This is determined by the StartDate in the filename.
        logger.remove_old_logs()
    } else {
        0
    };
    Log::noticeln(&format!("Remove {} files.", remove_num));

    // しょっぱなにプログラムが壊れてないかテストしているぜ☆（＾～＾）
    // こんなとこに書かない方がいいが、テストを毎回するのが めんどくさいんで 実行するたびにテストさせているぜ☆（＾～＾）
    // Step 1.
    Log::noticeln("Hello, world!!");
    Log::println("こんにちわ、世界！！");
    // こんにちわ、世界！！

    // Step 2.
    Log::println(&format!("Nought=|{}|", Piece::Nought));
    // Nought=|O|
    Log::println(&format!("Cross =|{}|", Piece::Cross));
    // Cross =|X|
    Log::println(&format!("Win   =|{}|", GameResult::Win));
    // Win   =|win|
    Log::println(&format!("Draw  =|{}|", GameResult::Draw));
    // Draw  =|draw|
    Log::println(&format!("Lose  =|{}|", GameResult::Lose));
    // Lose  =|lose|

    let mut pos = Position::default();
    Log::println(&pos.pos());
    // [Next 1 move(s) | Go O]
    //
    // +---+---+---+
    // |   |   |   | マスを選んでください。例 `do 7`
    // +---+---+---+
    // |   |   |   |    7 8 9
    // +---+---+---+    4 5 6
    // |   |   |   |    1 2 3
    // +---+---+---+
    // ぜったい None が返ってこない仕様のときは .unwrap() でヌル・チェックを飛ばせだぜ☆（＾～＾）
    Log::println(&Position::result(GameResult::Win, Some(Piece::Nought)).unwrap());
    // win O

    let search = Search::new(pos.friend, pos.pieces_num, true);
    Log::println(&format!("pv=|{}|", search.pv(&pos)));
    // pv=||
    Log::println(&Search::info_header(&pos));
    // info nps ...... nodes ...... pv O X O X O X O X O
    // 適当な内容を入れて、入れ物として、入れた中身を見せてくれるか、チェックしろだぜ☆（＾～＾）
    Log::println(&search.info_forward(123, &pos, 1, Some("Hello!")));
    // info nps    123 nodes      0 pv                   | + [1] | ->   to height 1 |       |      | + "Hello!"
    Log::println(&search.info_forward_leaf(456, &pos, 1, GameResult::Win, Some("Hello!")));
    // info nps    456 nodes      0 pv                   | + [1] | .       height 0 |       | win  | + "Hello!"
    Log::println(&search.info_backward(789, &pos, 1, GameResult::Win, Some("Hello!")));
    // info nps    789 nodes      0 pv                   |       | <- from height 1 | + [1] | win  | + "Hello!"

    // Step 3.
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

    // Step 4.
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

    // Step 5.
    Log::println(&format!("xfen=|{}|", pos.to_xfen()));
    // xfen=|xfen 3/3/3 o|
    pos.do_("2");
    Log::println(&pos.pos());
    // [Next 2 move(s) | Go x]
    //
    // +---+---+---+
    // |   |   |   | マスを選んでください。例 `do 7`
    // +---+---+---+
    // |   |   |   |    7 8 9
    // +---+---+---+    4 5 6
    // |   | o |   |    1 2 3
    // +---+---+---+
    let xfen = "xfen xo1/xox/oxo o";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!("Invalid xfen=|{}|", xfen)
    };
    Log::println(&pos.pos());
    // [Next 9 move(s) | Go o]
    //
    // +---+---+---+
    // | x | o |   | マスを選んでください。例 `do 7`
    // +---+---+---+
    // | x | o | x |    7 8 9
    // +---+---+---+    4 5 6
    // | o | x | o |    1 2 3
    // +---+---+---+
    let xfen = "xfen 3/3/3 x moves 1 7 4 8 9 3 6 2 5";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!("Invalid xfen=|{}|", xfen)
    };
    Log::println(&pos.pos());
    // win x
    // [Next 10 move(s) | Go o]
    //
    // +---+---+---+
    // | o | o | x | マスを選んでください。例 `do 7`
    // +---+---+---+
    // | x | x | x |    7 8 9
    // +---+---+---+    4 5 6
    // | x | o | o |    1 2 3
    // +---+---+---+
    pos.undo();
    Log::println(&pos.pos());
    // [Next 9 move(s) | Go x]
    //
    // +---+---+---+
    // | o | o | x | マスを選んでください。例 `do 7`
    // +---+---+---+
    // | x |   | x |    7 8 9
    // +---+---+---+    4 5 6
    // | x | o | o |    1 2 3
    // +---+---+---+

    // Step 6.
    // Step 7.
    let xfen = "xfen o2/xox/oxo x";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!("Invalid xfen=|{}|", xfen)
    };
    Log::println(&format!("win=|{}|", pos.is_opponent_win()));
    // win=|True|
    let xfen = "xfen xox/oxo/oxo x";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!("Invalid xfen=|{}|", xfen)
    };
    Log::println(&format!("draw=|{}|", pos.is_draw()));
    // draw=|True|

    // Step 8.
    // 探索してないんだから、 nodes も nps も 0 になるはずだよな☆（＾～＾）
    thread::sleep(time::Duration::from_secs(1));
    Log::println(&format!("nodes={}", search.nodes));
    // nodes=0
    Log::println(&format!("sec  ={}", search.sec()));
    // sec  =1
    Log::println(&format!("nps  ={}", search.nps()));
    // nps  =0

    // Step 9.
    let xfen = "xfen 3/3/3 o moves 1 5 2 3 7 4";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!("Invalid xfen=|{}|", xfen)
    };
    let mut search = Search::new(pos.friend, pos.pieces_num, true);
    let (addr, result) = search.go(&mut pos);
    // info nps ...... nodes ...... pv O X O X O X O X O
    // info nps      1 nodes      1 pv 6                 | - [6] | ->   to height 8 |       |      | - "Search."
    // info nps      2 nodes      2 pv 6 8               | + [8] | ->   to height 9 |       |      | + "Search."
    // info nps      3 nodes      3 pv 6 8 9             | - [9] | .       height 9 |       | draw | - "It's ok."
    // info nps      3 nodes      3 pv 6 8               |       | <- from height 8 | + [9] | draw |
    // info nps      3 nodes      3 pv 6                 |       | <- from height 7 | - [8] | draw | - "Fmmm."
    // info nps      4 nodes      4 pv 6 9               | + [9] | ->   to height 9 |       |      | + "Search."
    // info nps      5 nodes      5 pv 6 9 8             | - [8] | .       height 9 |       | draw | - "It's ok."
    // info nps      5 nodes      5 pv 6 9               |       | <- from height 8 | + [8] | draw |
    // info nps      5 nodes      5 pv 6                 |       | <- from height 7 | - [9] | draw | - "Fmmm."
    // info nps      5 nodes      5 pv                   |       | <- from height 6 | + [6] | draw | + "Fmmm."
    // info nps      6 nodes      6 pv 8                 | - [8] | ->   to height 8 |       |      | - "Search."
    // info nps      7 nodes      7 pv 8 6               | + [6] | .       height 8 |       | win  | + "Hooray!"
    // info nps      7 nodes      7 pv 8                 |       | <- from height 7 | - [6] | win  |
    // info nps      7 nodes      7 pv                   |       | <- from height 6 | + [8] | lose | + "Resign."
    // info nps      8 nodes      8 pv 9                 | - [9] | ->   to height 8 |       |      | - "Search."
    // info nps      9 nodes      9 pv 9 6               | + [6] | .       height 8 |       | win  | + "Hooray!"
    // info nps      9 nodes      9 pv 9                 |       | <- from height 7 | - [6] | win  |
    // info nps      9 nodes      9 pv                   |       | <- from height 6 | + [9] | lose | + "Resign."
    Log::println(&format!("result=|{}|", result));
    // result=|draw|
    Log::println(&format!(
        "bestmove=|{}|",
        if let Some(addr) = addr {
            format!("{}", addr).to_string()
        } else {
            "resign".to_string()
        }
    ));
    // bestmove=|6|

    // End.
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
            // コンピューター将棋の USIプロトコル で 'info string' というのがあって
            // 最後に画面に出せるかもしれないから頭に付けている☆（＾～＾）
            Err(e) => panic!(format!(
                "info string (Err.32)  Failed to read line. / {}",
                e
            )),
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
        } else if p.starts_with("quit") {
            break;
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

    // Wait for logging to complete. Time out 30 seconds.
    Log::wait_for_logging_to_complete(30, |elapsed_secs, rest_threads| {
        println!(
            "{} second(s). Wait for {} thread(s).",
            elapsed_secs, rest_threads
        );
    });
}
