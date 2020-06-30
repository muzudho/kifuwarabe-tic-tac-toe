use crate::piece::Piece;
use crate::position::{Position, BOARD_LEN, MAX_MOVES, MOVES_LEN};

/// 探索部☆（＾～＾）
pub struct Search {
    /// この探索を始めたのはどっち側か☆（＾～＾）
    root_friend: Piece,
    /// この探索を始めたのは何手目か☆（＾～＾）
    root_move_num: usize,
    /// 現在局面からの、棋譜☆（＾～＾）ややこしいんで [0] は使わないぜ☆（＾～＾）
    moves: [u8; 10],
    /// 現在局面からの読みの深さ☆（＾～＾）1スタート☆（＾～＾）
    depth: usize,
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
                println!("info pv O X O X O X O X O");
            }
            Piece::Cross => {
                println!("info pv X O X O X O X O X");
            }
        }
        self.node(pos)
    }

    // 後ろ向き探索のときの表示だぜ☆（＾～＾）
    fn backward_str(
        &self,
        pv: String,
        friend: String,
        addr: usize,
        cur_mate: Option<i8>,
        child_mate: Option<i8>,
    ) -> String {
        format!(
            "pv {: <17} | <- from depth {} | {} [{}] | {} |{}",
            pv,
            self.depth,
            friend,
            addr,
            if let Some(child_mate) = child_mate {
                format!("mate {: >2}", child_mate)
            } else {
                "       ".to_string()
            },
            if let Some(cur_mate) = cur_mate {
                if let Some(child_mate) = child_mate {
                    if child_mate.abs() < cur_mate.abs() {
                        // より短手数のメートを見つけたら。
                        format!(" Faster mate rather than {}.", cur_mate)
                    } else {
                        // 長手数のメートは要らないぜ☆（＾～＾）
                        format!(" Ignore mate. It's longer than(or equals) {}.", cur_mate)
                    }
                } else {
                    format!(" Not change mate {}.", cur_mate)
                }
            } else {
                if let Some(_) = child_mate {
                    // 新しく見つけたメート。
                    format!(" Found mate.")
                } else {
                    "".to_string()
                }
            },
        )
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
                    println!(
                        "info pv {: <17} | .       depth {} | {} [{}] | {:4}    |",
                        self.pv(),
                        self.depth - 1,
                        if pos.friend == self.root_friend {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        addr,
                        if pos.friend == self.root_friend {
                            "win".to_string()
                        } else {
                            "lose".to_string()
                        },
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
                    println!(
                        "info pv {: <17} | <- from depth {} | {} [{}] | mate {: >2} |",
                        self.pv(),
                        self.depth,
                        if pos.friend == self.root_friend {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        addr,
                        mate
                    );

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), Some(mate));
                } else if MAX_MOVES - self.root_move_num + 1 < self.depth {
                    // 勝っていなくて、深さ上限に達したら、〇×ゲームでは 他に置く場所もないから引き分け確定だぜ☆（＾～＾）
                    println!(
                        "info pv {: <17} | .       depth {} | {} [{}] |{}| Draw.",
                        self.pv(),
                        self.depth - 1,
                        if pos.friend == self.root_friend {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        addr,
                        if let Some(cur_mate) = cur_mate {
                            format!("     ({: >2})", cur_mate)
                        } else {
                            "         ".to_string()
                        },
                    );
                    // 次の枝の探索へ☆（＾～＾）
                    self.depth -= 1;
                    pos.board[addr] = None;
                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    println!(
                        "info pv {: <17} | <- from depth {} | {} [{}] |         |",
                        self.pv(),
                        self.depth,
                        if pos.friend == self.root_friend {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        addr
                    );

                    continue;
                } else {
                    // 勝ってないなら☆（＾～＾）
                    println!(
                        "info pv {: <17} | ->   to depth {} | {} [{}] |{}|",
                        self.pv(),
                        self.depth,
                        if pos.friend == self.root_friend {
                            "+".to_string()
                        } else {
                            "-".to_string()
                        },
                        addr,
                        if let Some(cur_mate) = cur_mate {
                            format!("     ({: >2})", cur_mate)
                        } else {
                            "         ".to_string()
                        }
                    );
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

                enum UpdateReadon {
                    /// 置ける場所があれば、必ず置かなければならないから、最初の１個はとりあえず選ぶぜ☆（＾～＾）
                    GettingFirst(String),
                    /// 今までに見つけた手より良い手なら、更新だぜ☆（＾～＾）
                    Better(String),
                }
                let update_reason = if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    if let Some(child_mate) = child_mate {
                        if (0 < child_mate && pos.friend == self.root_friend)
                            || (child_mate <= 0 && pos.friend != self.root_friend)
                        {
                            // （メートが正の数で、探索している方のターン）または、（メートが０または負数で、探索していない方のターン）なら、そいつの勝ちだぜ☆（＾～＾）
                            Some(UpdateReadon::GettingFirst("Good.".to_string()))
                        } else {
                            // 負け☆（＾～＾）
                            Some(UpdateReadon::GettingFirst("Bad.".to_string()))
                        }
                    } else {
                        Some(UpdateReadon::GettingFirst("Draw.".to_string()))
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
                                    Some(UpdateReadon::Better(
                                        "Cross-counter checkmate.".to_string(),
                                    ))
                                } else {
                                    if cur_mate.abs() < child_mate.abs() {
                                        // 今まで メートされる手ばかりだったが、手数を伸ばす手を見つけたぜ☆（＾～＾）
                                        Some(UpdateReadon::Better("Delayed the bad.".to_string()))
                                    } else {
                                        None
                                    }
                                }
                            } else {
                                // 今まで メートされる手ばかりだったが、引き分けにできるぜ☆（＾～＾）！
                                Some(UpdateReadon::Better("Found the draw.".to_string()))
                            }
                        } else {
                            // 今までの手は、メート掛ける手のとき☆（＾ｑ＾）
                            if let Some(cihld_mate) = child_mate {
                                if 0 < cihld_mate && cihld_mate.abs() < cur_mate.abs() {
                                    // より短手数のメートをかける手を見つけてたら、更新するぜ☆（＾～＾）
                                    Some(UpdateReadon::Better("Shorter checkmate.".to_string()))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    } else {
                        // 今まで、引き分けの手だけ見つけているケースで。
                        if let Some(child_mate) = child_mate {
                            // メート0 は負数（負け）扱いで☆（＾～＾）
                            if 0 < child_mate {
                                // こっちからメートする手を見つけたぜ☆（＾～＾）
                                Some(UpdateReadon::Better("Found checkmate.".to_string()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                };

                // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                if let Some(u_reason) = update_reason {
                    // 更新することは確定☆（＾～＾）
                    best_addr = Some(addr as u8);
                    cur_mate = child_mate;

                    match u_reason {
                        UpdateReadon::GettingFirst(comment) => {
                            println!(
                                "info {} UPDATE at first.{}",
                                self.backward_str(
                                    self.pv(),
                                    if pos.friend == self.root_friend {
                                        "+".to_string()
                                    } else {
                                        "-".to_string()
                                    },
                                    addr,
                                    cur_mate,
                                    child_mate
                                ),
                                if comment != "" {
                                    format!(" # {}", comment)
                                } else {
                                    comment
                                }
                            );
                        }
                        UpdateReadon::Better(comment) => {
                            // 短手数のメートを良い方へ更新したら、更新するぜ☆（＾～＾）
                            println!(
                                "info {} UPDATE # {}",
                                self.backward_str(
                                    self.pv(),
                                    if pos.friend == self.root_friend {
                                        "+".to_string()
                                    } else {
                                        "-".to_string()
                                    },
                                    addr,
                                    cur_mate,
                                    child_mate,
                                ),
                                comment
                            );
                        }
                    }
                } else {
                    // 更新がないとき☆（＾～＾）
                    println!(
                        "info {}",
                        self.backward_str(
                            self.pv(),
                            if pos.friend == self.root_friend {
                                "+".to_string()
                            } else {
                                "-".to_string()
                            },
                            addr,
                            cur_mate,
                            child_mate
                        ),
                    );
                }
            }
        }

        /*
        if let None = best_addr {
            // 置くところが無かったのなら☆（＾～＾）
            println!("info .. {: <17} |    Found draw.", "");
        }
        */

        (best_addr, cur_mate)
    }
}
