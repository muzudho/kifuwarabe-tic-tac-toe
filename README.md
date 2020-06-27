# tic-tac-toe
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