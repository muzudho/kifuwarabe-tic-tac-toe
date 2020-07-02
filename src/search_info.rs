use crate::log::Log;
use crate::piece::Piece;
use crate::position::Position;
use crate::position::SQUARES_NUM;
use crate::search::GameResult;
use crate::search::Search;

impl Search {
    pub fn info_header(&self, pos: &mut Position) {
        match pos.friend {
            Piece::Nought => {
                Log::println("info nps ...... nodes ...... pv O X O X O X O X O");
            }
            Piece::Cross => {
                Log::println(&format!(
                    "info nps ...... nodes ...... pv X O X O X O X O X"
                ));
            }
        }
    }

    /// 前向き探索中だぜ☆（＾～＾）
    pub fn info_forward(&self, pos: &mut Position, addr: usize, comment: Option<String>) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | ->   to {} |       |      |{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < self.pieces_num + 1 {
                "none    ".to_string()
            } else {
                format!("height {}", self.pieces_num + 1)
            },
            if let Some(comment) = comment {
                format!(
                    " {} \"{}\"",
                    if pos.friend == self.root_friend {
                        "+".to_string()
                    } else {
                        "-".to_string()
                    },
                    comment
                )
            } else {
                "".to_string()
            },
        ))
    }
    /// 葉だぜ☆（＾～＾）
    pub fn info_leaf(
        &self,
        pos: &mut Position,
        addr: usize,
        result: GameResult,
        comment: Option<String>,
    ) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | .       {} |       |{}|{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < self.pieces_num {
                "none    ".to_string()
            } else {
                format!("height {}", self.pieces_num)
            },
            match result {
                GameResult::Win => " win  ".to_string(),
                GameResult::Draw => " draw ".to_string(),
                GameResult::Lose => " lose ".to_string(),
            },
            if let Some(comment) = comment {
                format!(
                    " {} \"{}\"",
                    if pos.friend == self.root_friend {
                        "+".to_string()
                    } else {
                        "-".to_string()
                    },
                    comment
                )
            } else {
                "".to_string()
            },
        ));
    }
    /// 後ろ向き探索のときの表示だぜ☆（＾～＾）
    pub fn info_backward(
        &self,
        pos: &mut Position,
        addr: usize,
        result: GameResult,
        comment: Option<String>,
    ) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} |       | <- from {} | {} [{}] |{}|{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if SQUARES_NUM < self.pieces_num + 1 {
                "none    ".to_string()
            } else {
                format!("height {}", self.pieces_num + 1)
            },
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            match result {
                GameResult::Win => " win  ".to_string(),
                GameResult::Draw => " draw ".to_string(),
                GameResult::Lose => " lose ".to_string(),
            },
            if let Some(comment) = comment {
                format!(
                    " {} \"{}\"",
                    if pos.friend == self.root_friend {
                        "+".to_string()
                    } else {
                        "-".to_string()
                    },
                    comment
                )
            } else {
                "".to_string()
            }
        ));
    }
}
