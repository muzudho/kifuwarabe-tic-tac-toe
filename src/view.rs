use crate::log::Log;
use crate::position::{Piece, Position, SQUARES_NUM};
use crate::search::{GameResult, Search};
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::position::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}

impl Position {
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!("{}", piece)
        } else {
            "   ".to_string()
        }
    }
    pub fn pos(&self) {
        Log::println(&format!(
            "[Next {} move(s) | Go {}]
",
            self.pieces_num + 1,
            self.friend
        ));
        // 書式を指定したりで、桁がずれるのは仕方ないぜ☆（＾～＾）
        Log::println(&format!(
            "\
+---+---+---+
|{0: ^3}|{1: ^3}|{2: ^3}| マスを選んでください。例 `do 7`
+---+---+---+
|{3: ^3}|{4: ^3}|{5: ^3}|    7 8 9
+---+---+---+    4 5 6
|{6: ^3}|{7: ^3}|{8: ^3}|    1 2 3
+---+---+---+
",
            self.cell(7),
            self.cell(8),
            self.cell(9),
            self.cell(4),
            self.cell(5),
            self.cell(6),
            self.cell(1),
            self.cell(2),
            self.cell(3)
        ));
    }
}
impl Search {
    /// Principal variation. 今読んでる読み筋☆（＾～＾）
    pub fn pv(&self, pos: &mut Position) -> String {
        let mut pv = String::new();
        for t in self.root_pieces_num..pos.pieces_num {
            pv.push_str(&format!("{} ", pos.history[t]));
        }
        pv.trim_end().to_string()
    }

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
            self.pv(pos),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < pos.pieces_num + 1 {
                "none    ".to_string()
            } else {
                format!("height {}", pos.pieces_num + 1)
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
            self.pv(pos),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < pos.pieces_num {
                "none    ".to_string()
            } else {
                format!("height {}", pos.pieces_num)
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
            self.pv(pos),
            if SQUARES_NUM < pos.pieces_num + 1 {
                "none    ".to_string()
            } else {
                format!("height {}", pos.pieces_num + 1)
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
