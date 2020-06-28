//! 局面データを文字列にしたり、文字列を局面データに復元するのに使うぜ☆（＾～＾）
use crate::piece::Piece;
use crate::position::Position;

impl Position {
    /// xfen を board に変換するぜ☆（＾～＾）
    pub fn from_xfen(xfen: &str) -> Option<Position> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut board = Position::default();

        // 文字数☆（＾～＾）
        let mut count = -1isize;
        // 番地☆（＾～＾） 0 は未使用☆（＾～＾）
        // 7 8 9
        // 4 5 6
        // 1 2 3
        let mut addr = 7;
        // Rust言語では文字列に配列のインデックスを使ったアクセスはできないので、
        // 一手間かけるぜ☆（＾～＾）
        for ch in xfen.chars() {
            // 先にカウントアップ☆（＾～＾）
            count += 1;
            if count < "xfen ".len() as isize {
                // 先頭のキーワードは読み飛ばすぜ☆（＾～＾）
                continue;
            }
            match ch {
                'x' => {
                    board.board[addr] = Some(Piece::Cross);
                    addr += 1;
                }
                'o' => {
                    board.board[addr] = Some(Piece::Nought);
                    addr += 1;
                }
                '1' => addr += 1,
                '2' => addr += 2,
                '3' => addr += 3,
                '/' => addr -= 6,
                ' ' => break,
                _ => panic!("xfen error: {}", ch),
            }
        }

        // TODO 次の手番
        Some(board)
    }

    /// 駒を置く
    /// 最初は、合法か判定せずに　とりあえず動かせだぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `move_` - 指し手。ここでは駒を置く場所。 `1` とか `7` など。
    pub fn do_(&mut self, move_: &str) {
        let addr: usize = match move_.parse() {
            Ok(x) => x,
            Err(x) => panic!("do_: move_={} {}", move_, x),
        };

        println!("Debug   | move_={} addr={}", move_, addr);

        // 合法手チェック☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if let Some(_piece_val) = self.board[addr as usize] {
            panic!(
                "移動先のマスに駒があってはダメだぜ☆（＾～＾） addr={}",
                addr
            )
        }

        self.board[addr] = Some(self.friend);
        self.change_phase();
    }
}
