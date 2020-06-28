# https://qrunch.net/@kiduki/entries/iZKrWR4ES5ZTLlk7?ref=qrunch

# https://hub.docker.com/_/ubuntu
FROM ubuntu:19.10

LABEL maintainer "muzudho <muzudho1@gmail.com>"

ENV USER muzudho
ENV GROUP muzudho
# ほんとはパスワード埋め込んではいけないんだが、ちゃんとした方法を説明するのがめんどくさいし☆（＾～＾）
ENV PASS muzudho
ENV HOME /home/${USER}
ENV SHELL /bin/bash

# 一般ユーザの追加
RUN useradd -m ${USER}
# sudo権限の付与
RUN gpasswd -a ${USER} sudo
# パスワード設定
RUN echo "${USER}:${PASS}" | chpasswd
# RUN/CMDでbashを利用する
SHELL ["/bin/bash", "-c"]

# よく使うパッケージをインストール。接続エラーしてしまうので４回に分ける。
RUN apt-get update && apt-get install -y \
    apt-utils \
    curl
RUN apt-get update && apt-get install -y \
    file \
    gcc
RUN apt-get update && apt-get install -y \
    git \
    less
RUN apt-get update && apt-get install -y \
    neovim \
    sudo

# RUN/CMDを実行するユーザを指定
USER ${USER}
WORKDIR /home/${USER}

# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /home/${USER}/.cargo/env
ENV PATH $PATH:/home/${USER}/.cargo/bin

# ファイルの転送
Copy src /home/${USER}/tic-tac-toe/src
RUN chown ${USER}:${GROUP} /home/${USER}/tic-tac-toe/src

Copy Cargo.toml /home/${USER}/tic-tac-toe/Cargo.toml
RUN chown ${USER}:${GROUP} /home/${USER}/tic-tac-toe/Cargo.toml

WORKDIR /home/${USER}/tic-tac-toe
