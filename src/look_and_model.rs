use crate::log::Log;
use std::fmt;
use std::time::Instant;

/// 駒とか、石とかのことだが、〇×は 何なんだろうな、マーク☆（＾～＾）？
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}

/// 〇×ゲームは完全解析できるから、評価ではなくて、ゲームの結果が分かるんだよな☆（＾～＾）
#[derive(Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

/// 1スタートで9まで☆（＾～＾） 配列には0番地もあるから、要素数は10だぜ☆（＾～＾）
pub const BOARD_LEN: usize = 10;

/// 盤上に置ける最大の駒数だぜ☆（＾～＾） ９マスしか置くとこないから９だぜ☆（＾～＾）
pub const SQUARES_NUM: usize = 9;

/// 局面☆（＾～＾）ゲームデータをセーブしたり、ロードしたりするときの保存されてる現状だぜ☆（＾～＾）
#[derive(Debug)]
pub struct Position {
    /// 次に盤に置く駒☆（＾～＾）
    /// 英語では 手番は your turn, 相手版は your opponent's turn なんで 手番という英語は無い☆（＾～＾）
    /// 自分という意味の単語はプログラム用語と被りまくるんで、
    /// あまり被らない 味方(friend) を手番の意味で たまたま使ってるだけだぜ☆（＾～＾）
    pub friend: Piece,

    /// 開始局面の盤の各マス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub starting_board: [Option<Piece>; BOARD_LEN],
    /// 盤の上に最初から駒が何個置いてあったかだぜ☆（＾～＾）
    pub starting_pieces_num: usize,

    /// 現状の盤の各マス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub board: [Option<Piece>; BOARD_LEN],

    /// 棋譜だぜ☆（＾～＾）駒を置いた番地を並べてけだぜ☆（＾～＾）
    pub history: [u8; SQUARES_NUM],

    /// 盤の上に駒が何個置いてあるかだぜ☆（＾～＾）
    pub pieces_num: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            friend: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            starting_pieces_num: 0,
            board: [None; BOARD_LEN],
            history: [0; SQUARES_NUM],
            pieces_num: 0,
        }
    }
}
impl Position {
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!(" {} ", piece)
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
        // 書式指定子は cell関数の方に任せるぜ☆（＾～＾）
        Log::println(&format!(
            "\
+---+---+---+
|{0}|{1}|{2}| マスを選んでください。例 `do 7`
+---+---+---+
|{3}|{4}|{5}|    7 8 9
+---+---+---+    4 5 6
|{6}|{7}|{8}|    1 2 3
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

    pub fn print_result(&self) {
        if self.is_opponent_win() {
            Log::println(&format!("win {}", self.opponent()));
        } else if self.is_draw() {
            Log::println(&format!("draw"));
        }
    }
}

/// 探索部☆（＾～＾）
pub struct Search {
    /// この探索を始めたのはどっち側か☆（＾～＾）
    pub start_friend: Piece,
    /// この探索を始めたときに石はいくつ置いてあったか☆（＾～＾）
    pub start_pieces_num: usize,
    /// 探索した状態ノード数☆（＾～＾）
    pub nodes: u32,
    /// この構造体を生成した時点からストップ・ウォッチを開始するぜ☆（＾～＾）
    pub stopwatch: Instant,
    /// info の出力の有無。
    pub info_enable: bool,
}
impl Search {
    /// 初期値だぜ☆（＾～＾）
    pub fn new(friend: Piece, start_pieces_num: usize, info_enable: bool) -> Self {
        Search {
            start_friend: friend,
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
            info_enable: info_enable,
        }
    }

    /// Principal variation. 今読んでる読み筋☆（＾～＾）
    pub fn pv(&self, pos: &mut Position) -> String {
        let mut pv = String::new();
        for t in self.start_pieces_num..pos.pieces_num {
            pv.push_str(&format!("{} ", pos.history[t]));
        }
        pv.trim_end().to_string()
    }

    pub fn info_header(pos: &mut Position) {
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
        let friend_str = if pos.friend == self.start_friend {
            "+".to_string()
        } else {
            "-".to_string()
        };
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | ->   to {} |       |      |{}",
            self.nps(),
            self.nodes,
            self.pv(pos),
            friend_str,
            addr,
            if SQUARES_NUM < pos.pieces_num + 1 {
                "none    ".to_string()
            } else {
                format!("height {}", pos.pieces_num + 1)
            },
            if let Some(comment) = comment {
                format!(" {} \"{}\"", friend_str, comment)
            } else {
                "".to_string()
            },
        ))
    }
    /// 前向き探索で葉に着いたぜ☆（＾～＾）
    pub fn info_forward_leaf(
        &self,
        pos: &mut Position,
        addr: usize,
        result: GameResult,
        comment: Option<String>,
    ) {
        let friend_str = if pos.friend == self.start_friend {
            "+".to_string()
        } else {
            "-".to_string()
        };
        Log::println(&format!(
            "info nps {: >6} nodes {: >6} pv {: <17} | {} [{}] | .       {} |       |{}|{}",
            self.nps(),
            self.nodes,
            self.pv(pos),
            friend_str,
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
                format!(" {} \"{}\"", friend_str, comment)
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
        let friend_str = if pos.friend == self.start_friend {
            "+".to_string()
        } else {
            "-".to_string()
        };
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
            friend_str,
            addr,
            match result {
                GameResult::Win => " win  ".to_string(),
                GameResult::Draw => " draw ".to_string(),
                GameResult::Lose => " lose ".to_string(),
            },
            if let Some(comment) = comment {
                format!(" {} \"{}\"", friend_str, comment)
            } else {
                "".to_string()
            }
        ));
    }
}
