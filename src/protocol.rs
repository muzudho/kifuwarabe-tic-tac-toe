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

        #[derive(Debug)]
        enum MachineState {
            /// 最初☆（＾～＾）
            Start,
            /// 盤の解析中☆（＾～＾）
            Board,
            /// 手番の解析中☆（＾～＾）
            Phase,
        }
        let mut machine_state = MachineState::Start;
        // Rust言語では文字列に配列のインデックスを使ったアクセスはできないので、
        // 一手間かけるぜ☆（＾～＾）
        for ch in xfen.chars() {
            // 先にカウントアップ☆（＾～＾）
            count += 1;

            // 分け分からんバグが出たらデバッグ・ライトしろだぜ☆（＾～＾）
            /*
            println!(
                "Trace   | machine_state={:?}, addr={} count={} ch={}",
                machine_state, addr, count, ch
            );
            */

            match machine_state {
                MachineState::Start => {
                    if count + 1 == "xfen ".len() as isize {
                        // 先頭のキーワードを読み飛ばしたら次へ☆（＾～＾）
                        machine_state = MachineState::Board;
                    }
                }
                MachineState::Board => match ch {
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
                    ' ' => {
                        machine_state = MachineState::Phase;
                    }
                    _ => {
                        println!("Error   | xfen board error: {}", ch);
                        return None;
                    }
                },
                MachineState::Phase => {
                    match ch {
                        'x' => {
                            board.friend = Piece::Cross;
                        }
                        'o' => {
                            board.friend = Piece::Nought;
                        }
                        _ => {
                            println!("Error   | xfen phase error: {}", ch);
                            return None;
                        }
                    }
                    break;
                }
            }
        }

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
            Err(_x) => {
                println!(
                    "Error   | `do 数字` で入力してくれだぜ☆（＾～＾） 入力=|{}|",
                    move_
                );
                return;
            }
        };

        // println!("Debug   | move_={} addr={}", move_, addr);

        // 合法手チェック☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if let Some(_piece_val) = self.board[addr as usize] {
            println!(
                "Error   | 移動先のマスに駒があってはダメだぜ☆（＾～＾） 番地={}",
                addr
            );
            return;
        }

        self.board[addr] = Some(self.friend);

        // 勝ち負け判定☆（*＾～＾*）
        if self.is_win() {
            println!("win {}", self.friend);
        }
        self.change_phase();
    }
}
