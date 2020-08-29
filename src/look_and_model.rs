//! Display and data structure.  
//! 表示と、データ構造です。  
use crate::{
    GameResult, Piece, Position, Search, SearchDirection, SearchInfo, BOARD_LEN, SQUARES_NUM,
};
use std::fmt;
use std::time::Instant;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::look_and_model::GameResult::*;
        match self {
            Win => write!(f, "win"),
            Draw => write!(f, "draw"),
            Lose => write!(f, "lose"),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            starting_turn: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            starting_pieces_num: 0,
            turn: Piece::Nought,
            board: [None; BOARD_LEN],
            history: [0; SQUARES_NUM],
            pieces_num: 0,
            pv: String::new(),
            info_enabled: true,
        }
    }
}
impl Position {
    /// Display of square.  
    /// マスの表示。  
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!(" {} ", piece)
        } else {
            "   ".to_string()
        }
    }
    /// Display of position.  
    /// 局面の表示。  
    pub fn pos(&self) -> String {
        let s = &mut format!(
            "[Next {} move(s) | Go {}]

",
            self.pieces_num + 1,
            self.turn
        );
        s.push_str(&format!(
            "\
+---+---+---+ Please select a square. Example `do 7`
|{0}|{1}|{2}| マスを選んでください。例 `do 7`
+---+---+---+
|{3}|{4}|{5}|    7 8 9
+---+---+---+    4 5 6
|{6}|{7}|{8}|    1 2 3
+---+---+---+",
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
        s.to_string()
    }

    /// Display results.  
    /// 結果の表示。  
    pub fn result(result: GameResult, winner: Option<Piece>) -> Option<String> {
        match result {
            // ぜったい None が返ってこない仕様のときは .unwrap() でヌル・チェックを飛ばせだぜ☆（＾～＾）
            GameResult::Win => Some(format!("win {}", winner.unwrap()).to_string()),
            GameResult::Draw => Some(format!("draw").to_string()),
            GameResult::Lose => None,
        }
    }
}

impl Search {
    pub fn new(start_pieces_num: usize) -> Self {
        Search {
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
        }
    }

    /// Header.
    /// 見出し。
    pub fn info_header(pos: &Position) -> String {
        match pos.turn {
            Piece::Nought => {
                "info string \"nps\":......, \"nodes\":......, \"pv\":[O,X,O,X,O,X,O,X,O]"
                    .to_string()
            }
            Piece::Cross => {
                format!("info string \"nps\":......, \"nodes\":......, \"pv\":[X,O,X,O,X,O,X,O,X]")
                    .to_string()
            }
        }
    }
}

impl SearchInfo {
    pub fn new() -> Self {
        SearchInfo {
            search_direction: SearchDirection::Forward,
            nps: 0,
            nodes: 0,
            pv: "".to_string(),
            chosen_sq: 0,
            leaf: false,
            pieces_num: None,
            result: None,
            turn: Piece::Nought,
            comment: None,
        }
    }

    /// Information during a forward/backward search.
    /// 前向き/後ろ向き 探索中の情報。
    pub fn to_string(&self) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}, \"pv\":[{: <17}]{}{}{}{}{} }}",
            self.nps,
            self.nodes,
            self.pv,
            match self.search_direction {
                SearchDirection::Forward => {
                    format!(", \"push\":\"{}\"", self.chosen_sq)
                }
                SearchDirection::Backward => {
                    format!(", \"pop\" :\"{}\"", self.chosen_sq)
                }
            },
            if self.leaf {
                ", \"leaf\": true"
            } else {
                "              "
            },
            if let Some(pieces_num) = self.pieces_num {
                format!(", \"pieces\":{}", pieces_num)
            } else {
                "            ".to_string()
            },
            if let Some(result) = self.result {
                format!(", \"result\":{:6}", format!("\"{}\"", result.to_string()))
            } else {
                "                 ".to_string()
            },
            if let Some(comment) = &self.comment {
                format!(", \"{}\":\"{}\"", self.turn, comment).to_string()
            } else {
                format!(", \"{}\":\"\"", self.turn).to_string()
            },
        )
        .to_string()
    }
}
