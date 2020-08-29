//! Win/Lose judgment.  
//! 勝敗判定。  
use crate::look_and_model::Position;
use crate::BOARD_LEN;

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// It will now be determined whether the player who already placed the stone, not the player who placed the stone, won.
    /// これから石を置くプレイヤーではなく、既に石を置いたプレイヤーが勝ったか判定します。  
    pub fn is_opponent_win(&self) -> bool {
        // Since there are only 8 patterns, let's check all.
        // 8パターンしかないので、全部チェックしましょう。
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

    /// If the player who has already placed the stone is not winning
    /// and the current player has no place to place the stone, it is a draw.
    /// 既に石を置いたプレイヤーが勝っていなくて、今のプレイヤーが石を置く場所がなければ引き分けです。
    pub fn is_draw(&self) -> bool {
        if self.is_opponent_win() {
            return false;
        }
        for sq in 1..BOARD_LEN {
            if let None = self.board[sq] {
                return false;
            }
        }

        true
    }
}
