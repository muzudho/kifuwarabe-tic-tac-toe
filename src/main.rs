//! This is the entry point to the program.  
//! プログラムへの入り口です。  

// Publish:
//
// (1) `cargo test`
// (2) `cargo run --release`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Version up on Cargo.toml.
// (6) `cargo doc --open`
// (7) Comit to Git-hub.
// (8) `cargo publish --dry-run`
// (9) `cargo publish`

extern crate chrono;
extern crate lazy_static;
extern crate regex;

mod command_line_parser;
mod computer_player;
mod log;
mod look_and_model;
mod performance_measurement;
mod position;
mod test;
mod uxi_protocol;
mod win_lose_judgment;

use casual_logger::{Level, Log};
use command_line_parser::CommandLineParser;
use log::LogExt;
use look_and_model::{GameResult, Piece, Position, Search};
use std;
use std::{thread, time};
use test::test_win_lose_judgement;

fn main() {
    // Log file name.
    // ログ ファイル名。
    Log::set_file_name("kifuwarabe-tic-tac-toe");
    // Log level.
    // ログ レベル。
    Log::set_level(Level::Info);
    // Log file retention days.
    // ログ ファイル保持日数。
    Log::set_retention_days(2);
    // Remove old log files. This is determined by the date in the filename.
    // 古いログファイルを削除します。これは、ファイル名の日付によって決定されます。
    Log::remove_old_logs();

    // It is a unit test. I am writing it here because it is a hassle.
    // Check it against the explanation in README.md.
    // 単体テストです。めんどくさいのでここに書いています。
    // README.mdの解説と照らし合わせてみてください。
    if Log::enabled(Level::Debug) {
        // Step 1.
        Log::debugln("Hello, world!!");
        Log::print_debug("こんにちわ、世界！！");
        // こんにちわ、世界！！

        // Step 2.
        Log::print_debug(&format!("Nought=|{}|", Piece::Nought));
        // Nought=|O|
        Log::print_debug(&format!("Cross =|{}|", Piece::Cross));
        // Cross =|X|
        Log::print_debug(&format!("Win   =|{}|", GameResult::Win));
        // Win   =|win|
        Log::print_debug(&format!("Draw  =|{}|", GameResult::Draw));
        // Draw  =|draw|
        Log::print_debug(&format!("Lose  =|{}|", GameResult::Lose));
        // Lose  =|lose|

        let mut pos = Position::default();
        Log::print_debug(&pos.pos());
        // [Next 1 move(s) | Go O]
        //
        // +---+---+---+
        // |   |   |   | マスを選んでください。例 `do 7`
        // +---+---+---+
        // |   |   |   |    7 8 9
        // +---+---+---+    4 5 6
        // |   |   |   |    1 2 3
        // +---+---+---+
        // If not None is returned, .unwrap() skips the None check.
        // ぜったい None が返ってこないときは .unwrap() で None チェックを飛ばします。
        Log::print_debug(&Position::result(GameResult::Win, Some(Piece::Nought)).unwrap());
        // win O

        let search = Search::new(pos.turn, pos.pieces_num);
        Log::print_debug(&format!("pv=|{}|", search.pv(&pos, ',')));
        // pv=||
        Log::print_debug(&Search::info_header(&pos));
        // info nps ...... nodes ...... pv O X O X O X O X O
        // 適当な内容を入れて、入れ物として、入れた中身を見せてくれるか、チェックしろだぜ☆（＾～＾）
        Log::print_debug(&search.info_forward(123, &pos, 1, Some("Hello!")));
        // info nps    123 nodes      0 pv                   | + [1] | ->   to height 1 |       |      | + "Hello!"
        Log::print_debug(&search.info_forward_leaf(456, &pos, 1, GameResult::Win, Some("Hello!")));
        // info nps    456 nodes      0 pv                   | + [1] | .       height 0 |       | win  | + "Hello!"
        Log::print_debug(&search.info_backward(789, &pos, 1, GameResult::Win, Some("Hello!")));
        // info nps    789 nodes      0 pv                   |       | <- from height 1 | + [1] | win  | + "Hello!"

        // Step 3.
        pos.do_move(1);
        Log::print_debug(&pos.pos());
        // [Next 2 move(s) | Go x]
        //
        //         +---+---+---+ Please select a square. Example `do 7`
        //         |   |   |   | マスを選んでください。例 `do 7`
        //         +---+---+---+
        //         |   |   |   |    7 8 9
        //         +---+---+---+    4 5 6
        //         | o |   |   |    1 2 3
        //         +---+---+---+
        pos.undo_move();
        Log::print_debug(&pos.pos());
        // [Next 1 move(s) | Go o]
        //
        //         +---+---+---+ Please select a square. Example `do 7`
        //         |   |   |   | マスを選んでください。例 `do 7`
        //         +---+---+---+
        //         |   |   |   |    7 8 9
        //         +---+---+---+    4 5 6
        //         |   |   |   |    1 2 3
        //         +---+---+---+
        Log::print_debug(&format!("opponent={}", pos.opponent()));

        // Step 4.
        let mut p = CommandLineParser::new("Go to the Moon!");
        Log::print_debug(&format!("Go to   =|{}|", p.starts_with("Go to")));
        // Go to   =|True|
        Log::print_debug(&format!("Goto    =|{}|", p.starts_with("Goto")));
        // Goto    =|False|
        Log::print_debug(&format!("p.starts=|{}|", p.starts));
        // p.starts=|0|
        Log::print_debug(&format!(
            "p.rest  =|{}|",
            if let Some(rest) = p.rest() { rest } else { "" }
        ));
        // p.rest  =|Go to the Moon!|
        p.go_next_to("Go to");
        Log::print_debug(&format!("p.starts=|{}|", p.starts));
        // p.starts=|5|
        Log::print_debug(&format!(
            "p.rest  =|{}|",
            if let Some(rest) = p.rest() { rest } else { "" }
        ));
        // p.rest  =| the Moon!|

        // Step 5.
        Log::print_debug(&format!("xfen=|{}|", pos.to_xfen()));
        // xfen=|xfen 3/3/3 o|
        pos.do_("2");
        Log::print_debug(&pos.pos());
        // [Next 2 move(s) | Go x]
        //
        // +---+---+---+ Please select a square. Example `do 7`
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
            panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
        };
        Log::print_debug(&pos.pos());
        // [Next 9 move(s) | Go o]
        //
        // +---+---+---+ Please select a square. Example `do 7`
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
            panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
        };
        Log::print_debug(&pos.pos());
        // win x
        // [Next 10 move(s) | Go o]
        //
        // +---+---+---+ Please select a square. Example `do 7`
        // | o | o | x | マスを選んでください。例 `do 7`
        // +---+---+---+
        // | x | x | x |    7 8 9
        // +---+---+---+    4 5 6
        // | x | o | o |    1 2 3
        // +---+---+---+
        pos.undo();
        Log::print_debug(&pos.pos());
        // [Next 9 move(s) | Go x]
        //
        // +---+---+---+ Please select a square. Example `do 7`
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
            panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
        };
        Log::print_debug(&format!("win=|{}|", pos.is_opponent_win()));
        // win=|True|
        let xfen = "xfen xox/oxo/oxo x";
        pos = if let Some(pos) = Position::from_xfen(xfen) {
            pos
        } else {
            panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
        };
        Log::print_debug(&format!("draw=|{}|", pos.is_draw()));
        // draw=|True|

        // Step 8.
        // Since we have not searched, both nodes and nps will be 0.
        // 探索してないので、 nodes も nps も 0 になります。
        thread::sleep(time::Duration::from_secs(1));
        Log::print_debug(&format!("nodes={}", search.nodes));
        // nodes=0
        Log::print_debug(&format!("sec  ={}", search.sec()));
        // sec  =1
        Log::print_debug(&format!("nps  ={}", search.nps()));
        // nps  =0

        // Step 9.
        let xfen = "xfen 3/3/3 o moves 1 5 2 3 7 4";
        pos = if let Some(pos) = Position::from_xfen(xfen) {
            pos
        } else {
            panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
        };
        let mut search = Search::new(pos.turn, pos.pieces_num);
        let (sq, result) = search.go(&mut pos);
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
        Log::print_debug(&format!("result=|{}|", result));
        // result=|draw|
        Log::print_debug(&format!(
            "bestmove=|{}|",
            if let Some(sq) = sq {
                format!("{}", sq).to_string()
            } else {
                "resign".to_string()
            }
        ));
        // bestmove=|6|

        // End.
        test_win_lose_judgement();
    }

    // 説明を出そうぜ☆（＾～＾）
    Log::print_notice(
        "Kifuwarabe's tic-tac-toe
きふわらべの〇×ゲーム

Command:
コマンド:
`do 7`      - Mark number 7.
              手番のプレイヤーが、 7 番地に印を付けます。
`go`        - The computer shows the next move.
              コンピューターが次の1手を示します。
`info-off`  - no info output.
              info出力なし。
`info-on`   - There is info output.(Default)
              info出力あり(既定)。
`pos`       - Position display.
              局面表示。
`position xfen 3/3/3 o moves 5 1 2 8 4 6 3 7 9`
            - Starting position and moves.
              初期局面と棋譜を入力。
`undo`      - 1 back.
              1手戻します。
`uxi`       - Returns 'uxiok tic-tac-toe {protocol-version}'. It is a version of the protocol, not software.
              'uxiok tic-tac-toe {protocol-version}' を返します。ソフトではなくプロトコルのバージョンです。
`xfen`      - The current xfen string display.
              現局面のxfen文字列表示。

Let's input from `pos`.
`pos` から入力してみましょう。
",
    );

    // Starting position.
    // 初期局面。
    let mut pos = Position::default();

    // End the loop with 'quit'. Forced termination with [Ctrl]+[C].
    // 'quit' でループを終了。 [Ctrl]+[C] で強制終了。
    loop {
        let mut line: String = String::new();
        // Wait for command line input from standard input.
        // 標準入力からのコマンドライン入力を待機します。
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            // Tips. You can separate error numbers by simply specifying the line number.
            // テクニック。 エラー番号は行番号を振っておくだけで少しはばらけます。
            Err(e) => panic!(Log::print_fatal(&format!(
                "(Err.373) Failed to read line. / {}",
                e
            ))),
        };

        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineParser::new(&line);

        // It is in alphabetical order because it is easy to find.
        // 探しやすいからアルファベット順です。
        if p.starts_with("do") {
            p.go_next_to("do ");
            if let Some(rest) = p.rest() {
                pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::new(pos.turn, pos.pieces_num);
            let (sq, result) = search.go(&mut pos);
            if let Some(sq) = sq {
                Log::print_info(&format!(
                    "info string result={:?} nps={}",
                    result,
                    search.nps()
                ));
                Log::print_notice(&format!("bestmove {}", sq));
            } else {
                Log::print_notice("resign");
            }
        } else if p.starts_with("info-off") {
            Log::set_level(Level::Notice);
        } else if p.starts_with("info-on") {
            Log::set_level(Level::Info);
        } else if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = look_and_model::Position::from_xfen(rest) {
                    pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            Log::print_notice(&pos.pos());
        } else if p.starts_with("quit") {
            break;
        } else if p.starts_with("undo") {
            pos.undo();
        } else if p.starts_with("uxi") {
            Log::print_notice("uxiok tic-tac-toe v20200718.0.0");
        } else if p.starts_with("xfen") {
            Log::print_notice(&format!("{}", pos.to_xfen()));
        } else {
            Log::print_debug(&format!("Debug   | Invalid command=|{:?}|", p));
        }
    }

    // Wait for logging to complete.
    // ロギングが完了するまで待ちます。
    Log::wait();
}
