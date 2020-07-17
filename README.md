# Kifuwarabe's tic-tac-toe

This is tic-tac-toe edition.  
Kifuwarabe(きふわらべ) is computer shogi/go thinking engine.  
There is **no** GUI (Graphical user interface), It's CUI (Characterbased user interface).  
Author: Satoshi TAKAHASHI (Handle: むずでょ)  

Participation:  

| Year | Game  | Event                                                              | Rank |  All |
| ---- | ----- | ------------------------------------------------------------------ | ---: | ---: |
| 2014 | Shogi | [SDT2](https://denou.jp/tournament2014/)                           |   24 |   25 |
| 2015 | Shogi | [WCSC25](http://www2.computer-shogi.org/wcsc25/)                   |   39 |   39 |
| 2015 | Shogi | [SDT3](https://denou.jp/tournament2015/)                           |   28 |   28 |
| 2016 | Go    | [UEC9](http://www.computer-go.jp/uec/public_html/eng/index.shtml)  |   25 |   30 |
| 2016 | Shogi | [WCSC26](http://www2.computer-shogi.org/wcsc26/)                   |   26 |   51 |
| 2016 | Shogi | [SDT4](https://denou.jp/tournament2016/)                           |   33 |   35 |
| 2017 | Go    | [UEC10](http://www.computer-go.jp/uec/public_html/eng/index.shtml) |   25 |   30 |
| 2017 | Shogi | [WCSC27](http://www2.computer-shogi.org/wcsc27/)                   |   47 |   50 |
| 2017 | Shogi | [SDT5](https://denou.jp/tournament2017/)                           |   42 |   42 |
| 2017 | Go    | [AIR2017](https://www.igoshogi.net/ai_ryusei2017/01/en/)           |   16 |   18 |
| 2018 | Shogi | [WCSC28](http://www2.computer-shogi.org/wcsc28/)                   |   55 |   56 |
| 2018 | Go    | [AIR2018](https://www.igoshogi.net/ai_ryusei/01/en/)               |   14 |   14 |
| 2019 | Shogi | [WCSC29](http://www2.computer-shogi.org/wcsc29/)                   |   52 |   56 |
| 2019 | Go    | [UEC11](http://entcog.c.ooco.jp/entcog/new_uec/en/)                |   15 |   18 |
| 2020 | Shogi | [WCSOC1](http://www2.computer-shogi.org/wcso1.html)                |   34 |   39 |

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

Code:  

```shell
cargo run --release
```

## How to make tic tac toe?

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  
**But logger's difficult to make, so use library.**  

* [x] Step 1. Use logger library.
  * [x] Use casual_logger library.
* [x] 'log.rs'
  * [x] Extend the logger.

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [x] Step 2. 'look_and_model.rs'
  * [x] Piece - "O", "X".
  * [x] Game result - Win/Draw/Lose.
  * [x] Position - It's the board.
  * [x] Search - Computer player search info.

If you want to play immediately, you have the talent of a game creator.  
Being able to control your position means being able to play.  

* [x] Step 3. 'position.rs'
  * [x] do_move
  * [x] undo_move
  * [x] opponent

Let's enter commands into the computer. Create a command line parser.  

* [x] Step 4. 'command_line_parser.rs'
  * [x] Input.
  * [x] Starts with.
  * [x] Go next to.
  * [x] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [x] Step 5. 'uxi_protocol.rs'
  * [x] To XFEN.
  * [x] Do. (Before 'From XFEN') Excludes legal moves and winning/losing decisions.
  * [x] From XFEN.
  * [x] Undo.

Let's make a principal command.  

* [x] Step 6. 'main.rs'
  * [x] position.
  * [x] pos.
  * [x] do.
  * [x] undo.
  * [x] uxi.
  * [x] xfen.

Before you make a computer player, let's judge the outcome. And let's test.  

* [x] Step 7. 'win_lose_judgment.rs'
  * [x] Win.
  * [x] Draw - Not win, not lose, can not play.
  * [ ] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [x] Step 8. 'performance_measurement.rs'
  * [x] Seconds. - Stopwatch.
  * [x] Node per second.

Finally, let's make a computer player.  

* [x] Step 9. 'computer_player.rs'
  * [x] Search.
  * [ ] Evaluation - None.
* [x] 'main.py'
  * [x] Create "go" command.
* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'
