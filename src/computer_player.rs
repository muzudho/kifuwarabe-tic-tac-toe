//! The thinking department of a computer.  
//! See 'Search' struct in 'look_and_model' for details.  
//! コンピューターの思考部です。  
//! 詳しくは 'look_and_model' の 'Search' 構造体 を見てください。  
use crate::look_and_model::{
    GameResult, Position, Search, SearchDirection, BOARD_LEN, SQUARES_NUM,
};
use crate::LogExt;
use casual_logger::{Level, Log};

/// Search.  
/// 探索部。  
impl Search {
    /// This is the place to put the stone.  
    /// 石を置く場所です。  
    ///
    /// # Arguments
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        if Log::enabled(Level::Info) {
            Log::print_info(&Search::info_header(pos));
        }
        self.node(pos)
    }

    /// The state node of the search tree. Commonly called search.  
    /// 検索ツリーの状態ノード。一般に 'search' と呼ばれます。  
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    fn node(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        let mut best_sq = None;
        let mut best_result = GameResult::Lose;

        for sq in 1..BOARD_LEN {
            // I only look at the empty square.
            // 空きマスだけを見ます。
            if let None = pos.board[sq] {
                let mut cut_off = None;
                let mut info_leaf = false;
                let mut info_result = None;
                let mut info_comment = None;
                // Let's put a stone for now.
                // とりあえず石を置きましょう。
                pos.do_move(sq);
                self.nodes += 1;

                // Determine if opponent have won.
                // 対戦相手が勝ったかどうかを確認します。
                if pos.is_opponent_win() {
                    // The opponent wins.
                    // 対戦相手の勝ち。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_str(
                            self.nps(),
                            &pos.pv,
                            SearchDirection::Forward,
                            sq,
                            true,
                            None,
                            Some(GameResult::Win),
                            pos.turn,
                            Some("Resign."),
                        ));
                    }

                    // (2) Remove the placed stone.
                    // (二) 置いた石は取り除きます。
                    pos.undo_move();

                    // (3) Outputs backward search information.
                    // (三) 後ろ向き探索の情報を出力します。
                    info_result = Some(GameResult::Win);

                    // (4) The search ends.
                    // (四) 探索を終了します。
                    cut_off = Some(CutOff::Win);
                } else if SQUARES_NUM <= pos.pieces_num {
                    // Draw if there is no place to put.
                    // 置く場所が無ければ引き分け。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    info_leaf = true;
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_str(
                            self.nps(),
                            &pos.pv,
                            SearchDirection::Forward,
                            sq,
                            info_leaf,
                            None,
                            Some(GameResult::Draw),
                            pos.turn,
                            Some("It is ok."),
                        ));
                    }

                    // (2) Remove the placed stone.
                    // (二) 置いた石は取り除きます。
                    pos.undo_move();

                    // (3) Outputs backward search information.
                    // (三) 後ろ向き探索の情報を出力します。
                    info_result = Some(GameResult::Draw);

                    // (4) The search ends.
                    // (四) 探索を終了します。
                    cut_off = Some(CutOff::Draw);
                } else {
                    // I will continue.
                    // まだ続けます。

                    // (1) Outputs information for forward search.
                    // (一) 前向き探索の情報を出力します。
                    if Log::enabled(Level::Info) {
                        Log::print_info(&self.info_str(
                            self.nps(),
                            &pos.pv,
                            SearchDirection::Forward,
                            sq,
                            false,
                            None,
                            None,
                            pos.turn,
                            Some("Search."),
                        ));
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
                            info_result = Some(GameResult::Win);
                            info_comment = Some("Hooray!");

                            // (4) The search ends.
                            // (四) 探索を終了します。
                            cut_off = Some(CutOff::Win);
                        }
                        GameResult::Draw => {
                            // If neither is wrong, draw.
                            // お互いがミスしなければ引き分け。
                            // (3) Outputs backward search information.
                            // (三) 後ろ向き探索の情報を出力します。
                            info_result = Some(GameResult::Draw);
                            info_comment = Some("Fmmm.");

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

                            // (3) Outputs backward search information.
                            // (三) 後ろ向き探索の情報を出力します。
                            info_result = Some(GameResult::Lose);
                            info_comment = Some("Damn!");

                            // (4) I will continue.
                            // (四) まだ続けます。
                        }
                    }
                }

                // (3) Outputs backward search information.
                // (三) 後ろ向き探索の情報を出力します。
                if Log::enabled(Level::Info) {
                    Log::print_info(&self.info_str(
                        self.nps(),
                        &pos.pv,
                        SearchDirection::Backward,
                        sq,
                        info_leaf,
                        Some(pos.pieces_num),
                        info_result,
                        pos.turn,
                        info_comment,
                    ));
                }

                // (4) Depending on the condition, the sibling node search is skipped.
                // (四) 条件によっては、兄弟ノードの検索がスキップされます。
                if let Some(cut_off) = cut_off {
                    match cut_off {
                        CutOff::Win => {
                            return (Some(sq as u8), GameResult::Win);
                        }
                        CutOff::Draw => {
                            return (Some(sq as u8), GameResult::Draw);
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

/// The reason for ending the forward search.
/// 前向き探索を終了した理由。
enum CutOff {
    /// End with a win.
    /// 勝ちにつき、終了。
    Win,
    /// End with a draw.
    /// 引き分けにつき、終了。
    Draw,
}
