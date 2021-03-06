# Kifuwarabe's tic-tac-toe

Programming computer shogi (Japanese chess) is difficult.  
コンピュータ将棋のプログラミングは難しいです。  

How can I improve my programming skills?  
プログラミングスキルを向上させるにはどうすればよいですか？  

A strategy of gradually stepping up from a simple game is not appropriate.  
単純なゲームから徐々にステップアップする戦略は適切ではありません。  

The reason is that the right program to solve the problem depends entirely on the game. Unless you have general artificial intelligence.  
その理由は、問題を解決するための適切なプログラムが完全にゲームに依存しているためです。汎用人工知能を持っていない限り。  

I recommend that you write computer shogi software directly, rather than bypassing it.  
迂回するのではなく、コンピューター将棋ソフトウェアを直接作成することをお勧めします。  

Of course it's best to start by modifying the strongest library. Helps to find out what is missing.  
もちろん、最強のライブラリを変更することから始めるのが最善です。 何が欠けているかを知るのに役立ちます。  

However, if you want to overcome the weaknesses of basic programming skills unrelated to computer shogi, it makes sense to start with a simple program.  
ただし、コンピュータ将棋とは関係のない基本的なプログラミングスキルの弱点を克服したい場合は、単純なプログラムから始めるのが理にかなっています。  

For example, a tic-tac-toe game can be fully analyzed on your home computer, which is useful for checking your skills.  
たとえば、三目並べゲームは自宅のコンピューターで完全に分析できるため、スキルをチェックするのに役立ちます。  

Computer shogi, computer go program Kifuwarabe has been converted into tic-tac-toe.  
コンピュータ将棋、コンピュータ囲碁プログラム「きふわらべ」を三目並べに変換しました。  

It was implemented with an unspecified UXI protocol that imitates the UCI / USI protocol. X has no meaning.　 
UCI / USIプロトコルを模した未指定のUXIプロトコルで実装しました。 X に意味はありません。  

Come see the repository.  
リポジトリをご覧ください。  

There is **no** GUI (Graphical user interface). Thought engine only.  
GUIは **ありません**。思考エンジンのみです。  

## Demonstration

Terminal:  

```shell
cargo run --example main
```

In game:  

```plain
Kifuwarabe's tic-tac-toe
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

pos
[Next 1 move(s) | Go O]

+---+---+---+ Please select a square. Example `do 7`
|   |   |   | マスを選んでください。例 `do 7`
+---+---+---+
|   |   |   |    7 8 9
+---+---+---+    4 5 6
|   |   |   |    1 2 3
+---+---+---+
xfen
xfen 3/3/3 o
position xfen oxx/1o1/2x o
pos
[Next 6 move(s) | Go O]

+---+---+---+ Please select a square. Example `do 7`
| O | X | X | マスを選んでください。例 `do 7`
+---+---+---+
|   | O |   |    7 8 9
+---+---+---+    4 5 6
|   |   | X |    1 2 3
+---+---+---+
do 6
pos
[Next 7 move(s) | Go X]

+---+---+---+ Please select a square. Example `do 7`
| O | X | X | マスを選んでください。例 `do 7`
+---+---+---+
|   | O | O |    7 8 9
+---+---+---+    4 5 6
|   |   | X |    1 2 3
+---+---+---+
go
info string "nps":......, "nodes":......, "pv":[X,O,X,O,X,O,X,O,X]
info json { "nps":     1, "nodes":     1, "pv":[1                ], "push":"1",               "pieces":7,                  "turn":"O", "comment":"Search." }
info json { "nps":     2, "nodes":     2, "pv":[1,2              ], "push":"2",               "pieces":8,                  "turn":"X", "comment":"Search." }
info json { "nps":     3, "nodes":     3, "pv":[1,2,4            ], "push":"4", "leaf": true, "pieces":9, "result":"draw", "turn":"O", "comment":"It is ok." }
info json { "nps":     3, "nodes":     3, "pv":[1,2              ], "pop" :"4",               "pieces":8, "result":"draw", "turn":"X" }
info json { "nps":     3, "nodes":     3, "pv":[1                ], "pop" :"2",               "pieces":7, "result":"draw", "turn":"O", "comment":"Fmmm." }
info json { "nps":     4, "nodes":     4, "pv":[1,4              ], "push":"4", "leaf": true, "pieces":8, "result":"win" , "turn":"X", "comment":"Resign." }
info json { "nps":     4, "nodes":     4, "pv":[1                ], "pop" :"4",               "pieces":7, "result":"win" , "turn":"O" }
info json { "nps":     4, "nodes":     4, "pv":[                 ], "pop" :"1",               "pieces":6, "result":"lose", "turn":"X", "comment":"Damn!" }
info json { "nps":     5, "nodes":     5, "pv":[2                ], "push":"2",               "pieces":7,                  "turn":"O", "comment":"Search." }
info json { "nps":     6, "nodes":     6, "pv":[2,1              ], "push":"1",               "pieces":8,                  "turn":"X", "comment":"Search." }
info json { "nps":     7, "nodes":     7, "pv":[2,1,4            ], "push":"4", "leaf": true, "pieces":9, "result":"draw", "turn":"O", "comment":"It is ok." }
info json { "nps":     7, "nodes":     7, "pv":[2,1              ], "pop" :"4",               "pieces":8, "result":"draw", "turn":"X" }
info json { "nps":     7, "nodes":     7, "pv":[2                ], "pop" :"1",               "pieces":7, "result":"draw", "turn":"O", "comment":"Fmmm." }
info json { "nps":     8, "nodes":     8, "pv":[2,4              ], "push":"4", "leaf": true, "pieces":8, "result":"win" , "turn":"X", "comment":"Resign." }
info json { "nps":     8, "nodes":     8, "pv":[2                ], "pop" :"4",               "pieces":7, "result":"win" , "turn":"O" }
info json { "nps":     8, "nodes":     8, "pv":[                 ], "pop" :"2",               "pieces":6, "result":"lose", "turn":"X", "comment":"Damn!" }
info json { "nps":     9, "nodes":     9, "pv":[4                ], "push":"4",               "pieces":7,                  "turn":"O", "comment":"Search." }
info json { "nps":    10, "nodes":    10, "pv":[4,1              ], "push":"1",               "pieces":8,                  "turn":"X", "comment":"Search." }
info json { "nps":    11, "nodes":    11, "pv":[4,1,2            ], "push":"2", "leaf": true, "pieces":9, "result":"draw", "turn":"O", "comment":"It is ok." }
info json { "nps":    11, "nodes":    11, "pv":[4,1              ], "pop" :"2",               "pieces":8, "result":"draw", "turn":"X" }
info json { "nps":    11, "nodes":    11, "pv":[4                ], "pop" :"1",               "pieces":7, "result":"draw", "turn":"O", "comment":"Fmmm." }
info json { "nps":    12, "nodes":    12, "pv":[4,2              ], "push":"2",               "pieces":8,                  "turn":"X", "comment":"Search." }
info json { "nps":    13, "nodes":    13, "pv":[4,2,1            ], "push":"1", "leaf": true, "pieces":9, "result":"draw", "turn":"O", "comment":"It is ok." }
info json { "nps":    13, "nodes":    13, "pv":[4,2              ], "pop" :"1",               "pieces":8, "result":"draw", "turn":"X" }
info json { "nps":    13, "nodes":    13, "pv":[4                ], "pop" :"2",               "pieces":7, "result":"draw", "turn":"O", "comment":"Fmmm." }
info json { "nps":    13, "nodes":    13, "pv":[                 ], "pop" :"4",               "pieces":6, "result":"draw", "turn":"X", "comment":"Fmmm." }
info string result=Draw nps=13
bestmove 4
do 4
pos
[Next 8 move(s) | Go O]

+---+---+---+ Please select a square. Example `do 7`
| O | X | X | マスを選んでください。例 `do 7`
+---+---+---+
| X | O | O |    7 8 9
+---+---+---+    4 5 6
|   |   | X |    1 2 3
+---+---+---+
xfen
xfen oxx/1o1/2x o moves 6 4
quit
```

## Introduction

Soft: Kifuwarabe(きふわらべ)  
Author: Satoshi TAKAHASHI (Handle: むずでょ)  

Participation:  

| Year | Game  | Event                                                              | Rank |  All |
| ---- | ----- | ------------------------------------------------------------------ | ---: | ---: |
| 2014 | Shogi | [SDT2](https://denou.jp/tournament2014/)                           |   24 |   25 |
| 2015 | Shogi | [WCSC25](http://www2.computer-shogi.org/wcsc25/index_e.html)       |   39 |   39 |
| 2015 | Shogi | [SDT3](https://denou.jp/tournament2015/)                           |   28 |   28 |
| 2016 | Go    | [UEC9](http://www.computer-go.jp/uec/public_html/eng/index.shtml)  |   25 |   30 |
| 2016 | Shogi | [WCSC26](http://www2.computer-shogi.org/wcsc26/index_e.html)       |   26 |   51 |
| 2016 | Shogi | [SDT4](https://denou.jp/tournament2016/)                           |   33 |   35 |
| 2017 | Go    | [UEC10](http://www.computer-go.jp/uec/public_html/eng/index.shtml) |   25 |   30 |
| 2017 | Shogi | [WCSC27](http://www2.computer-shogi.org/wcsc27/index_e.html)       |   47 |   50 |
| 2017 | Shogi | [SDT5](https://denou.jp/tournament2017/)                           |   42 |   42 |
| 2017 | Go    | [AIR2017](https://www.igoshogi.net/ai_ryusei2017/01/en/)           |   16 |   18 |
| 2018 | Shogi | [WCSC28](http://www2.computer-shogi.org/wcsc28/index_e.html)       |   55 |   56 |
| 2018 | Go    | [AIR2018](https://www.igoshogi.net/ai_ryusei/01/en/)               |   14 |   14 |
| 2019 | Shogi | [WCSC29](http://www2.computer-shogi.org/wcsc29/index_e.html)       |   52 |   56 |
| 2019 | Go    | [UEC11](http://entcog.c.ooco.jp/entcog/new_uec/en/)                |   15 |   18 |
| 2020 | Shogi | [WCSOC1](http://www2.computer-shogi.org/wcso1.html)                |   34 |   39 |

2020 Tic-tac-toe is new!  

Kifuwarabe's tic-tac-toe other programming languages edition:  

* [Go edition](https://github.com/muzudho/tic-tac-toe-on-golang)
* [Python edition](https://github.com/muzudho/tic-tac-toe-on-python)

## Set up

Set up Visual studio code.  

Supports multi-byte character strings:  

* `[File] - [Preferences] - [Settings]` from main menu.
* Key type `files.autoGuessEncoding` into `Search settings` text box.
* Check `Files: Auto Guess Encoding` check box each `User`, `Workspace`, `tic-tac-toe` tab.
* Save.

Appear the terminal:  

* `[Terminal] - [New Terminal]` from main menu.

Find out how to install Rust:  

* [Rust](https://www.rust-lang.org/)

## How to use

Cargo.toml:  

* `<Rust-project>`/Cargo.toml

```toml
# ...omitted...

[dependencies]

kifuwarabe_tic_tac_toe = "0.2.4" # Please input latest version.
casual_logger = "0.6.5" # Please input latest version.
```

Your code:  

* `<Rust-project>`/examples/main.rs

```rust
use casual_logger::{Level, Log};
use kifuwarabe_tic_tac_toe::{
    engine::{Engine, Response},
    log::LogExt,
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

    let mut engine = Engine::default();
    engine.title();

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

        if let Some(response) = engine.enter(&line) {
            match response {
                Response::Quit => {
                    break;
                }
            }
        }
    }

    // Wait for logging to complete.
    // ロギングが完了するまで待ちます。
    Log::flush();
}
```

Terminal:  

```shell
cargo run --example main
```

In game:  

```plain
Kifuwarabe's tic-tac-toe
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


```

Input:  

```plain
pos
```

Output:  

```plain
[Next 1 move(s) | Go O]

+---+---+---+ Please select a square. Example `do 7`
|   |   |   | マスを選んでください。例 `do 7`
+---+---+---+
|   |   |   |    7 8 9
+---+---+---+    4 5 6
|   |   |   |    1 2 3
+---+---+---+

```

Input:  

```plain
do 7
pos
```

Output:  

```plain
[Next 2 move(s) | Go X]

+---+---+---+ Please select a square. Example `do 7`
| O |   |   | マスを選んでください。例 `do 7`
+---+---+---+
|   |   |   |    7 8 9
+---+---+---+    4 5 6
|   |   |   |    1 2 3
+---+---+---+

```

Input:  

```plain
go
```

Output:  

```plain
...omitted...
info json { "nps":  1417, "nodes":  5669, "pv":[9,1,8,2,5        ], "pop" :"3",               "pieces":6, "result":"win" , "turn":"O" }
info json { "nps":  1417, "nodes":  5669, "pv":[9,1,8,2          ], "pop" :"5",               "pieces":5, "result":"lose", "turn":"X", "comment":"Damn!" }
info json { "nps":  1417, "nodes":  5670, "pv":[9,1,8,2,6        ], "push":"6",               "pieces":6,                  "turn":"O", "comment":"Search." }
info json { "nps":  1417, "nodes":  5671, "pv":[9,1,8,2,6,3      ], "push":"3", "leaf": true, "pieces":7, "result":"win" , "turn":"X", "comment":"Resign." }
info json { "nps":  1417, "nodes":  5671, "pv":[9,1,8,2,6        ], "pop" :"3",               "pieces":6, "result":"win" , "turn":"O" }
info json { "nps":  1417, "nodes":  5671, "pv":[9,1,8,2          ], "pop" :"6",               "pieces":5, "result":"lose", "turn":"X", "comment":"Damn!" }
info json { "nps":  1417, "nodes":  5671, "pv":[9,1,8            ], "pop" :"2",               "pieces":4, "result":"win" , "turn":"O", "comment":"Hooray!" }
info json { "nps":  1417, "nodes":  5671, "pv":[9,1              ], "pop" :"8",               "pieces":3, "result":"lose", "turn":"X", "comment":"Damn!" }
info json { "nps":  1417, "nodes":  5671, "pv":[9                ], "pop" :"1",               "pieces":2, "result":"win" , "turn":"O", "comment":"Hooray!" }
info json { "nps":  1417, "nodes":  5671, "pv":[                 ], "pop" :"9",               "pieces":1, "result":"lose", "turn":"X", "comment":"Damn!" }
info string result=Draw nps=1417
bestmove 5
```

Input:  

```plain
do 5
pos
```

Output:  

```plain
[Next 3 move(s) | Go O]

+---+---+---+ Please select a square. Example `do 7`
| O |   |   | マスを選んでください。例 `do 7`
+---+---+---+
|   | X |   |    7 8 9
+---+---+---+    4 5 6
|   |   |   |    1 2 3
+---+---+---+

```

Input:  

```plain
quit
```

## Knowledge

```plain
position xfen 3/3/3 o moves 5 1 2 8 4 6 3 7 9
                            -----------------
                            Game record. Numbers of square.
                    -
                    Turn at the start.
              -----
              Board at the start.
```

## How to program a tick-tac-tow game?

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  
**But logger's difficult to make, so use library.**  

* [x] Step 1. Use logger library.
  * [x] Use casual_logger library at 'Cargo.toml', 'main.rs'.
  * [x] Create the 'log.rs' file.
    * [x] Extend the logger.

Let's proceed with development while testing.  

* [x] Step 2. Create the `test.rs` file.
  * Add little by little as you progress through the steps.  

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [x] Step 3. Create the 'main.rs'(or 'lib.rs') and 'look_and_model.rs' file.
  * [x] Piece - "O", "X".
  * [x] Game result - Win/Draw/Lose.
  * [x] Position - It's the board.
  * [x] Search - Computer player search.
  * [x] Search info - Computer player search info.

If you want to play immediately, you have the talent of a game creator.  
Being able to control your position means being able to play.  

* [x] Step 4. 'position.rs'
  * [x] do_move
  * [x] undo_move
  * [x] opponent

Let's enter commands into the computer. Create a command line parser.  

* [x] Step 5. 'command_line_seek.rs'
  * [x] Starts with.
  * [x] Go next to.
  * [x] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [x] Step 6. 'uxi_protocol.rs'
  * [x] Do. (Before 'From XFEN') Excludes legal moves and winning/losing decisions.
  * [x] To XFEN.
  * [x] From XFEN.
  * [x] Undo.

Let's make a principal command.  

* [x] Step 7. Create the 'engine.rs' file. Command line.
  * [x] position.
  * [x] pos.
  * [x] do.
  * [x] undo.
  * [x] uxi.
  * [x] xfen.
* [x] Step 8. Create the 'src/main.rs' or 'examples/main.rs'.

Before you make a computer player, let's judge the outcome. And let's test.  

* [x] Step 9. 'win_lose_judgment.rs'
  * [x] Win.
  * [x] Draw - Not win, not lose, can not play.
  * [ ] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [x] Step 10. 'performance_measurement.rs'
  * [x] Seconds. - Stopwatch.
  * [x] Node per second.

Finally, let's make a computer player.  

* [x] Step 11. 'computer_player.rs'
  * [x] Search.
  * [ ] Evaluation - None.
* [x] 'main.py'
  * [x] Create "go" command.
* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'
