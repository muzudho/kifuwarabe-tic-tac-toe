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
        // もう全部置いていて、置き場所がないときだぜ☆（＾～＾）
        self.node(pos)
    }

    fn node(&mut self, pos: &mut Position) -> (Option<u8>, i8) {
        let mut best_addr = None;
        let mut best_mate = 0;

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
                    println!("info win {} pv {}", pos.friend, self.pv());

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    pos.board[addr] = None;
                    self.depth -= 1;
                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), 1);
                }

                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (address, mut mate) = self.node(pos);
                if 0 < mate {
                    // 相手がメートしたら、こっちはメートされてるんだぜ☆（＾～＾）
                    mate = -mate - 1;
                } else if mate < 0 {
                    // 相手がメートされてるんなら、こっちはメートしてるんだぜ☆（＾～＾）
                    mate = -mate + 1;
                }

                if best_addr == None || mate < best_mate {
                    // 最初に見つけた手か、より短手数のメートを見つけていたら、更新だぜ☆（＾～＾）
                    best_addr = address;
                    best_mate = mate;
                }

                // 置いたところを戻そうぜ☆（＾～＾）？
                pos.board[addr] = None;
                self.depth -= 1;
                pos.change_phase();
            }
        }

        (best_addr, best_mate)
    }
}
