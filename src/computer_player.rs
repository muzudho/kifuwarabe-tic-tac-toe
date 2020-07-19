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
                let mut backward_cut_off = None;
                let mut info_leaf = false;
                let mut info_backwarding = None;
                let mut info_result = None;
                let mut info_comment = None;
                // Let's put a stone for now.
                // とりあえず石を置きましょう。
                pos.do_move(sq);
                self.nodes += 1;

                // Find out why you are not doing a forward search.
                // If not, I will search.
                // 前向き検索を行わない理由を調べてください。
                // 無ければ探索します。
                let forward_cut_off = if pos.is_opponent_win() {
                    // The opponent wins.
                    // 対戦相手の勝ち。
                    if Log::enabled(Level::Info) {
                        info_result = Some(GameResult::Win);
                        info_comment = Some("Resign.");
                    }
                    Some(ForwardCutOff::OpponentWin)
                } else if SQUARES_NUM <= pos.pieces_num {
                    // Draw if there is no place to put.
                    // 置く場所が無ければ引き分け。
                    if Log::enabled(Level::Info) {
                        info_leaf = true;
                        info_result = Some(GameResult::Draw);
                        info_comment = Some("It is ok.");
                    }
                    Some(ForwardCutOff::Draw)
                } else {
                    if Log::enabled(Level::Info) {
                        info_comment = Some("Search.");
                    }
                    None
                };

                // (1) Outputs information for forward search.
                // (一) 前向き探索の情報を出力します。
                Log::print_info(&Search::info_str(
                    self.nps(),
                    self.nodes,
                    &pos.pv,
                    SearchDirection::Forward,
                    sq,
                    info_leaf,
                    None,
                    info_result,
                    pos.turn,
                    info_comment,
                ));

                if let None = forward_cut_off {
                    // If you move forward, it's your opponent's turn.
                    // 前向きに探索したら、次は対戦相手の番です。
                    let (_opponent_sq, opponent_game_result) = self.node(pos);

                    // I'm back.
                    // 戻ってきました。
                    info_backwarding = Some(opponent_game_result);
                }

                // (2) Remove the placed stone.
                // (二) 置いた石は取り除きます。
                pos.undo_move();

                if let Some(opponent_game_result) = info_backwarding {
                    match opponent_game_result {
                        GameResult::Lose => {
                            // I beat the opponent.
                            // 相手を負かしました。
                            info_result = Some(GameResult::Win);
                            info_comment = Some("Hooray!");

                            // The search ends.
                            // 探索を終了します。
                            backward_cut_off = Some(BackwardCutOff::YouWin);
                        }
                        GameResult::Draw => {
                            // If neither is wrong, draw.
                            // お互いがミスしなければ引き分け。
                            info_result = Some(GameResult::Draw);
                            info_comment = Some("Fmmm.");

                            match best_result {
                                GameResult::Lose => {
                                    // If it gets better, change it to this. Generally called 'Update alpha evaluation'.
                                    // 良くなるならこの手に変えます。一般的には 'α評価値の更新' と呼びます。
                                    best_sq = Some(sq as u8);
                                    best_result = GameResult::Draw;
                                }
                                _ => {}
                            }
                            // I will continue.
                            // まだ続けます。
                        }
                        GameResult::Win => {
                            // Don't choose to lose.
                            // 自分が負ける手は選びません。
                            info_result = Some(GameResult::Lose);
                            info_comment = Some("Damn!");

                            // I will continue.
                            // まだ続けます。
                        }
                    }
                }

                // (3) Outputs backward search information.
                // (三) 後ろ向き探索の情報を出力します。
                if Log::enabled(Level::Info) {
                    Log::print_info(&Search::info_str(
                        self.nps(),
                        self.nodes,
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
                if let Some(forward_cut_off) = forward_cut_off {
                    match forward_cut_off {
                        ForwardCutOff::OpponentWin => {
                            return (Some(sq as u8), GameResult::Win);
                        }
                        ForwardCutOff::Draw => {
                            return (Some(sq as u8), GameResult::Draw);
                        }
                    }
                } else if let Some(backward_cut_off) = backward_cut_off {
                    match backward_cut_off {
                        BackwardCutOff::YouWin => {
                            return (Some(sq as u8), GameResult::Win);
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
enum ForwardCutOff {
    /// End with a opponent win.  
    /// 相手の勝ちにつき、終了。  
    OpponentWin,
    /// End with a draw.  
    /// 引き分けにつき、終了。  
    Draw,
}

/// The reason for ending the backward search.  
/// 後ろ向き探索を終了した理由。  
enum BackwardCutOff {
    /// End with a you win.  
    /// あなたの勝ちにつき、終了。  
    YouWin,
}
