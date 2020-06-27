//! 局面データを文字列にしたり、文字列を局面データに復元するのに使うぜ☆（＾～＾）
use crate::board::Board;
use crate::piece::Piece;

impl Board {
    /// xfen を board に変換するぜ☆（＾～＾）
    pub fn from_xfen(xfen: &str) -> Option<Board> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut board = Board::default();

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
                    board.squares[addr] = Some(Piece::Cross);
                    addr += 1;
                }
                'o' => {
                    board.squares[addr] = Some(Piece::Nought);
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

    /*
    /// TODO 指す
    /// 最初は、合法か判定せずに　とりあえず動かせだぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `move_` - 指し手。`b5c3` など。
    pub fn do_(&mut self, move_: &str) {
        fn a_to_u8(ch: char) -> u8 {
            match ch {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                _ => panic!("知らないアルファベットだぜ☆（＾～＾） ch={}", ch),
            }
        }
        fn n_to_u8(ch: char) -> u8 {
            match ch {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                _ => panic!("知らない数字だぜ☆（＾～＾） ch={}", ch),
            }
        }

        let mut src_sq = 0;
        let mut dst_sq = 0;

        for (i, ch) in move_.chars().enumerate() {
            match i {
                0 => {
                    src_sq += 10 * a_to_u8(ch);
                }
                1 => {
                    src_sq += n_to_u8(ch);
                }
                2 => {
                    dst_sq += 10 * a_to_u8(ch);
                }
                3 => {
                    dst_sq += n_to_u8(ch);
                }
                _ => panic!("知らない構文だぜ☆（＾～＾） i={}", i),
            }
        }

        println!(
            "Debug   | do_ move_={} src_sq={} dst_sq={}",
            move_, src_sq, dst_sq
        );

        // 合法手チェック☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if let Some(_piece_val) = self.pieces[dst_sq as usize] {
            panic!(
                "移動先のマスに駒があってはダメだぜ☆（＾～＾） dst_sq={}",
                dst_sq
            )
        }

        self.pieces[dst_sq as usize] = self.pieces[src_sq as usize];
        self.pieces[src_sq as usize] = None;
    }
    */
}
