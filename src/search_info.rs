use crate::log::Log;
use crate::position::Position;
use crate::position::SQUARES_NUM;
use crate::search::Search;

impl Search {
    /// 前向き探索中だぜ☆（＾～＾）
    pub fn info_forward(
        &self,
        pos: &mut Position,
        addr: usize,
        mate: Option<i8>,
        comment: Option<String>,
    ) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | ->   to {} |       |{}|{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < self.depth {
                "none   ".to_string()
            } else {
                format!("depth {}", self.depth)
            },
            if let Some(cur_mate) = mate {
                format!("     ({: >2})", cur_mate)
            } else {
                "         ".to_string()
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
        result: String,
        comment: Option<String>,
    ) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | .       {} |       | {:4}    |{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if SQUARES_NUM < self.depth - 1 {
                "none   ".to_string()
            } else {
                format!("depth {}", self.depth - 1)
            },
            result,
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
        mate: Option<i8>,
        comment: Option<String>,
    ) {
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} |       | <- from {} | {} [{}] |{}|{}",
            self.nps(),
            self.nodes,
            self.pv(),
            if SQUARES_NUM < self.depth {
                "none   ".to_string()
            } else {
                format!("depth {}", self.depth)
            },
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            if let Some(mate) = mate {
                format!(" mate {: >2} ", mate)
            } else {
                "         ".to_string()
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
