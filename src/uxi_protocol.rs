//! Converts a position into a string or restores a string into a position.  
//! 局面を文字列に変換したり、文字列を局面に復元します。  
use crate::log::LogExt;
use crate::look_and_model::{GameResult, Piece, Position};
use casual_logger::Log;

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
impl Position {
    /// Converts the current position to xfen.  
    /// 現局面を xfen に変換します。  
    pub fn to_xfen(&self) -> String {
        let mut xfen = String::default();
        xfen.push_str("xfen ");

        // Starting board.
        // 開始盤面。
        let mut spaces = 0;
        for sq in [7, 8, 9, 4, 5, 6, 1, 2, 3].iter() {
            if let Some(piece) = self.starting_board[*sq as usize] {
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

            if *sq == 9 || *sq == 6 {
                if 0 < spaces {
                    xfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                xfen.push('/');
            }
        }

        // Flush the remaining space.
        // 残っているスペースを flush します。
        if 0 < spaces {
            xfen.push_str(&spaces.to_string());
        }

        // Next stone at the start.
        // 開始局面で、次に置く石。
        match self.starting_turn {
            Piece::Nought => {
                xfen.push_str(" o");
            }
            Piece::Cross => {
                xfen.push_str(" x");
            }
        }

        // A game record.
        // 棋譜。
        if 0 < self.pieces_num - self.starting_pieces_num {
            xfen.push_str(" moves");
            for i in self.starting_pieces_num..self.pieces_num {
                xfen.push_str(&format!(" {}", self.history[i].to_string()));
            }
        }

        xfen.to_string()
    }

    /// Convert xfen to board.  
    /// xfen を盤に変換します。  
    pub fn from_xfen(xfen: &str) -> Option<Position> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut pos = Position::default();
        let mut starts = 0usize;
        // Square. 0 is unused.
        // マス。 0 は未使用。
        // 7 8 9
        // 4 5 6
        // 1 2 3
        // The upper left is 7.
        // 左上が7。
        let mut sq = 7;

        #[derive(Debug)]
        enum MachineState {
            /// Parse start.
            /// パース開始。
            Start,
            /// Analyzing the board on the initial stage.
            /// 初期局面の盤上を解析中。
            StartingBoard,
            /// My turn is being analyzed.
            /// 手番の解析中。
            Phase,
            /// Reading ` moves `.
            /// ` moves ` 読取中。
            MovesLabel,
            /// The game record is being analyzed.
            /// 棋譜の解析中。
            Moves,
        }
        let mut machine_state = MachineState::Start;
        // Read one character at a time.
        // １文字ずつ読取。
        for (i, ch) in xfen.chars().enumerate() {
            match machine_state {
                MachineState::Start => {
                    if i + 1 == "xfen ".len() {
                        // If you skip the top `xfen `, go to the next.
                        // 先頭の `xfen ` を読み飛ばしたら次へ。
                        machine_state = MachineState::StartingBoard;
                    }
                }
                MachineState::StartingBoard => match ch {
                    'x' => {
                        // It's not the order of the game, so I don't know the turn.
                        // 棋譜の順ではないので、手番は分かりません。
                        pos.starting_board[sq] = Some(Piece::Cross);
                        pos.pieces_num += 1;
                        sq += 1;
                    }
                    'o' => {
                        pos.starting_board[sq] = Some(Piece::Nought);
                        pos.pieces_num += 1;
                        sq += 1;
                    }
                    '1' => sq += 1,
                    '2' => sq += 2,
                    '3' => sq += 3,
                    '/' => sq -= 6,
                    ' ' => {
                        // Explicitly clone.
                        // 明示的にクローンしてください。
                        pos.board = pos.starting_board.clone();
                        pos.starting_pieces_num = pos.pieces_num;
                        machine_state = MachineState::Phase;
                    }
                    _ => {
                        Log::error(&format!("(Err.144) xfen starting_board error: {}", ch));
                        return None;
                    }
                },
                MachineState::Phase => {
                    match ch {
                        'x' => {
                            pos.starting_turn = Piece::Cross;
                            pos.turn = Piece::Cross;
                        }
                        'o' => {
                            pos.starting_turn = Piece::Nought;
                            pos.turn = Piece::Nought;
                        }
                        _ => {
                            Log::error(&format!("(Err.157) xfen phase error: {}", ch));
                            return None;
                        }
                    }
                    // Temporary memory.
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

    /// Place the pieces. If you are programming yourself, legal move decisions can be postponed.  
    /// 駒を置きます。自分でプログラミングするなら、合法手は後回しで構いません。  
    ///
    /// # Arguments
    ///
    /// * `arg_str` - The rest of the command line. Here is the place to put the pieces. For example, `1` or `7`. (コマンドラインの残り。ここでは駒を置く場所。 `1` とか `7` など)
    pub fn do_(&mut self, arg_str: &str) {
        let sq: usize = match arg_str.parse() {
            Ok(x) => x,
            Err(_x) => {
                Log::error(&format!(
                    "(Err.194) Please input 'do <number>'. args=|{}|",
                    arg_str
                ));
                return;
            }
        };

        // Legal hand judgment. There should be no stones in the destination square.
        // 合法手判定。 移動先のマスに石があってはいけません。
        if sq < 1 || 9 < sq {
            Log::error(&format!("(Err.204) Specify from 1 to 9. Square={}", sq));
            return;
        } else if let Some(_piece_val) = self.board[sq as usize] {
            Log::error(&format!(
                "(Err.211) Please put it in a place where there are no pieces. Square={}",
                sq
            ));
            return;
        }

        self.redo_move(sq);

        // Win/loss judgment. Let's implement this after creating Position::result and is_opponent_win().
        // 勝ち負け判定。 これは Position::result, is_opponent_win() を作ったあとで実装しましょう。
        if self.is_opponent_win() {
            if let Some(result) = Position::result(GameResult::Win, Some(self.opponent())) {
                Log::print_notice(&result);
            }
        } else if self.is_draw() {
            if let Some(result) = Position::result(GameResult::Draw, None) {
                Log::print_notice(&result);
            }
        }
    }

    /// 1 back.  
    /// １手戻します。  
    pub fn undo(&mut self) {
        self.undo_move();
    }
}
