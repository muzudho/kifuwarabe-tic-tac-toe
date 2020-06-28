use crate::piece::Piece;
use crate::position::Position;

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
        // 0,-1,2,-3,4... のように、0を除くと、 負の奇数と、正の偶数が交互に出てくるぜ☆（＾～＾）
        let mut shortest_mate: Option<i8> = None;

        for addr in 1..10 {
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
                    println!("info pv {: <17} | <- MATE_ZERO", self.pv());

                    // 探索終了だぜ☆（＾～＾）
                    return (
                        Some(addr as u8),
                        // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                        Some(-1),
                    );
                } else {
                    println!("info pv {: <17} | ->", self.pv());
                }

                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, friend_mate) = self.node(pos);

                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.board[addr] = None;
                self.depth -= 1;
                pos.change_phase();

                // 後ろ向き探索のときの表示だぜ☆（＾～＾）
                fn backward_str(
                    pv: String,
                    friend: Piece,
                    addr: usize,
                    s_mate: Option<i8>,
                    f_mate: Option<i8>,
                ) -> String {
                    format!(
                        "pv {: <17} | <- {} {}addr{}{}",
                        pv,
                        friend,
                        addr,
                        if let Some(mate) = s_mate {
                            format!(" s_mate{}", mate)
                        } else {
                            "".to_string()
                        },
                        if let Some(mate) = f_mate {
                            format!(" f_mate{}", mate)
                        } else {
                            "".to_string()
                        }
                    )
                }

                enum UpdateReadon {
                    /// 置ける場所があれば、必ず置かなければならないから、最初の１個はとりあえず選ぶぜ☆（＾～＾）
                    GettingFirst,
                    /// 短手数でメートかけれるなら更新しないとな☆（＾～＾）
                    ShorterGoodMate,
                    /// メートされるケースで、手数を伸ばす手を見つけたぜ☆（＾～＾）
                    LongerBadMate,
                    /// メート食らってたのを、メートかけるんだから、すごい良い手だぜ☆（＾～＾）！更新するぜ☆（＾～＾）
                    GoodCounterMate,
                    /// メートを食らってたのを、引き分けにできるぜ☆（＾～＾）！
                    GoodDraw,
                }
                let update_reason = if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    Some(UpdateReadon::GettingFirst)
                } else {
                    if let Some(s_mate) = shortest_mate {
                        if s_mate < 0 {
                            // 今までの手は、メート食らう手のとき☆（／＿＼）
                            if let Some(f_mate) = friend_mate {
                                if 0 < f_mate {
                                    // 今まで メートされる手ばかりだったが、メートできる手を見つけたぜ☆（＾～＾）
                                    Some(UpdateReadon::GoodCounterMate)
                                } else if f_mate < 0 {
                                    if s_mate.abs() < f_mate.abs() {
                                        // 今まで メートされる手ばかりだったが、手数を伸ばす手を見つけたぜ☆（＾～＾）
                                        Some(UpdateReadon::LongerBadMate)
                                    } else {
                                        None
                                    }
                                } else {
                                    panic!("メートは0にならないはずだぜ☆（＾～＾）！");
                                }
                            } else {
                                // 今まで メートされる手ばかりだったが、引き分けにできるぜ☆（＾～＾）！
                                Some(UpdateReadon::GoodDraw)
                            }
                        } else if s_mate == 0 {
                            // 今までの手は、引き分けのとき☆（ー＿ー）
                            if let Some(f_mate) = friend_mate {
                                if 0 < f_mate {
                                    // 今まで 引き分けの手ばかりだったが、メートできる手を見つけたぜ☆（＾～＾）
                                    Some(UpdateReadon::GoodCounterMate)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            // 今までの手は、メート掛ける手のとき☆（＾ｑ＾）
                            if let Some(f_mate) = friend_mate {
                                if 0 < f_mate && f_mate.abs() < s_mate.abs() {
                                    // より短手数のメートをかける手を見つけてたら、更新するぜ☆（＾～＾）
                                    Some(UpdateReadon::ShorterGoodMate)
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
                            if 0 < f_mate {
                                // こっちからメートする手を見つけたぜ☆（＾～＾）
                                Some(UpdateReadon::ShorterGoodMate)
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
                        UpdateReadon::GettingFirst => {
                            println!(
                                "info {} GETTING-FIRST{}",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
                                ),
                                if let Some(s_mate) = shortest_mate {
                                    if 0 < s_mate {
                                        format!(" # So good!")
                                    } else if s_mate < 0 {
                                        format!(" # So bad!")
                                    } else {
                                        "".to_string()
                                    }
                                } else {
                                    "".to_string()
                                },
                            );
                        }
                        UpdateReadon::GoodCounterMate => {
                            // メート食らってたのを、メートかけるんだから、すごい良い手だぜ☆（＾～＾）！更新するぜ☆（＾～＾）
                            println!(
                                "info {} UPDATE # Excellent!",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
                                ),
                            );
                        }
                        UpdateReadon::ShorterGoodMate => {
                            // 短手数のメートを良い方へ更新したら、更新するぜ☆（＾～＾）
                            println!(
                                "info {} UPDATE # Great!",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
                                ),
                            );
                        }
                        UpdateReadon::LongerBadMate => {
                            // メートされるケースで、手数を伸ばす手を見つけたぜ☆（＾～＾）
                            println!(
                                "info {} UPDATE # Increase the number of steps.",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
                                ),
                            );
                        }
                        UpdateReadon::GoodDraw => {
                            // メートを食らってたのを、引き分けにできるぜ☆（＾～＾）！
                            println!(
                                "info {} UPDATE # Good draw.",
                                backward_str(
                                    self.pv(),
                                    pos.friend,
                                    addr,
                                    shortest_mate,
                                    friend_mate
                                ),
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
            println!("info .. {: <17} | draw", "");
        }

        (
            best_addr,
            if let Some(s_mate) = shortest_mate {
                if 0 < s_mate {
                    let x = -(s_mate + 1);
                    println!("s_mate={} だったんで {} にした。", s_mate, x);
                    // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                    Some(x)
                } else if s_mate < 0 {
                    let x = -(s_mate - 1);
                    println!("s_mate={} だったんで {} にした。", s_mate, x);
                    // 自分がメートされてるんなら、相手はメートしてるんだぜ☆（＾～＾）
                    Some(x)
                } else {
                    panic!("ここは通らないはずだぜ☆（＾～＾）！");
                }
            } else {
                println!("drawだよな☆（＾～＾）");
                None
            },
        )
    }
}
