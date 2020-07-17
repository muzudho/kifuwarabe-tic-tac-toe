use crate::look_and_model::{GameResult, Position, Search, BOARD_LEN, SQUARES_NUM};
use crate::LogExt;
use casual_logger::{Level, Log};

/// Search.  
/// 探索部。  
impl Search {
    /// This is the place to put the stone.  
    /// 石を置く場所です。  
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        if Log::enabled(Level::Info) {
            Log::print_info(&Search::info_header(pos));
        }
        self.node(pos)
    }

    /// The state node of the search tree. Commonly called search.  
    /// 検索ツリーの状態ノード。一般に 'search' と呼ばれます。  
    fn node(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        let mut best_sq = None;
        let mut best_result = GameResult::Lose;

        for sq in 1..BOARD_LEN {
            // I only look at the empty square.
            // 空きマスだけを見ます。
            if let None = pos.board[sq] {
                // Let's put a stone for now.
                // とりあえず石を置きましょう。
                pos.do_move(sq);
                self.nodes += 1;

                // Prior knowledge. Proceeding from the root toward the leaves is called a forward search.
                // The process of returning from the leaves toward the root is called backward search.
                // 予備知識。 根から葉に向かって進んでいることを前向き探索と呼びます。
                // 葉から根に戻っていることを後ろ向き探索と呼びます。

                // Determine if opponent have won.
                // 対戦相手が勝ったかどうかを確認します。
                if pos.is_opponent_win() {
                    // The opponent wins.
                    // 対戦相手の勝ち。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_forward_leaf(
                            self.nps(),
                            pos,
                            sq,
                            GameResult::Win,
                            Some("Hooray!"),
                        ));
                    }

                    // (2) Remove the placed stone.
                    // (二) 置いた石は取り除きます。
                    pos.undo_move();

                    // (3) Outputs backward search information.
                    // (三) 後ろ向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_backward(
                            self.nps(),
                            pos,
                            sq,
                            GameResult::Win,
                            None,
                        ));
                    }
                    // (4) The search ends.
                    // (四) 探索を終了します。
                    return (Some(sq as u8), GameResult::Win);
                } else if SQUARES_NUM <= pos.pieces_num {
                    // Draw if there is no place to put.
                    // 置く場所が無ければ引き分け。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_forward_leaf(
                            self.nps(),
                            pos,
                            sq,
                            GameResult::Draw,
                            Some("It's ok."),
                        ));
                    }

                    // (2) Remove the placed stone.
                    // (二) 置いた石は取り除きます。
                    pos.undo_move();

                    // (3) Outputs backward search information.
                    // (三) 後ろ向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_backward(
                            self.nps(),
                            pos,
                            sq,
                            GameResult::Draw,
                            None,
                        ));
                    }
                    // (4) The search ends.
                    // (四) 探索を終了します。
                    return (Some(sq as u8), GameResult::Draw);
                } else {
                    // I will continue.
                    // まだ続けます。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_forward(self.nps(), pos, sq, Some("Search.")));
                    }
                }

                // It's opponent's turn.
                // 相手の番です。
                let (_opponent_sq, opponent_game_result) = self.node(pos);

                // (2) Remove the placed stone.
                // (二) 置いた石は取り除きます。
                pos.undo_move();

                match opponent_game_result {
                    GameResult::Lose => {
                        // I beat the opponent.
                        // 相手を負かしました。

                        // (3) Outputs backward search information.
                        // (三) 後ろ向き探索の情報を出力します。
                        if Log::enabled(Level::Info) {
                            Log::print_info(&self.info_backward(
                                self.nps(),
                                pos,
                                sq,
                                GameResult::Win,
                                Some("Ok."),
                            ));
                        }

                        // (4) The search ends.
                        // (四) 探索を終了します。
                        return (Some(sq as u8), GameResult::Win);
                    }
                    GameResult::Draw => {
                        // If neither is wrong, draw.
                        // お互いがミスしなければ引き分け。

                        // (3) Outputs backward search information.
                        // (三) 後ろ向き探索の情報を出力します。
                        if Log::enabled(Level::Info) {
                            Log::print_info(&self.info_backward(
                                self.nps(),
                                pos,
                                sq,
                                GameResult::Draw,
                                Some("Fmmm."),
                            ));
                        }

                        // (4) I will continue.
                        // (四) まだ続けます。
                        match best_result {
                            GameResult::Lose => {
                                // If it gets better, change it to this. Generally called 'Update alpha evaluation'.
                                // 良くなるならこの手に変えます。一般的には 'α評価値の更新' と呼びます。
                                best_sq = Some(sq as u8);
                                best_result = GameResult::Draw;
                            }
                            _ => {}
                        }
                    }
                    GameResult::Win => {
                        // Don't choose to lose.
                        // 自分が負ける手は選びません。

                        // (4) I will continue.
                        // (四) まだ続けます。
                        if Log::enabled(Level::Info) {
                            Log::print_info(&self.info_backward(
                                self.nps(),
                                pos,
                                sq,
                                GameResult::Lose,
                                Some("Resign."),
                            ));
                        }
                    }
                }
            }
        }

        // End of turn.
        // 手番の終わり。
        (best_sq, best_result)
    }
}
