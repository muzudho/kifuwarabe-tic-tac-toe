//! 局面データを文字列にしたり、文字列を局面データに復元するのに使うぜ☆（＾～＾）
use crate::log::Log;
use crate::piece::Piece;
use crate::position::Position;

impl Position {
    /// 現局面を xfen に変換するぜ☆（＾～＾）
    pub fn to_xfen(&self) -> String {
        let mut xfen = String::default();
        xfen.push_str("xfen ");

        // StartingBoard
        let mut spaces = 0;
        for addr in [7, 8, 9, 4, 5, 6, 1, 2, 3].iter() {
            if let Some(piece) = self.starting_board[*addr as usize] {
                if 0 < spaces {
                    xfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                xfen.push(match piece {
                    Piece::Nought => 'o',
                    Piece::Cross => 'x',
                });
            } else {
                spaces += 1;
            }

            if *addr == 9 || *addr == 6 {
                if 0 < spaces {
                    xfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                xfen.push('/');
            }
        }

        // 残ってるスペースの flush を忘れないぜ☆（＾～＾）
        if 0 < spaces {
            xfen.push_str(&spaces.to_string());
        }

        // Phase
        match self.friend {
            Piece::Nought => {
                xfen.push_str(" o");
            }
            Piece::Cross => {
                xfen.push_str(" x");
            }
        }

        // Moves
        if 0 < self.pieces_num {
            xfen.push_str(" moves");
            for i in 0..self.pieces_num {
                xfen.push_str(&format!(" {}", self.history[i].to_string()));
            }
        }

        xfen.to_string()
    }

    /// xfen を board に変換するぜ☆（＾～＾）
    pub fn from_xfen(xfen: &str) -> Option<Position> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut pos = Position::default();

        // 文字数☆（＾～＾）
        let mut starts = 0usize;
        // 番地☆（＾～＾） 0 は未使用☆（＾～＾）
        // 7 8 9
        // 4 5 6
        // 1 2 3
        let mut addr = 7;

        #[derive(Debug)]
        enum MachineState {
            /// 最初☆（＾～＾）
            Start,
            /// 初期局面の盤上を解析中☆（＾～＾）
            StartingBoard,
            /// 手番の解析中☆（＾～＾）
            Phase,
            /// ` moves ` 読取中☆（＾～＾）
            MovesLabel,
            /// 棋譜の解析中☆（＾～＾）
            Moves,
        }
        let mut machine_state = MachineState::Start;
        // Rust言語では文字列に配列のインデックスを使ったアクセスはできないので、
        // 一手間かけるぜ☆（＾～＾）
        for (i, ch) in xfen.chars().enumerate() {
            match machine_state {
                MachineState::Start => {
                    if i + 1 == "xfen ".len() {
                        // 先頭のキーワードを読み飛ばしたら次へ☆（＾～＾）
                        machine_state = MachineState::StartingBoard;
                    }
                }
                MachineState::StartingBoard => match ch {
                    'x' => {
                        pos.starting_board[addr] = Some(Piece::Cross);
                        pos.pieces_num += 1;
                        addr += 1;
                    }
                    'o' => {
                        pos.starting_board[addr] = Some(Piece::Nought);
                        pos.pieces_num += 1;
                        addr += 1;
                    }
                    '1' => addr += 1,
                    '2' => addr += 2,
                    '3' => addr += 3,
                    '/' => addr -= 6,
                    ' ' => {
                        // 明示的にクローン☆（＾～＾）
                        pos.board = pos.starting_board.clone();
                        machine_state = MachineState::Phase;
                    }
                    _ => {
                        Log::println(&format!("Error   | xfen starting_board error: {}", ch));
                        return None;
                    }
                },
                MachineState::Phase => {
                    match ch {
                        'x' => {
                            pos.friend = Piece::Cross;
                        }
                        'o' => {
                            pos.friend = Piece::Nought;
                        }
                        _ => {
                            Log::println(&format!("Error   | xfen phase error: {}", ch));
                            return None;
                        }
                    }
                    // 一時記憶。
                    starts = i;
                    machine_state = MachineState::MovesLabel;
                }
                MachineState::MovesLabel => {
                    if starts + " moves ".len() <= i {
                        machine_state = MachineState::Moves;
                    }
                }
                MachineState::Moves => {
                    if ch == ' ' {
                    } else {
                        pos.do_(&ch.to_string());
                    }
                }
            }
        }

        Some(pos)
    }

    /// 駒を置く
    /// 最初は、合法か判定せずに　とりあえず動かせだぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `move_` - 指し手。ここでは駒を置く場所。 `1` とか `7` など。
    pub fn do_(&mut self, line: &str) {
        // Log::println(&format("Trace   | do_ line={}", line));
        let addr: usize = match line.parse() {
            Ok(x) => x,
            Err(_x) => {
                Log::println(&format!(
                    "Error   | `do 数字` で入力してくれだぜ☆（＾～＾） 入力=|{}|",
                    line
                ));
                return;
            }
        };

        // 合法手チェック☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if addr < 1 || 9 < addr {
            Log::println(&format!(
                "Error   | 1～9 で指定してくれだぜ☆（＾～＾） 番地={}",
                addr
            ));
            return;
        } else if let Some(_piece_val) = self.board[addr as usize] {
            Log::println(&format!(
                "Error   | 移動先のマスに駒があってはダメだぜ☆（＾～＾） 番地={}",
                addr
            ));
            return;
        }

        self.board[addr] = Some(self.friend);

        // 勝ち負け判定☆（*＾～＾*）
        if self.is_win() {
            Log::println(&format!("win {}", self.friend));
        } else if self.is_draw() {
            Log::println(&format!("draw"));
        }

        self.add_move(addr as u8);
        self.change_phase();
    }

    /// 駒を戻す
    pub fn undo(&mut self) {
        self.change_phase();
        let addr = self.remove_move();
        Log::println(&format!("Trace   | undo addr={}", addr));
        self.board[addr as usize] = None;
    }
}
