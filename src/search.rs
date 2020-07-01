use crate::log::Log;
use crate::piece::Piece;
use crate::position::{Position, BOARD_LEN, MAX_MOVES, MOVES_LEN};

/// 探索部☆（＾～＾）
pub struct Search {
    /// この探索を始めたのはどっち側か☆（＾～＾）
    pub root_friend: Piece,
    /// この探索を始めたのは何手目か☆（＾～＾）
    root_move_num: usize,
    /// 現在局面からの、棋譜☆（＾～＾）ややこしいんで [0] は使わないぜ☆（＾～＾）
    moves: [u8; 10],
    /// 現在局面からの読みの深さ☆（＾～＾）1スタート☆（＾～＾）
    pub depth: usize,
}
impl Search {
    pub fn new(friend: Piece, move_num: usize) -> Self {
        Search {
            root_friend: friend,
            root_move_num: move_num,
            moves: [0; MOVES_LEN],
            depth: 1,
        }
    }
    /// Principal variation. 今読んでる読み筋☆（＾～＾）
    pub fn pv(&self) -> String {
        let mut pv = String::new();
        for d in 1..self.depth {
            pv.push_str(&format!("{} ", self.moves[d]));
        }
        pv.trim_end().to_string()
    }
    /// 最善の番地を返すぜ☆（＾～＾）
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, Option<i8>) {
        match pos.friend {
            Piece::Nought => {
                Log::println("info pv O X O X O X O X O");
            }
            Piece::Cross => {
                Log::println(&format!("info pv X O X O X O X O X"));
            }
        }
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
                // 棋譜にも付けようぜ☆（＾～＾）
                self.moves[self.depth] = addr as u8;
                self.depth += 1;

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
                    self.depth -= 1;
                    pos.board[addr] = None;

                    // メートが出るぜ☆（＾～＾）
                    // 偶数手番は相手の勝ちなんで負数に、奇数手番は自分の勝ちなんで正の数にしろだぜ☆（＾～＾）
                    let mate = if self.depth % 2 == 0 {
                        -(self.depth as i8)
                    } else {
                        self.depth as i8
                    };

                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    self.info_backward(pos, addr, Some(mate), None);

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), Some(mate));
                } else if MAX_MOVES - self.root_move_num + 1 < self.depth {
                    // 勝っていなくて、深さ上限に達したら、〇×ゲームでは 他に置く場所もないから引き分け確定だぜ☆（＾～＾）
                    self.info_leaf(pos, addr, "draw".to_string(), Some("Fmmm.".to_string()));
                    // 次の枝の探索へ☆（＾～＾）
                    self.depth -= 1;
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
                let (_opponent_address, child_mate) = self.node(pos);

                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.change_phase();
                pos.remove_move();
                self.depth -= 1;
                pos.board[addr] = None;

                // 子枝のメートを見て、採用するか棄却するか選ぶぜ☆（＾～＾）
                let (update, comment) = if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    if let Some(child_mate) = child_mate {
                        if (0 < child_mate && pos.friend == self.root_friend)
                            || (child_mate <= 0 && pos.friend != self.root_friend)
                        {
                            // （メートが正の数で、探索している方のターン）または、（メートが０または負数で、探索していない方のターン）なら、そいつの勝ちだぜ☆（＾～＾）
                            (true, Some("At first, mate is good.".to_string()))
                        } else {
                            // 負け☆（＾～＾）合法手だが、こんな手は採用してはいけないぜ☆（＾～＾）
                            (false, Some("Damn! I don't choose the square!".to_string()))
                        }
                    } else {
                        (true, Some("At first, draw is good.".to_string()))
                    }
                } else {
                    if let Some(cur_mate) = cur_mate {
                        // メート0 は負数（負け）扱いで☆（＾～＾）
                        if cur_mate < 0 {
                            // 今までの手は、メート食らう手のとき☆（／＿＼）
                            if let Some(child_mate) = child_mate {
                                // メート0 は負数（負け）扱いで☆（＾～＾）
                                if 0 < child_mate {
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
                                    if cur_mate.abs() < child_mate.abs() {
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
                            if let Some(cihld_mate) = child_mate {
                                if 0 < cihld_mate && cihld_mate.abs() < cur_mate.abs() {
                                    // より短手数のメートをかける手を見つけてたら、更新するぜ☆（＾～＾）
                                    (
                                        true,
                                        Some("It's good! Shorter checkmate is better.".to_string()),
                                    )
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
                        // 今まで、引き分けの手だけ見つけているケースで。
                        if let Some(child_mate) = child_mate {
                            // メート0 は負数（負け）扱いで☆（＾～＾）
                            if 0 < child_mate {
                                // こっちからメートする手を見つけたぜ☆（＾～＾）
                                (true, Some("Thumbs up! I found a mate!".to_string()))
                            } else {
                                // 引き分けだったのにメートになるなんて☆（＾～＾）
                                (false, Some("I messed up! I lost the draw!".to_string()))
                            }
                        } else {
                            // 引き分けが変わってなければ☆（＾～＾）
                            (false, Some("Ok. It was a draw from before.".to_string()))
                        }
                    }
                };

                // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                if update {
                    // 更新することは確定☆（＾～＾）
                    best_addr = Some(addr as u8);
                    cur_mate = child_mate;
                }
                self.info_backward(pos, addr, child_mate, comment);
            }
        }

        (best_addr, cur_mate)
    }
}
