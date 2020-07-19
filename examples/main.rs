use casual_logger::{Level, Log};
use kifuwarabe_tic_tac_toe::{
    command_line_seek::CommandLineSeek,
    log::LogExt,
    look_and_model::{Position, Search},
};
use std;

/// This is the entry point to the program.  
/// プログラムへの入り口です。  
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
        let mut p = CommandLineSeek::new(&line);

        // It is in alphabetical order because it is easy to find.
        // 探しやすいからアルファベット順です。
        if p.starts_with("do") {
            p.go_next_to("do ");
            if let Some(rest) = p.rest() {
                pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::new(pos.pieces_num);
            let (sq, result) = search.go(&mut pos);
            Log::print_info(&format!(
                "info string result={:?} nps={}",
                result,
                search.nps()
            ));

            Log::print_notice(&format!(
                "bestmove {}",
                if let Some(sq) = sq {
                    sq.to_string()
                } else {
                    "resign".to_string()
                }
            ));
        } else if p.starts_with("info-off") {
            Log::set_level(Level::Notice);
        } else if p.starts_with("info-on") {
            Log::set_level(Level::Info);
        } else if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = Position::from_xfen(rest) {
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
