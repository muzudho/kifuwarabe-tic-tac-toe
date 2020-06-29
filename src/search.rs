use crate::piece::Piece;
use crate::position::Position;
use crate::position::BOARD_LEN;

/// 探索部☆（＾～＾）
pub struct Search {
    /// 現在局面からの、棋譜☆（＾～＾）
    pub moves: [u8; 10],
    /// 現在局面からの読みの深さ☆（＾～＾）
    pub depth: usize,
}
impl Default for Search {
    fn default() -> Self {
        Search {
            moves: [0; 10],
            depth: 0,
        }
    }
}
impl Search {
    /// Principal variation. 今読んでる読み筋☆（＾～＾）
    pub fn pv(&self) -> String {
        let mut pv = String::new();
        for d in 0..self.depth {
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

    fn node(&mut self, pos: &mut Position) -> (Option<u8>, Option<i8>) {
        let mut best_addr = None;
        // 0,1,-2,3,-4... のように、0を除くと、 正の奇数（勝ち）と、負の偶数（負け）が交互に出てくるぜ☆（＾～＾）
        let mut shortest_mate: Option<i8> = None;

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
                    println!("info pv {: <17} | -> {} win", self.pv(), pos.friend);

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    pos.board[addr] = None;
                    self.depth -= 1;

                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    println!("info pv {: <17} | <- Found lion-catch.", self.pv());

                    // 探索終了だぜ☆（＾～＾）
                    return (
                        Some(addr as u8),
                        // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                        Some(-2),
                    );
                } else {
                    println!("info pv {: <17} | ->", self.pv());
                }

                pos.add_move(addr as u8);
                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, friend_mate) = self.node(pos);

                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.change_phase();
                pos.remove_move();
                self.depth -= 1;
                pos.board[addr] = None;

                // 後ろ向き探索のときの表示だぜ☆（＾～＾）
                fn backward_str(
                    pv: String,
                    friend: Piece,
                    addr: usize,
                    s_mate: Option<i8>,
                    f_mate: Option<i8>,
                ) -> String {
                    format!(
                        "pv {: <17} | <- {} [{}]{}",
                        pv,
                        friend,
                        addr,
                        if let Some(s_mate) = s_mate {
                            if let Some(f_mate) = f_mate {
                                if f_mate.abs() < s_mate.abs() {
                                    format!(" Faster mate {} rather than {}.", f_mate, s_mate)
                                } else if f_mate.abs() == s_mate.abs() {
                                    // 最初に見つけたメートか、既に見つけているメートと手数が同じかは区別できない。
                                    format!(" Found mate {}.", f_mate)
                                } else {
                                    format!(" Ignore mate {} rather than {}.", f_mate, s_mate)
                                }
                            } else {
                                format!(" Not change mate {}.", s_mate)
                            }
                        } else {
                            if let Some(f_mate) = f_mate {
                                // 新しく見つけたメート。
                                format!(" Found mate {}.", f_mate)
                            } else {
                                "".to_string()
                            }
                        },
                    )
                }

                enum UpdateReadon {
                    /// 置ける場所があれば、必ず置かなければならないから、最初の１個はとりあえず選ぶぜ☆（＾～＾）
                    GettingFirst(String),
                    /// 今までに見つけた手より良い手なら、更新だぜ☆（＾～＾）
                    Better(String),
                }
                let update_reason = if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    if let Some(s_mate) = shortest_mate {
                        // メート0 は負数（負け）扱いで☆（＾～＾）
                        if 0 < s_mate {
                            Some(UpdateReadon::GettingFirst("Good.".to_string()))
                        } else {
                            Some(UpdateReadon::GettingFirst("Bad.".to_string()))
                        }
                    } else {
                        Some(UpdateReadon::GettingFirst("".to_string()))
                    }
                } else {
                    if let Some(s_mate) = shortest_mate {
                        // メート0 は負数（負け）扱いで☆（＾～＾）
                        if s_mate < 0 {
                            // 今までの手は、メート食らう手のとき☆（／＿＼）
                            if let Some(f_mate) = friend_mate {
                                // メート0 は負数（負け）扱いで☆（＾～＾）
                                if 0 < f_mate {
                                    // 今まで メートされる手ばかりだったが、メートできる手を見つけたぜ☆（＾～＾）
                                    // メート食らってたのを、メートかけるんだから、すごい良い手だぜ☆（＾～＾）！更新するぜ☆（＾～＾）
                                    Some(UpdateReadon::Better(
                                        "Cross-counter checkmate.".to_string(),
                                    ))
                                } else {
                                    if s_mate.abs() < f_mate.abs() {
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
                            if let Some(f_mate) = friend_mate {
                                if 0 < f_mate && f_mate.abs() < s_mate.abs() {
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
                        if let Some(f_mate) = friend_mate {
                            // メート0 は負数（負け）扱いで☆（＾～＾）
                            if 0 < f_mate {
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
                    shortest_mate = friend_mate;

                    match u_reason {
                        UpdateReadon::GettingFirst(comment) => {
                            println!(
                                "info {} At first.{}",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
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
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate,
                                ),
                                comment
                            );
                        }
                    }
                } else {
                    // 更新がないとき☆（＾～＾）
                    println!(
                        "info {}",
                        backward_str(self.pv(), pos.friend, addr, shortest_mate, friend_mate),
                    );
                }
            }
        }

        if let None = best_addr {
            // 置くところが無かったのなら☆（＾～＾）
            println!("info .. {: <17} |    Found draw.", "");
        }

        (
            best_addr,
            if let Some(s_mate) = shortest_mate {
                // メート0 は負数（負け）扱いで☆（＾～＾）
                if 0 < s_mate {
                    let x = -(s_mate + 1);
                    // println!("s_mate={} だったんで {} にした。", s_mate, x);
                    // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                    Some(x)
                } else {
                    let x = -(s_mate - 1);
                    // println!("s_mate={} だったんで {} にした。", s_mate, x);
                    // 自分がメートされてるんなら、相手はメートしてるんだぜ☆（＾～＾）
                    Some(x)
                }
            } else {
                // println!("drawだよな☆（＾～＾）");
                None
            },
        )
    }
}
