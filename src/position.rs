use crate::piece::Piece;

pub struct Position {
    /// 次に盤に置く駒☆（＾～＾）
    /// 英語では 手番は your turn, 相手版は your opponent's turn なんで 手番という英語は無い☆（＾～＾）
    /// 自分という意味の単語はプログラム用語と被りまくるんで、
    /// あまり被らない 味方(friend) を手番の意味で たまたま使ってるだけだぜ☆（＾～＾）
    pub friend: Piece,

    /// 盤のマス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub board: [Option<Piece>; 10],
}
impl Default for Position {
    fn default() -> Self {
        Position {
            friend: Piece::Nought,
            board: [None; 10],
        }
    }
}
impl Position {
    pub fn change_phase(&mut self) {
        use crate::piece::Piece::*;
        self.friend = match self.friend {
            Nought => Cross,
            Cross => Nought,
        };
    }
}
