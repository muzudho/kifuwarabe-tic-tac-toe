//! Position.  
//! 局面。  
use crate::look_and_model::{Piece, Position};

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// Place the stone.  
    /// １手指します。  
    pub fn do_move(&mut self, sq: usize) {
        // I placed a stone.
        // 石を置いた。
        self.board[sq] = Some(self.friend);
        // Write on the game record.
        // 棋譜に書きます。
        self.history[self.pieces_num] = sq as u8;
        // After writing on the game, count the stones you have placed.
        // 棋譜に書いたあと、置いた石を数えます。
        self.pieces_num += 1;
        // Change of turn.
        // 手番交代。
        self.friend = self.opponent();
    }

    /// 1 back.  
    /// 1手戻します。  
    pub fn undo_move(&mut self) {
        // Change of turn.
        // 手番交代。
        self.friend = self.opponent();
        // The number of stones points to the next element of the array,
        // so first reduce it and then extract the contents of the array.
        // 石の数は配列の次の要素を指しているので、先に戻してから、配列の中身を取り出してください。
        self.pieces_num -= 1;
        // Turn off the stone.
        // 石を消します。
        let sq = self.history[self.pieces_num];
        self.board[sq as usize] = None;
    }
    /// Opponent.
    /// 相手番。
    pub fn opponent(&self) -> Piece {
        use crate::position::Piece::*;
        match self.friend {
            Nought => Cross,
            Cross => Nought,
        }
    }
}
