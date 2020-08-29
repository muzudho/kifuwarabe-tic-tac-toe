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

/// Circle and cross mark. It corresponds to the stone in Go.  
/// 丸と十字の印です。囲碁で言うところの石に当たります。  
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// 〇
    Nought,
    /// ×
    Cross,
}
