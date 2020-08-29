//! A small example before developing computer chess and computer shogi.  
//! Tic-tac-toe is an unspecified UXI protocol. X has no meaning.  
//! Come see the repository.  
//! コンピューター・チェスおよびコンピューター将棋を開発する前の小さな例です。  
//! 三目並べは、未指定のUXIプロトコルです。 Xには意味がありません。  
//! リポジトリをご覧ください。  

// Publish:
//
// (1) `cargo test`
// (2a) `cargo run --example debug`
// (2b) `cargo run --example main`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Version up on Cargo.toml.
// (6) `cargo doc --open`
// (7) Comit to Git-hub.
// (8) `cargo publish --dry-run`
// (9) `cargo publish`

extern crate chrono;
extern crate lazy_static;
extern crate regex;

pub mod command_line_seek;
mod computer_player;
pub mod engine;
pub mod log;
pub mod look_and_model;
mod performance_measurement;
mod position;
pub mod test;
mod uxi_protocol;
mod win_lose_judgment;

use std::time::Instant;

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

/// Circle and cross mark. It corresponds to the stone in Go.  
/// 丸と十字の印です。囲碁で言うところの石に当たります。  
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}

/// It is a game that can be fully analyzed, so please use the result instead of the evaluation value.  
/// 完全解析できるゲームなので、評価値ではなく結果を使います。  
#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

/// A record of the game used to suspend or resume it.  
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。  
#[derive(Debug)]
pub struct Position {
    /// Turn. The stone to be placed next at the start.  
    /// 開始局面での手番。次に置かれる石。  
    pub starting_turn: Piece,

    /// The board at the start. [0] is unused.  
    /// 開始時の盤面。 [0] は未使用。  
    pub starting_board: [Option<Piece>; BOARD_LEN],

    /// The number of stones on the board at the start.  
    /// 開始時に盤の上に有った石の数。  
    pub starting_pieces_num: usize,

    /// Turn. The stone to be placed next.  
    /// 手番。次に置かれる石。  
    pub turn: Piece,

    /// The current board. [0] is unused.  
    /// 現在の盤面。 [0] は未使用。  
    pub board: [Option<Piece>; BOARD_LEN],

    /// Match record. An array of addresses where the pieces will be placed.  
    /// 棋譜。駒を置いた番地を並べたもの。  
    pub history: [u8; SQUARES_NUM],

    /// The number of stones currently on the board.  
    /// 現在、盤の上に有る石の数。  
    pub pieces_num: usize,

    /// Principal variation.
    /// 今読んでる読み筋。
    pub pv: String,

    /// Display info during search. It is not info level in the log.  
    /// 探索中の info 表示を行います。 ログの情報レベルのことではありません。  
    pub info_enabled: bool,
}

/// Proceeding from the root toward the leaves is called a forward search.
/// The process of returning from the leaves toward the root is called backward search.
/// 根から葉に向かって進んでいることを前向き探索と呼びます。
/// 葉から根に戻っていることを後ろ向き探索と呼びます。
pub enum SearchDirection {
    /// Forward search.
    /// 前向き探索。
    Forward,
    /// Backward search.
    /// 後ろ向き探索。
    Backward,
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

pub struct SearchInfo {
    /// Node par seconds.  
    /// 秒間当たりの探索ノード数。  
    pub nps: u64,

    /// Searched nodes.  
    /// 探索状態ノード数。  
    pub nodes: u32,

    /// Principal variation.  
    /// 読み筋の本筋。  
    pub pv: String,

    /// Search direction.  
    /// 探索方向。  
    pub search_direction: SearchDirection,

    /// Chosen square.  
    /// 選んだマス。  
    pub chosen_sq: usize,

    /// Is leaf?  
    /// 葉ですか？  
    pub leaf: bool,

    /// Number of pieces on the board.  
    /// 盤上の石の数。  
    pub pieces_num: Option<usize>,

    /// Result.  
    /// 結果。  
    pub result: Option<GameResult>,

    /// Turn.  
    /// 手番。  
    pub turn: Piece,

    /// Comment.  
    /// コメント。  
    pub comment: Option<String>,
}
