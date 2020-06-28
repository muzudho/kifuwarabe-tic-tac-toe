# Kifuwarabe's tic-tac-toe

〇×ゲームだぜ☆（＾～＾）

## Test run

あなたのローカルPCでテストする方法です。  

Input:  

```shell
cargo run
```

## Run

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

# How to make

* [x] 空っぽの盤を、デバッグ・ウィンドウへ表示
* [x] XFENを作成して、駒の配置の入力
* [x] 駒を置いての盤を、デバッグ・ウィンドウへ表示
* [x] UXIプロトコルを作成し、`pos` コマンド打鍵で盤を、ターミナルへ表示
* [x] `do` コマンドを作って、駒を置く
* [x] ポジション構造体に 手番(friend) 変数を追加。`do` コマンドで置かれる駒を交互にします
* [x] `is_win` 関数を作成し、３つ並んでいれば `win` と表示します
