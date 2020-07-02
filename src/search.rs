use crate::log::Log;
use crate::piece::Piece;
use crate::position::{Position, BOARD_LEN, MAX_MOVES, SQUARES_NUM};
use std::time::Instant;

/// 探索部☆（＾～＾）
pub struct Search {
    /// この探索を始めたのはどっち側か☆（＾～＾）
    pub root_friend: Piece,
    /// この探索を始めたのは何手目か☆（＾～＾）
    root_move_num: usize,
    /// 現在局面からの、棋譜☆（＾～＾）
    history: [u8; SQUARES_NUM],
    /// 現在局面から置いた石の数☆（＾～＾）
    pub pieces_num: usize,
    /// 探索したノード数☆（＾～＾）
    pub nodes: u32,
    /// この構造体を生成した時点からストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
    /*
    /// 勝ち筋☆（＾～＾）
    best_pv: BestPv,
    */
}
impl Search {
    pub fn new(friend: Piece, move_num: usize) -> Self {
        Search {
            root_friend: friend,
            root_move_num: move_num,
            history: [0; SQUARES_NUM],
            pieces_num: 0,
            nodes: 0,
            stopwatch: Instant::now(),
            // best_pv: BestPv::default(),
        }
    }
    fn sec(&self) -> u64 {
        self.stopwatch.elapsed().as_secs()
    }

    pub fn nps(&self) -> u64 {
        let sec = self.sec();
        if 0 < sec {
            self.nodes as u64 / sec
        } else {
            0
        }
    }

    /// Principal variation. 今読んでる読み筋☆（＾～＾）
    pub fn pv(&self) -> String {
        let mut pv = String::new();
        for d in 0..self.pieces_num {
            pv.push_str(&format!("{} ", self.history[d]));
        }
        pv.trim_end().to_string()
    }
    /// 最善の番地を返すぜ☆（＾～＾）
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, Option<i8>) {
        self.info_header(pos);
        self.node(pos)
    }

    fn node(&mut self, pos: &mut Position) -> (Option<u8>, Option<i8>) {
        let mut best_addr = None;
        // 0,1,-2,3,-4... のように、0を除くと、 正の奇数（勝ち）と、負の偶数（負け）が交互に出てくるぜ☆（＾～＾）
        let mut cur_mate: Option<i8> = None;

        for addr in 1..BOARD_LEN {
            // 空きマスがあれば
            if let None = pos.board[addr] {
                // とりあえず置いてみようぜ☆（＾～＾）
                pos.board[addr] = Some(pos.friend);
                self.nodes += 1;
                // 棋譜にも付けようぜ☆（＾～＾）
                self.history[self.pieces_num] = addr as u8;
                self.pieces_num += 1;

                // 深い方に潜ってるときの読み筋☆（＾～＾）いわゆる前向き☆（＾～＾）
                // 勝ったかどうか判定しようぜ☆（＾～＾）？
                if pos.is_win() {
                    // 勝ったなら☆（＾～＾）
                    self.info_leaf(
                        pos,
                        addr,
                        if pos.friend == self.root_friend {
                            "win".to_string()
                        } else {
                            "lose".to_string()
                        },
                        Some("Hooray!".to_string()),
                    );

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    self.pieces_num -= 1;
                    pos.board[addr] = None;

                    // 投了図では メートは -0 ☆（＾～＾）
                    let mate = -0;

                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    self.info_backward(pos, addr, Some(mate), None);

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), Some(mate));
                } else if MAX_MOVES - self.root_move_num < self.pieces_num {
                    // 勝っていなくて、深さ上限に達したら、〇×ゲームでは 他に置く場所もないから引き分け確定だぜ☆（＾～＾）
                    self.info_leaf(pos, addr, "draw".to_string(), Some("Fmmm.".to_string()));
                    // 次の枝の探索へ☆（＾～＾）
                    self.pieces_num -= 1;
                    pos.board[addr] = None;
                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    self.info_backward(pos, addr, None, None);

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), None);
                } else {
                    // 勝ってないなら☆（＾～＾）
                    self.info_forward(pos, addr, cur_mate, Some("Search.".to_string()));
                }

                pos.add_move(addr as u8);
                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, opponent_mate) = self.node(pos);
                // メートは絶対値が増えて、符号を反転しろだぜ☆（＾～＾）
                let friend_mate = if let Some(opponent_mate) = opponent_mate {
                    if opponent_mate == 0 {
                        // 次が 1 の都合上、 0 は負数としてくれだぜ☆（＾～＾）
                        Some(1)
                    } else {
                        Some((-opponent_mate / opponent_mate) * (opponent_mate.abs() + 1))
                    }
                } else {
                    None
                };
                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.change_phase();
                pos.remove_move();
                self.pieces_num -= 1;
                pos.board[addr] = None;

                // 子枝のメートを見て、採用するか棄却するか選ぶぜ☆（＾～＾）
                let (update, comment) = if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    if let Some(friend_mate) = friend_mate {
                        if friend_mate.abs() % 2 == 1 {
                            // メートが奇数なら手番の勝ちだぜ☆（＾～＾）
                            (true, Some("At first, mate is good.".to_string()))
                        } else {
                            // 負け☆（＾～＾）合法手だが、こんな手は採用してはいけないぜ☆（＾～＾）
                            (
                                false,
                                Some("At first, Damn! I don't choose the square!".to_string()),
                            )
                        }
                    } else {
                        (true, Some("At first, draw is better.".to_string()))
                    }
                } else {
                    if let Some(cur_mate) = cur_mate {
                        // メート0 は負数（負け）扱いで☆（＾～＾）
                        if cur_mate < 0 {
                            // 今までの手は、メート食らう手のとき☆（／＿＼）
                            if let Some(friend_mate) = friend_mate {
                                if friend_mate.abs() % 2 == 1 {
                                    // メートが奇数なら手番の勝ちだぜ☆（＾～＾）
                                    // 今まで メートされる手ばかりだったが、メートできる手を見つけたぜ☆（＾～＾）
                                    // メート食らってたのを、メートかけるんだから、すごい良い手だぜ☆（＾～＾）！更新するぜ☆（＾～＾）
                                    (
                                        true,
                                        Some(
                                            "Alright! Cross-counter checkmate is great!"
                                                .to_string(),
                                        ),
                                    )
                                } else {
                                    if cur_mate.abs() < friend_mate.abs() {
                                        // 今まで メートされる手ばかりだったが、手数を伸ばす手を見つけたぜ☆（＾～＾）
                                        (true, Some("Delayed the bad is better.".to_string()))
                                    } else {
                                        // 今もメートされていて、いいとこなし☆（＾～＾）
                                        (false, Some("Oh. There was no good point.".to_string()))
                                    }
                                }
                            } else {
                                // 今まで メートされる手ばかりだったが、引き分けにできるぜ☆（＾～＾）！
                                (true, Some("I was relieved. Draw is better.".to_string()))
                            }
                        } else {
                            // 今までの手は、メート掛ける手のとき☆（＾ｑ＾）
                            if let Some(friend_mate) = friend_mate {
                                if friend_mate.abs() % 2 == 1 {
                                    // メートが奇数なら手番の勝ちだぜ☆（＾～＾）
                                    if friend_mate.abs() < cur_mate.abs() {
                                        // より短手数のメートをかける手を見つけてたら、更新するぜ☆（＾～＾）
                                        (
                                            true,
                                            Some(
                                                "It's good! Shorter checkmate is better."
                                                    .to_string(),
                                            ),
                                        )
                                    } else {
                                        (false, Some("No, good.".to_string()))
                                    }
                                } else {
                                    // メートが長手数になるとか、メートが外れるとか☆（＾～＾）
                                    (
                                        false,
                                        Some("I was disappointed. It was good before.".to_string()),
                                    )
                                }
                            } else {
                                // メートしてたのに引き分けになるなんて☆（＾～＾）
                                (false, Some("What a hell! No more wins!".to_string()))
                            }
                        }
                    } else {
                        // 初回か、または 今まで引き分けで。
                        if let Some(friend_mate) = friend_mate {
                            if friend_mate.abs() % 2 == 1 {
                                // メートが奇数なら手番が勝ってるぜ☆（＾～＾）
                                (true, Some("Thumbs up! I found a mate!".to_string()))
                            } else {
                                // 負けてるなんて☆（＾～＾）
                                (false, Some("Damn!".to_string()))
                            }
                        } else {
                            // 勝ち負けがずっと見えてないなら☆（＾～＾）
                            (false, Some("It's ok. Even.".to_string()))
                        }
                    }
                };

                // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                if update {
                    // 更新することは確定☆（＾～＾）
                    best_addr = Some(addr as u8);
                    cur_mate = friend_mate;
                }
                self.info_backward(pos, addr, cur_mate /*friend_mate*/, comment);
            }
        }

        // メートは絶対値が増えて、符号を反転しろだぜ☆（＾～＾）
        (best_addr, cur_mate)
    }
}
/*
/// 勝ち筋☆（＾～＾）
pub struct BestPv {
    pub best_moves: [u8; MOVES_LEN],
    pub best_moves_len: usize,
}
impl Default for BestPv {
    fn default() -> Self {
        BestPv {
            best_moves: [0; MOVES_LEN],
            best_moves_len: 0,
        }
    }
}
*/
