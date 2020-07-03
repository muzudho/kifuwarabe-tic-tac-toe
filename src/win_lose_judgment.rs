//! 勝敗判定☆（＾～＾）
use crate::look_and_model::{Position, BOARD_LEN};

impl Position {
    /// 石を置いてから 勝敗判定をするので、
    /// 反対側の手番のやつが 石を３つ並べてたかどうかを調べるんだぜ☆（＾～＾）
    pub fn is_opponent_win(&self) -> bool {
        // 8パターンしかないので、全部チェックしてしまおうぜ☆（＾～＾）

        let opponent = self.opponent();

        // xxx
        // ...
        // ...
        (Some(opponent) == self.board[7]
            && Some(opponent) == self.board[8]
            && Some(opponent) == self.board[9])
        // ...
        // xxx
        // ...
        || (Some(opponent) == self.board[4]
                && Some(opponent) == self.board[5]
                && Some(opponent) == self.board[6])
        // ...
        // ...
        // xxx
        || (Some(opponent) == self.board[1]
                && Some(opponent) == self.board[2]
                && Some(opponent) == self.board[3])
        // x..
        // x..
        // x..
        || (Some(opponent) == self.board[7]
            && Some(opponent) == self.board[4]
            && Some(opponent) == self.board[1])
        // .x.
        // .x.
        // .x.
        || (Some(opponent) == self.board[8]
            && Some(opponent) == self.board[5]
            && Some(opponent) == self.board[2])
        // ..x
        // ..x
        // ..x
        || (Some(opponent) == self.board[9]
            && Some(opponent) == self.board[6]
            && Some(opponent) == self.board[3])
        // x..
        // .x.
        // ..x
        || (Some(opponent) == self.board[7]
            && Some(opponent) == self.board[5]
            && Some(opponent) == self.board[3])
        // ..x
        // .x.
        // x..
        || (Some(opponent) == self.board[9]
            && Some(opponent) == self.board[5]
            && Some(opponent) == self.board[1])
    }

    /// 石を置いてから 引き分け判定をするので、
    /// 反対側の手番のやつが 勝ってなくて、
    /// かつ、全てのマスが埋まってたら引き分けだぜ☆（＾～＾）
    pub fn is_draw(&self) -> bool {
        if self.is_opponent_win() {
            return false;
        }
        for addr in 1..BOARD_LEN {
            if let None = self.board[addr] {
                return false;
            }
        }

        true
    }
}
