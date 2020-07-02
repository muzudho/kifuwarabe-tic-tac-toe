use crate::piece::Piece;
use crate::position::{Position, BOARD_LEN, SQUARES_NUM};
use std::time::Instant;

#[derive(Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

/// 探索部☆（＾～＾）
pub struct Search {
    /// この探索を始めたのはどっち側か☆（＾～＾）
    pub root_friend: Piece,
    /// この探索を始めたときに石はいくつ置いてあったか☆（＾～＾）
    root_pieces_num: usize,
    /// 現在局面からの、棋譜☆（＾～＾）
    history: [u8; SQUARES_NUM],
    /// 現在局面から置いた石の数☆（＾～＾）
    pub pieces_num: usize,
    /// 探索したノード数☆（＾～＾）
    pub nodes: u32,
    /// この構造体を生成した時点からストップ・ウォッチを開始するぜ☆（＾～＾）
    stopwatch: Instant,
}
impl Search {
    pub fn new(friend: Piece, root_pieces_num: usize) -> Self {
        Search {
            root_friend: friend,
            root_pieces_num: root_pieces_num,
            history: [0; SQUARES_NUM],
            pieces_num: 0,
            nodes: 0,
            stopwatch: Instant::now(),
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
    pub fn go(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        self.info_header(pos);
        self.node(pos)
    }

    fn node(&mut self, pos: &mut Position) -> (Option<u8>, GameResult) {
        let mut grate_addr = None;
        let mut grate_result = GameResult::Lose;

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
                    self.info_leaf(pos, addr, GameResult::Win, Some("Hooray!".to_string()));

                    // 置いたところを戻そうぜ☆（＾～＾）？
                    self.pieces_num -= 1;
                    pos.board[addr] = None;

                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    self.info_backward(pos, addr, GameResult::Win, None);

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), GameResult::Win);
                } else if SQUARES_NUM <= self.root_pieces_num + self.pieces_num {
                    // 勝っていなくて、深さ上限に達したら、〇×ゲームでは 他に置く場所もないから引き分け確定だぜ☆（＾～＾）
                    self.info_leaf(pos, addr, GameResult::Draw, Some("It's ok.".to_string()));
                    // 次の枝の探索へ☆（＾～＾）
                    self.pieces_num -= 1;
                    pos.board[addr] = None;
                    // 浅い方に浮かんでるときの読み筋☆（＾～＾）いわゆる後ろ向き☆（＾～＾）
                    self.info_backward(pos, addr, GameResult::Draw, None);

                    // 探索終了だぜ☆（＾～＾）
                    return (Some(addr as u8), GameResult::Draw);
                } else {
                    // まだ続きがあるぜ☆（＾～＾）
                    self.info_forward(pos, addr, Some("Search.".to_string()));
                }

                pos.add_move(addr as u8);
                pos.change_phase();

                // 相手の番だぜ☆（＾～＾）
                let (_opponent_address, opponent_game_result) = self.node(pos);

                // 自分が置いたところを戻そうぜ☆（＾～＾）？
                pos.change_phase();
                pos.remove_move();
                self.pieces_num -= 1;
                pos.board[addr] = None;

                match opponent_game_result {
                    GameResult::Lose => {
                        // 相手の負けなら、この手で勝ちだぜ☆（＾～＾）
                        self.info_backward(pos, addr, GameResult::Win, Some("Ok.".to_string()));
                        return (Some(addr as u8), GameResult::Win);
                    }
                    GameResult::Draw => {
                        // 勝ち負けがずっと見えてないなら☆（＾～＾）
                        self.info_backward(pos, addr, GameResult::Draw, Some("Fmmm.".to_string()));
                        match grate_result {
                            GameResult::Lose => {
                                // 更新
                                grate_addr = Some(addr as u8);
                                grate_result = GameResult::Draw;
                            }
                            _ => {}
                        }
                    }
                    GameResult::Win => {
                        // 相手が勝つ手を選んではダメだぜ☆（＾～＾）
                        self.info_backward(
                            pos,
                            addr,
                            GameResult::Lose,
                            Some("Resign.".to_string()),
                        );
                    }
                }
            }
        }

        (grate_addr, grate_result)
    }
}
