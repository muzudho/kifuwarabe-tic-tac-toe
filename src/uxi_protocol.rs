//! 局面データを文字列にしたり、文字列を局面データに復元するのに使うぜ☆（＾～＾）
use crate::look_and_model::{GameResult, Piece, Position};
use crate::LogExt;
use casual_logger::Log;

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
        if 0 < self.pieces_num - self.starting_pieces_num {
            xfen.push_str(" moves");
            for i in self.starting_pieces_num..self.pieces_num {
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
                        // 手番の順ではないので、手番は分からないぜ☆（＾～＾）
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
                        pos.starting_pieces_num = pos.pieces_num;
                        machine_state = MachineState::Phase;
                    }
                    _ => {
                        Log::errorln(&format!("xfen starting_board error: {}", ch));
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
                            Log::errorln(&format!("xfen phase error: {}", ch));
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

    /// 未来へ駒を置く
    /// 最初は、合法手判定や勝敗判定をせずに　とりあえず動かせだぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `arg_str` - コマンドラインの残り。ここでは駒を置く場所。 `1` とか `7` など。
    pub fn do_(&mut self, arg_str: &str) {
        // Log::println(&format("Trace   | do_ line={}", line));
        let addr: usize = match arg_str.parse() {
            Ok(x) => x,
            Err(_x) => {
                Log::errorln(&format!(
                    "`do 数字` で入力してくれだぜ☆（＾～＾） 引数=|{}|",
                    arg_str
                ));
                return;
            }
        };

        // 合法手判定☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if addr < 1 || 9 < addr {
            Log::errorln(&format!("1～9 で指定してくれだぜ☆（＾～＾） 番地={}", addr));
            return;
        } else if let Some(_piece_val) = self.board[addr as usize] {
            Log::errorln(&format!(
                "移動先のマスに駒があってはダメだぜ☆（＾～＾） 番地={}",
                addr
            ));
            return;
        }

        self.do_move(addr);

        // 勝ち負け判定☆（*＾～＾*）
        // これは PositionHelper, WinLoseJudgment を作ってから実装しろだぜ☆（＾～＾）
        if self.is_opponent_win() {
            if let Some(result) = Position::result(GameResult::Win, Some(self.opponent())) {
                Log::println(&result);
            }
        } else if self.is_draw() {
            if let Some(result) = Position::result(GameResult::Draw, None) {
                Log::println(&result);
            }
        }
    }

    /// 未来の駒を１つ戻す
    pub fn undo(&mut self) {
        self.undo_move();
    }
}
