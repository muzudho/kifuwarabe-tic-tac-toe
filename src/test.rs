//! Let's proceed with development while testing.  
//! テストしながら開発を進めましょう。  

use crate::command_line_seek::CommandLineSeek;
use crate::log::LogExt;
use crate::look_and_model::{GameResult, Piece, Position, Search, SearchDirection, SearchInfo};
use casual_logger::Log;
use std;
use std::{thread, time};

/// It is a unit test. I am writing it here because it is a hassle.
/// Check it against the explanation in README.md.
/// 単体テストです。めんどくさいのでここに書いています。
/// README.mdの解説と照らし合わせてみてください。
pub fn test() {
    // Step 1.
    Log::debugln("Hello, world!!");
    Log::print_debug("こんにちわ、世界！！");
    // こんにちわ、世界！！

    // Step 2 is this.

    // Step 3.
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
    /*
    [Next 1 move(s) | Go O]

    +---+---+---+
    |   |   |   | マスを選んでください。例 `do 7`
    +---+---+---+
    |   |   |   |    7 8 9
    +---+---+---+    4 5 6
    |   |   |   |    1 2 3
    +---+---+---+
    */
    // If not None is returned, .unwrap() skips the None check.
    // ぜったい None が返ってこないときは .unwrap() で None チェックを飛ばします。
    Log::print_debug(&Position::result(GameResult::Win, Some(Piece::Nought)).unwrap());
    // win O

    let search = Search::new(pos.pieces_num);
    Log::print_debug(&format!("pv=|{}|", pos.pv));
    // pv=||
    Log::print_debug(&Search::info_header(&pos));
    // info nps ...... nodes ...... pv O X O X O X O X O
    // 適当な内容を入れて、入れ物として、入れた中身を見せてくれるか、チェックしてください。
    let mut search_info = SearchInfo::new();
    search_info.nodes = search.nodes;
    search_info.pv = pos.pv.to_string();
    search_info.search_direction = SearchDirection::Forward;
    search_info.chosen_sq = 1;
    search_info.leaf = false;
    search_info.pieces_num = None;
    search_info.result = None;
    search_info.turn = Piece::Nought;
    search_info.comment = Some("Hello!".to_string());
    Log::print_debug(&Search::info_str(123, &search_info));
    // info nps    123 nodes      0 pv                   | + [1] | ->   to height 1 |       |      | + "Hello!"
    search_info = SearchInfo::new();
    search_info.nodes = search.nodes;
    search_info.pv = pos.pv.to_string();
    search_info.search_direction = SearchDirection::Forward;
    search_info.chosen_sq = 1;
    search_info.leaf = true;
    search_info.pieces_num = None;
    search_info.result = Some(GameResult::Win);
    search_info.turn = Piece::Cross;
    search_info.comment = Some("Hello!".to_string());
    Log::print_debug(&Search::info_str(456, &search_info));
    // info nps    456 nodes      0 pv                   | + [1] | .       height 0 |       | win  | + "Hello!"
    search_info = SearchInfo::new();
    search_info.nodes = search.nodes;
    search_info.pv = pos.pv.to_string();
    search_info.search_direction = SearchDirection::Backward;
    search_info.chosen_sq = 1;
    search_info.leaf = false;
    search_info.pieces_num = Some(pos.pieces_num);
    search_info.result = Some(GameResult::Win);
    search_info.turn = Piece::Nought;
    search_info.comment = Some("Hello!".to_string());
    Log::print_debug(&Search::info_str(789, &search_info));
    // info nps    789 nodes      0 pv                   |       | <- from height 1 | + [1] | win  | + "Hello!"

    // Step 4.
    pos.do_move(1);
    Log::print_debug(&pos.pos());
    /*
    [Next 2 move(s) | Go x]

    +---+---+---+ Please select a square. Example `do 7`
    |   |   |   | マスを選んでください。例 `do 7`
    +---+---+---+
    |   |   |   |    7 8 9
    +---+---+---+    4 5 6
    | o |   |   |    1 2 3
    +---+---+---+
    */
    pos.undo_move();
    Log::print_debug(&pos.pos());
    /*
    [Next 1 move(s) | Go o]

    +---+---+---+ Please select a square. Example `do 7`
    |   |   |   | マスを選んでください。例 `do 7`
    +---+---+---+
    |   |   |   |    7 8 9
    +---+---+---+    4 5 6
    |   |   |   |    1 2 3
    +---+---+---+
    */
    Log::print_debug(&format!("opponent=|{}|", pos.opponent()));
    // opponent=|X|

    // Step 5.
    let mut p = CommandLineSeek::new("Go to the Moon!");
    Log::print_debug(&format!("Go to   =|{}|", p.starts_with("Go to")));
    // Go to   =|True|
    Log::print_debug(&format!("Goto    =|{}|", p.starts_with("Goto")));
    // Goto    =|False|
    Log::print_debug(&format!("p.starts=|{}|", p.current()));
    // p.starts=|0|
    Log::print_debug(&format!(
        "p.rest  =|{}|",
        if let Some(rest) = p.rest() { rest } else { "" }
    ));
    // p.rest  =|Go to the Moon!|
    p.go_next_to("Go to");
    Log::print_debug(&format!("p.starts=|{}|", p.current()));
    // p.starts=|5|
    Log::print_debug(&format!(
        "p.rest  =|{}|",
        if let Some(rest) = p.rest() { rest } else { "" }
    ));
    // p.rest  =| the Moon!|

    // Step 6.
    Log::print_debug(&format!("xfen=|{}|", pos.to_xfen()));
    // xfen=|xfen 3/3/3 o|
    pos.do_("2");
    Log::print_debug(&pos.pos());
    /*
    [Next 2 move(s) | Go x]

    +---+---+---+ Please select a square. Example `do 7`
    |   |   |   | マスを選んでください。例 `do 7`
    +---+---+---+
    |   |   |   |    7 8 9
    +---+---+---+    4 5 6
    |   | o |   |    1 2 3
    +---+---+---+
    */
    let xfen = "xfen xo1/xox/oxo o";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
    };
    Log::print_debug(&pos.pos());
    /*
    [Next 9 move(s) | Go o]

    +---+---+---+ Please select a square. Example `do 7`
    | x | o |   | マスを選んでください。例 `do 7`
    +---+---+---+
    | x | o | x |    7 8 9
    +---+---+---+    4 5 6
    | o | x | o |    1 2 3
    +---+---+---+
    */
    let xfen = "xfen 3/3/3 x moves 1 7 4 8 9 3 6 2 5";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
    };
    Log::print_debug(&pos.pos());
    /*
    win x
    [Next 10 move(s) | Go o]

    +---+---+---+ Please select a square. Example `do 7`
    | o | o | x | マスを選んでください。例 `do 7`
    +---+---+---+
    | x | x | x |    7 8 9
    +---+---+---+    4 5 6
    | x | o | o |    1 2 3
    +---+---+---+
    */
    pos.undo();
    Log::print_debug(&pos.pos());
    /*
    [Next 9 move(s) | Go x]

    +---+---+---+ Please select a square. Example `do 7`
    | o | o | x | マスを選んでください。例 `do 7`
    +---+---+---+
    | x |   | x |    7 8 9
    +---+---+---+    4 5 6
    | x | o | o |    1 2 3
    +---+---+---+
    */

    // Step 7.
    // Step 8.
    // Step 9.
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

    // Step 10.
    // Since we have not searched, both nodes and nps will be 0.
    // 探索してないので、 nodes も nps も 0 になります。
    thread::sleep(time::Duration::from_secs(1));
    Log::print_debug(&format!("nodes={}", search.nodes));
    // nodes=0
    Log::print_debug(&format!("sec  ={}", search.sec()));
    // sec  =1
    Log::print_debug(&format!("nps  ={}", search.nps()));
    // nps  =0

    // Step 11.
    let xfen = "xfen 3/3/3 o moves 1 5 2 3 7 4";
    pos = if let Some(pos) = Position::from_xfen(xfen) {
        pos
    } else {
        panic!(Log::print_fatal(&format!("Invalid xfen=|{}|", xfen)))
    };
    let mut search = Search::new(pos.pieces_num);
    let (sq, result) = search.go(&mut pos);
    /*
    info string "nps":......, "nodes":......, "pv":[O,X,O,X,O,X,O,X,O]
    info json { "nps":     1, "nodes":     1, "pv":[6                ], "push":"6",               "pieces":7,                  "turn":"X", "comment":"Search." }
    info json { "nps":     2, "nodes":     2, "pv":[6,8              ], "push":"8",               "pieces":8,                  "turn":"O", "comment":"Search." }
    info json { "nps":     3, "nodes":     3, "pv":[6,8,9            ], "push":"9", "leaf": true, "pieces":9, "result":"draw", "turn":"X", "comment":"It is ok." }
    info json { "nps":     3, "nodes":     3, "pv":[6,8              ], "pop" :"9",               "pieces":8, "result":"draw", "turn":"O" }
    info json { "nps":     3, "nodes":     3, "pv":[6                ], "pop" :"8",               "pieces":7, "result":"draw", "turn":"X", "comment":"Fmmm." }
    info json { "nps":     4, "nodes":     4, "pv":[6,9              ], "push":"9",               "pieces":8,                  "turn":"O", "comment":"Search." }
    info json { "nps":     5, "nodes":     5, "pv":[6,9,8            ], "push":"8", "leaf": true, "pieces":9, "result":"draw", "turn":"X", "comment":"It is ok." }
    info json { "nps":     5, "nodes":     5, "pv":[6,9              ], "pop" :"8",               "pieces":8, "result":"draw", "turn":"O" }
    info json { "nps":     5, "nodes":     5, "pv":[6                ], "pop" :"9",               "pieces":7, "result":"draw", "turn":"X", "comment":"Fmmm." }
    info json { "nps":     5, "nodes":     5, "pv":[                 ], "pop" :"6",               "pieces":6, "result":"draw", "turn":"O", "comment":"Fmmm." }
    info json { "nps":     6, "nodes":     6, "pv":[8                ], "push":"8",               "pieces":7,                  "turn":"X", "comment":"Search." }
    info json { "nps":     7, "nodes":     7, "pv":[8,6              ], "push":"6", "leaf": true, "pieces":8, "result":"win" , "turn":"O", "comment":"Resign." }
    info json { "nps":     7, "nodes":     7, "pv":[8                ], "pop" :"6",               "pieces":7, "result":"win" , "turn":"X" }
    info json { "nps":     7, "nodes":     7, "pv":[                 ], "pop" :"8",               "pieces":6, "result":"lose", "turn":"O", "comment":"Damn!" }
    info json { "nps":     8, "nodes":     8, "pv":[9                ], "push":"9",               "pieces":7,                  "turn":"X", "comment":"Search." }
    info json { "nps":     9, "nodes":     9, "pv":[9,6              ], "push":"6", "leaf": true, "pieces":8, "result":"win" , "turn":"O", "comment":"Resign." }
    info json { "nps":     9, "nodes":     9, "pv":[9                ], "pop" :"6",               "pieces":7, "result":"win" , "turn":"X" }
    info json { "nps":     9, "nodes":     9, "pv":[                 ], "pop" :"9",               "pieces":6, "result":"lose", "turn":"O", "comment":"Damn!" }
    */
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

    // Wait for logging to complete.
    // ロギングが完了するまで待ちます。
    Log::flush();
}

/// Miscellaneous tests.  
/// 雑多なテストです。  
fn test_win_lose_judgement() {
    // O Win situation check.
    // O 勝ち局面チェック。
    debug_assert!(Position::from_xfen("xfen ooo/3/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/ooo/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/3/ooo x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen o2/o2/o2 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 1o1/1o1/1o1 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2o/2o/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen o2/1o1/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2o/1o1/o2 x")
        .unwrap()
        .is_opponent_win());
    // O Phase check that has not won.
    // O 勝ってない局面チェック。
    debug_assert!(!Position::from_xfen("xfen xoo/3/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/xoo/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/3/xoo x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen x2/o2/o2 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 1x1/1o1/1o1 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2x/2o/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen x2/1o1/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2x/1o1/o2 x")
        .unwrap()
        .is_opponent_win());
    // X Win situation check.
    // X 勝ち局面チェック。
    debug_assert!(Position::from_xfen("xfen xxx/3/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/xxx/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/3/xxx o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen x2/x2/x2 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 1x1/1x1/1x1 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2x/2x/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen x2/1x1/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2x/1x1/x2 o")
        .unwrap()
        .is_opponent_win());
    // X Phase check that has not won.
    // X 勝ってない局面チェック。
    debug_assert!(!Position::from_xfen("xfen oxx/3/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/oxx/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/3/oxx o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen o2/x2/x2 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 1o1/1x1/1x1 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2o/2x/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen o2/1x1/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2o/1x1/x2 o")
        .unwrap()
        .is_opponent_win());
    // Draw check.
    // 引き分けチェック。
    {
        debug_assert!(Position::from_xfen("xfen xox/xox/oxo x").unwrap().is_draw());
        debug_assert!(!Position::from_xfen("xfen xox/xox/oxo x")
            .unwrap()
            .is_opponent_win());
    }
}
