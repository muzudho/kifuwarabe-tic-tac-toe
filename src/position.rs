use crate::piece::Piece;

/// 1スタートで9まで☆（＾～＾） 配列には0番地もあるから、要素数は10だぜ☆（＾～＾）
const BOARD_LEN: usize = 10;

/// ９マスしか置くとこないから最大９手なんだが、配列の 0 はややこしいんで使わないことにして、要素数は10だぜ☆（＾～＾）
const MOVES_LEN: usize = 10;

pub struct Position {
    /// 次に盤に置く駒☆（＾～＾）
    /// 英語では 手番は your turn, 相手版は your opponent's turn なんで 手番という英語は無い☆（＾～＾）
    /// 自分という意味の単語はプログラム用語と被りまくるんで、
    /// あまり被らない 味方(friend) を手番の意味で たまたま使ってるだけだぜ☆（＾～＾）
    pub friend: Piece,

    /// 盤のマス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub board: [Option<Piece>; BOARD_LEN],

    /// 棋譜だぜ☆（＾～＾）駒を置いた番地を並べてけだぜ☆（＾～＾）
    pub moves: [u8; MOVES_LEN],

    /// 何手目かだぜ☆（＾～＾）初期局面では 1 ☆（＾～＾） 0 は使わないぜ☆（＾～＾）
    pub moves_num: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            friend: Piece::Nought,
            board: [None; BOARD_LEN],
            moves: [0; MOVES_LEN],
            moves_num: 1,
        }
    }
}
impl Position {
    pub fn add_move(&mut self, addr: u8) {
        self.moves[self.moves_num] = addr;
        self.moves_num += 1;
    }
    pub fn remove_move(&mut self) {
        self.moves_num -= 1;
    }
    pub fn change_phase(&mut self) {
        use crate::piece::Piece::*;
        self.friend = match self.friend {
            Nought => Cross,
            Cross => Nought,
        };
    }
    /// 手番を持ってる方が３つ並べてたら真だぜ☆（＾～＾）
    pub fn is_win(&self) -> bool {
        // 8パターンしかないので、全部チェックしてしまおうぜ☆（＾～＾）

        // xxx
        // ...
        // ...
        (Some(self.friend) == self.board[7]
            && Some(self.friend) == self.board[8]
            && Some(self.friend) == self.board[9])
        // ...
        // xxx
        // ...
        || (Some(self.friend) == self.board[4]
                && Some(self.friend) == self.board[5]
                && Some(self.friend) == self.board[6])
        // ...
        // ...
        // xxx
        || (Some(self.friend) == self.board[1]
                && Some(self.friend) == self.board[2]
                && Some(self.friend) == self.board[3])
        // x..
        // x..
        // x..
        || (Some(self.friend) == self.board[7]
            && Some(self.friend) == self.board[4]
            && Some(self.friend) == self.board[1])
        // .x.
        // .x.
        // .x.
        || (Some(self.friend) == self.board[8]
            && Some(self.friend) == self.board[5]
            && Some(self.friend) == self.board[2])
        // ..x
        // ..x
        // ..x
        || (Some(self.friend) == self.board[9]
            && Some(self.friend) == self.board[6]
            && Some(self.friend) == self.board[3])
        // x..
        // .x.
        // ..x
        || (Some(self.friend) == self.board[7]
            && Some(self.friend) == self.board[5]
            && Some(self.friend) == self.board[3])
        // ..x
        // .x.
        // x..
        || (Some(self.friend) == self.board[9]
            && Some(self.friend) == self.board[5]
            && Some(self.friend) == self.board[1])
    }
}
