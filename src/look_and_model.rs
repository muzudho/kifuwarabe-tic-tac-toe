//! Display and data structure.  
//! 表示と、データ構造です。  
use std::fmt;
use std::time::Instant;

/// Circle and cross mark. It corresponds to the stone in Go.  
/// 丸と十字の印です。囲碁で言うところの石に当たります。  
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

/// It is a game that can be fully analyzed, so please use the result instead of the evaluation value.  
/// 完全解析できるゲームなので、評価値ではなく結果を使います。  
#[derive(Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
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

/// The addresses of the squares start with 1 and end with 9.  
/// The array starts at 0, so the size is 10.  
/// マスの番地は1から始まり9で終わります。  
/// 配列は 0 から始まるのでサイズは10です。  
pub const BOARD_LEN: usize = 10;

/// The maximum number of stones that can be placed on the board.  
/// Since there are only 9 squares, it will be 9.  
/// 盤上に置ける石の最大数。  
/// ９マスしかないから９です。  
pub const SQUARES_NUM: usize = 9;

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
#[derive(Debug)]
pub struct Position {
    /// Turn. The stone to be placed next.  
    /// 手番。次に置かれる石。  
    pub turn: Piece,

    /// The board at the start. [0] is unused.  
    /// 開始時の盤面。 [0] は未使用。  
    pub starting_board: [Option<Piece>; BOARD_LEN],

    /// The number of stones on the board at the start.  
    /// 開始時に盤の上に有った石の数。  
    pub starting_pieces_num: usize,

    /// The current board. [0] is unused.  
    /// 現在の盤面。 [0] は未使用。  
    pub board: [Option<Piece>; BOARD_LEN],

    /// Match record. An array of addresses where the pieces will be placed.  
    /// 棋譜。駒を置いた番地を並べたもの。  
    pub history: [u8; SQUARES_NUM],

    /// The number of stones currently on the board.  
    /// 現在、盤の上に有る石の数。  
    pub pieces_num: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            turn: Piece::Nought,
            starting_board: [None; BOARD_LEN],
            starting_pieces_num: 0,
            board: [None; BOARD_LEN],
            history: [0; SQUARES_NUM],
            pieces_num: 0,
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

/// Search.  
/// 探索部。  
pub struct Search {
    /// The number of stones on the board at the start of this search.  
    /// この探索の開始時に盤の上に有った石の数。  
    pub start_pieces_num: usize,
    /// Number of state nodes searched.  
    /// 探索した状態ノード数。  
    pub nodes: u32,
    /// Start the stopwatch when this structure is created.  
    /// この構造体を生成した時点からストップ・ウォッチを開始します。  
    pub stopwatch: Instant,
}
impl Search {
    pub fn new(start_pieces_num: usize) -> Self {
        Search {
            start_pieces_num: start_pieces_num,
            nodes: 0,
            stopwatch: Instant::now(),
        }
    }

    /// Principal variation.
    /// 今読んでる読み筋。
    pub fn pv(&self, pos: &Position, separator: char) -> String {
        let mut pv = String::new();
        for t in self.start_pieces_num..pos.pieces_num {
            pv.push_str(&format!("{}{}", pos.history[t], separator));
        }

        if 0 < pv.len() {
            pv.pop();
        }

        pv.to_string()
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

    /// Information during a forward search.
    /// 前向き探索中の情報。
    pub fn info_forward(
        &self,
        nps: u64,
        pos: &Position,
        sq: usize,
        comment: Option<&str>,
    ) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}, \"pv\":[{: <17}], \"push\":\"{}\",               \"pieces\":{},                  \"turn\":\"{}\"{} }}",
            nps,
            self.nodes,
            self.pv(pos,','),
            sq,
            pos.pieces_num,
            pos.turn,
            if let Some(comment) = comment {
                format!(", \"comment\":\"{}\"",comment).to_string()
            } else {
                "".to_string()
            },
        )
        .to_string()
    }

    /// It's a leaf. Information during a forward search.
    /// 葉。前向き探索中の情報。
    pub fn info_forward_leaf(
        &self,
        nps: u64,
        pos: &Position,
        sq: usize,
        result: GameResult,
        comment: Option<&str>,
    ) -> String {
        format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}, \"pv\":[{: <17}], \"push\":\"{}\", \"leaf\": true, \"pieces\":{}, \"result\":{:6}, \"turn\":\"{}\"{} }}",
            nps,
            self.nodes,
            self.pv(pos,','),
            sq,
            pos.pieces_num,
            format!("\"{}\"",result.to_string()),
            pos.turn,
            if let Some(comment) = comment {
                format!(", \"comment\":\"{}\"", comment).to_string()
            } else {
                "".to_string()
            },
        )
        .to_string()
    }
    /// Information during a backward search.
    /// 後ろ向き探索中の情報。
    pub fn info_backward(
        &self,
        nps: u64,
        pos: &Position,
        sq: usize,
        result: GameResult,
        comment: Option<&str>,
    ) -> String {
        return format!(
            "info json {{ \"nps\":{: >6}, \"nodes\":{: >6}, \"pv\":[{: <17}], \"pop\" :\"{}\",               \"pieces\":{}, \"result\":{:6}, \"turn\":\"{}\"{} }}",
            nps,
            self.nodes,
            self.pv(pos,','),
            sq,
            pos.pieces_num,
            format!("\"{}\"",result.to_string()),
            pos.turn,
            if let Some(comment) = comment {
                format!(", \"comment\":\"{}\"", comment).to_string()
            } else {
                "".to_string()
            }
        )
        .to_string();
    }
}
