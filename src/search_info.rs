use crate::log::Log;
use crate::position::Position;
use crate::search::Search;

impl Search {
    /// 前向き探索中だぜ☆（＾～＾）
    pub fn info_forward(&self, pos: &mut Position, addr: usize, cur_mate: Option<i8>) {
        Log::println(&format!(
            "info pv {: <17} | {} [{}] | ->   to depth {} |       |{}|",
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            self.depth,
            if let Some(cur_mate) = cur_mate {
                format!("     ({: >2})", cur_mate)
            } else {
                "         ".to_string()
            }
        ))
    }
    pub fn info_win_up(&self, pos: &mut Position, addr: usize) {
        Log::println(&format!(
            "info pv {: <17} | {} [{}] | .       depth {} |       | {:4}    |",
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            self.depth - 1,
            if pos.friend == self.root_friend {
                "win".to_string()
            } else {
                "lose".to_string()
            },
        ));
    }
    pub fn info_draw_up(&self, pos: &mut Position, addr: usize) {
        Log::println(&format!(
            "info pv {: <17} | {} [{}] | .       depth {} |       | draw    |",
            self.pv(),
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            self.depth - 1,
        ))
    }
    // 後ろ向き探索のときの表示だぜ☆（＾～＾）
    pub fn backward_str(
        &self,
        pv: String,
        friend: String,
        addr: usize,
        child_mate: Option<i8>,
    ) -> String {
        format!(
            "pv {: <17} |       | <- from depth {} | {} [{}] | {} |",
            pv,
            self.depth,
            friend,
            addr,
            if let Some(child_mate) = child_mate {
                format!("mate {: >2}", child_mate)
            } else {
                "       ".to_string()
            },
        )
    }
    pub fn info_win_down(&self, pos: &mut Position, addr: usize, mate: i8) {
        Log::println(&format!(
            "info pv {: <17} |       | <- from depth {} | {} [{}] | mate {: >2} |",
            self.pv(),
            self.depth,
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr,
            mate
        ))
    }
    pub fn info_draw_down(&self, pos: &mut Position, addr: usize) {
        Log::println(&format!(
            "info pv {: <17} |       | <- from depth {} | {} [{}] |         |",
            self.pv(),
            self.depth,
            if pos.friend == self.root_friend {
                "+".to_string()
            } else {
                "-".to_string()
            },
            addr
        ))
    }
}
