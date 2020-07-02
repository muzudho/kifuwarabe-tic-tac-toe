//! ポジション☆（＾～＾）局面とか言われるやつだぜ☆（＾～＾）

/// 1スタートで9まで☆（＾～＾） 配列には0番地もあるから、要素数は10だぜ☆（＾～＾）
pub const BOARD_LEN: usize = 10;

/// 盤上に置ける最大の駒数だぜ☆（＾～＾） ９マスしか置くとこないから９だぜ☆（＾～＾）
pub const SQUARES_NUM: usize = 9;

/// 駒とか、石とかのことだが、〇×は 何なんだろうな、マーク☆（＾～＾）？
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}

/// 局面☆（＾～＾）ゲームデータをセーブしたり、ロードしたりするときの保存されてる現状だぜ☆（＾～＾）
#[derive(Debug)]
pub struct Position {
    /// 次に盤に置く駒☆（＾～＾）
    /// 英語では 手番は your turn, 相手版は your opponent's turn なんで 手番という英語は無い☆（＾～＾）
    /// 自分という意味の単語はプログラム用語と被りまくるんで、
    /// あまり被らない 味方(friend) を手番の意味で たまたま使ってるだけだぜ☆（＾～＾）
    pub friend: Piece,

    /// 開始局面の盤の各マス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub starting_board: [Option<Piece>; BOARD_LEN],

    /// 現状の盤の各マス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub board: [Option<Piece>; BOARD_LEN],

    /// 棋譜だぜ☆（＾～＾）駒を置いた番地を並べてけだぜ☆（＾～＾）
    pub history: [u8; SQUARES_NUM],

    /// 盤の上に駒が何個置いてあるかだぜ☆（＾～＾）
    pub pieces_num: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            friend: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            board: [None; BOARD_LEN],
            history: [0; SQUARES_NUM],
            pieces_num: 0,
        }
    }
}
impl Position {
    /// 過去を追加☆（＾～＾）
    pub fn do_move(&mut self, addr: usize) {
        // 石を置くぜ☆（＾～＾）
        self.board[addr] = Some(self.friend);
        // 棋譜に記すぜ☆（＾～＾）
        self.history[self.pieces_num] = addr as u8;
        // 棋譜に記した後にカウンターを増やすぜ☆（＾～＾）
        self.pieces_num += 1;
        // 手番は交代だぜ☆（＾～＾）
        self.change_phase();
    }
    /// 過去を削除☆（＾～＾）
    pub fn undo_move(&mut self) {
        // 手番は交代だぜ☆（＾～＾）
        self.change_phase();
        // 手数は次の要素を指しているんで、先に戻してから、配列の中身を取り出せだぜ☆（＾～＾）
        self.pieces_num -= 1;
        // 置いたところの石は削除な☆（＾～＾）
        let addr = self.history[self.pieces_num];
        self.board[addr as usize] = None;
    }
    pub fn opponent(&self) -> Piece {
        use crate::position::Piece::*;
        match self.friend {
            Nought => Cross,
            Cross => Nought,
        }
    }
    pub fn change_phase(&mut self) {
        self.friend = self.opponent();
    }

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
