# Kifuwarabe's tic-tac-toe

〇×ゲームだぜ☆（＾～＾）  

* [プログラミング入門動画](https://www.youtube.com/playlist?list=PLllMJuAE0iK6kSsi96OBXBgNZHFg7KB9f) - 小学生は変女に Scratch でも教えられてろだぜ☆（＾～＾）
* 他のプログラム言語で○×ゲーム
  * [Go ver](https://github.com/muzudho/tic-tac-toe-on-golang)
  * [Python ver](https://github.com/muzudho/tic-tac-toe-on-python)

## Visual Studio Code を使っている人へ

* やれ。
  * メインメニューから `[File] - [Preferences] - [Settings]` を開く。
  * Search settings 欄へ `files.autoGuessEncoding` と入力。
  * `Files: Auto Guess Encoding` チェックボックスをチェック。
  * まだ Settings を閉じるなだぜ。視認性の悪い画面を良く見ろ、 `User`, `Workspace`, `tic-tac-toe` と３つタブが並んでるだろ☆ それぞれある☆
  * 保存。

## Test run

あなたのローカルPCでテストする方法です。  

Input:  

```shell
cargo run
```

## Run

**Dockerなんか　うまくいかね☆（＾～＾）**

あらかじめ、Docker をインストールしておいてください。  
このエグザンプルでは、動作確認は、Dockerコンテナ上の仮想Ubuntuで行うものとします。  

Dockerイメージを作成するために、以下のコマンドを打鍵してください。  

Input:  

```shell
docker build --tag tic-tac-toe:0.1 .
```

Dockerコンテナを起動するために、以下のコマンドを打鍵してください。  

Input:  

```shell
docker run --name tic-tac-toe -it --rm tic-tac-toe:0.1
```

TODO カレント・ディレクトリの移動
TODO cargo run

## How to make tic tac toe?

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  
**But logger's difficult to make, so copy and paste it.**  

* [x] Step 1. Use logger library. (You can **COPY & PASTE** in 1 minutes)
  * [x] Use casual_logger library.

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
  * [ ] uxi.
  * [x] xfen.

Before you make a computer player, let's judge the outcome. And let's test.  

* [x] Step 7. 'win_lose_judgment.rs'
  * [x] Win.
  * [x] Draw - Not win, not lose, can not play.
  * [-] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [x] Step 8. 'performance_measurement.rs'
  * [x] Seconds. - Stopwatch.
  * [x] Node per second.

Finally, let's make a computer player.  

* [x] Step 9. 'computer_player.rs'
  * [x] Search.
  * [-] Evaluation - None.
* [x] 'main.py'
  * [x] Create "go" command.
* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'

# Test case

```plain
# mate +3 の局面。 5 -> [6,9]どう打っても -> [9,6]。
# x..
# o..
# o.x
position xfen x2/o2/o1x o

# mate +5 の局面？
# ...
# ...
# o.x
position xfen 3/3/o1x o

# Draw の局面。 9。
# xo.
# oxx
# xoo
position xfen xo1/oxx/xoo o
```

# Documentation

Input:  

```shell
cargo doc --open
```

# Test

Input:  

```shell
cargo test
```
