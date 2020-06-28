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

        for addr in 1..9 {
            // 空きマスがあれば
            if let None = pos.board[addr] {
                // とりあえず置いてみようぜ☆（＾～＾）
                pos.board[addr] = Some(pos.friend);
                // 棋譜にも付けようぜ☆（＾～＾）
                self.moves[self.depth] = addr as u8;
                self.depth += 1;

                // 深い方に潜ってるときの読み筋☆（＾～＾）いわゆる前向き☆（＾～＾）
                println!("info pv {: <17} |", self.pv());

                // 勝ったかどうか判定しようぜ☆（＾～＾）？
                if pos.is_win() {
                    // 勝ったなら☆（＾～＾）
                    println!("info .. {: <17} | {} win LEAF", "", pos.friend);

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    pos.board[addr] = None;
                    self.depth -= 1;

                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    println!("info pv {: <17} |", self.pv());

                    // 探索終了だぜ☆（＾～＾）
                    return (
                        Some(addr as u8),
                        // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                        Some(-1),
                    );
                }

                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, friend_mate) = self.node(pos);

                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.board[addr] = None;
                self.depth -= 1;
                pos.change_phase();

                if best_addr == None {
                    // 置ける場所があれば必ず選ばなければならないから、最初に見つけた置ける場所をひとまず調べるぜ☆（＾～＾）
                    best_addr = Some(addr as u8);
                    shortest_mate = friend_mate;
                    println!(
                        "info .. {: <17} | {} {}addr{} GETTING-FIRST{}",
                        "",
                        pos.friend,
                        addr,
                        if let Some(s_mate) = shortest_mate {
                            format!(" mate {}", s_mate)
                        } else {
                            "".to_string()
                        },
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
                } else if let Some(f_mate) = friend_mate {
                    const S_MATE_MAX: i8 = 127;

                    let s_mate = if let Some(s_mate) = shortest_mate {
                        s_mate
                    } else {
                        S_MATE_MAX
                    };

                    if f_mate.abs() < s_mate.abs() {
                        // より短手数のメートを見つけていたら、更新だぜ☆（＾～＾）
                        if 0 < f_mate {
                            best_addr = Some(addr as u8);
                            shortest_mate = friend_mate;
                            println!(
                                "info pv {: <17} | {} good{}addr{} UPDATE",
                                self.pv(),
                                pos.friend,
                                addr,
                                if s_mate == S_MATE_MAX {
                                    "".to_string()
                                } else {
                                    format!(" mate {}", s_mate)
                                },
                            );
                        } else {
                            println!(
                                "info pv {: <17} | {} bad{}addr{} # I do not choose.",
                                self.pv(),
                                pos.friend,
                                addr,
                                if s_mate == S_MATE_MAX {
                                    "".to_string()
                                } else {
                                    format!(" mate {}", s_mate)
                                },
                            );
                        }
                    }
                }

                // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                println!("info pv {: <17} |", self.pv());
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
                    // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                    Some(-(s_mate + 1))
                } else {
                    // 自分がメートされてるんなら、相手はメートしてるんだぜ☆（＾～＾）
                    Some(-(s_mate - 1))
                }
            } else {
                None
            },
        )
    }
}
