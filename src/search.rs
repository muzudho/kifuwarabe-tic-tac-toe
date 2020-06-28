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
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, i8) {
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

    fn node(&mut self, pos: &mut Position) -> (Option<u8>, i8) {
        let mut best_addr = None;
        // -1,2,-3,4... のように 負の奇数と、正の偶数が交互に出てくるぜ☆（＾～＾）
        let mut shortest_mate = 127i8;

        for addr in 1..9 {
            // 空きマスがあれば
            if let None = pos.board[addr] {
                // とりあえず置いてみようぜ☆（＾～＾）
                pos.board[addr] = Some(pos.friend);
                // 棋譜にも付けようぜ☆（＾～＾）
                self.moves[self.depth] = addr as u8;
                self.depth += 1;

                // 勝ったかどうか判定しようぜ☆（＾～＾）？
                if pos.is_win() {
                    // 勝ったなら☆（＾～＾）
                    println!("info pv {: <17} | {} win", self.pv(), pos.friend);

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    pos.board[addr] = None;
                    self.depth -= 1;
                    // 探索終了だぜ☆（＾～＾）
                    return (
                        Some(addr as u8),
                        // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                        -1,
                    );
                }

                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, friend_mate) = self.node(pos);

                // 相手が置いたところを戻そうぜ☆（＾～＾）？
                pos.board[addr] = None;
                self.depth -= 1;
                pos.change_phase();

                if best_addr == None && -1 < friend_mate {
                    // 最初に見つけた手でメートを掛けられていなければ、とりあえず採用☆（＾～＾）
                    best_addr = Some(addr as u8);
                    shortest_mate = friend_mate;
                    println!(
                        "info pv {: <17} | {} first-addr {}{} UPDATE",
                        self.pv(),
                        pos.friend,
                        addr,
                        if shortest_mate == 0 || shortest_mate == 127 {
                            "".to_string()
                        } else {
                            format!(" mate {}", shortest_mate)
                        }
                    );
                } else if friend_mate.abs() <= shortest_mate.abs() {
                    // 同じ手数、または、より短手数のメートを見つけていたら、更新だぜ☆（＾～＾）
                    if 0 < friend_mate {
                        best_addr = Some(addr as u8);
                        shortest_mate = friend_mate;
                        println!(
                            "info pv {: <17} | {} good-addr {}{} UPDATE",
                            self.pv(),
                            pos.friend,
                            addr,
                            if shortest_mate == 0 || shortest_mate == 127 {
                                "".to_string()
                            } else {
                                format!(" mate {}", shortest_mate)
                            }
                        );
                    } else {
                        println!(
                            "info pv {: <17} | {} bad-addr {}{}",
                            self.pv(),
                            pos.friend,
                            addr,
                            if shortest_mate == 0 || shortest_mate == 127 {
                                "".to_string()
                            } else {
                                format!(" mate {}", shortest_mate)
                            }
                        );
                    }
                }
            }
        }

        if let None = best_addr {
            // 置くところが無かったのなら☆（＾～＾）
            println!("info pv {: <17} | draw", self.pv());
        }

        (
            best_addr,
            if shortest_mate == 0 || shortest_mate == 127 {
                0
            } else if 0 < shortest_mate {
                // 自分がメートしたら、相手はメートされてるんだぜ☆（＾～＾）
                -(shortest_mate + 1)
            } else {
                // 自分がメートされてるんなら、相手はメートしてるんだぜ☆（＾～＾）
                -(shortest_mate - 1)
            },
        )
    }
}
