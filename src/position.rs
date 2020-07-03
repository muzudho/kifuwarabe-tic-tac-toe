//! ポジション☆（＾～＾）局面とか言われるやつだぜ☆（＾～＾）
use crate::look_and_model::{Piece, Position};

impl Position {
    /// １手指すぜ☆（＾～＾）
    pub fn do_move(&mut self, addr: usize) {
        // 石を置くぜ☆（＾～＾）
        self.board[addr] = Some(self.friend);
        // 棋譜に記すぜ☆（＾～＾）
        self.history[self.pieces_num] = addr as u8;
        // 棋譜に記した後にカウンターを増やすぜ☆（＾～＾）
        self.pieces_num += 1;
        // 手番は交代だぜ☆（＾～＾）
        self.friend = self.opponent();
    }
    /// １手戻すぜ☆（＾～＾）
    pub fn undo_move(&mut self) {
        // 手番は交代だぜ☆（＾～＾）
        self.friend = self.opponent();
        // 手数は次の要素を指しているんで、先に戻してから、配列の中身を取り出せだぜ☆（＾～＾）
        self.pieces_num -= 1;
        // 置いたところの石は削除な☆（＾～＾）
        let addr = self.history[self.pieces_num];
        self.board[addr as usize] = None;
    }
    /// 相手番☆（＾～＾）
    pub fn opponent(&self) -> Piece {
        use crate::position::Piece::*;
        match self.friend {
            Nought => Cross,
            Cross => Nought,
        }
    }
}
