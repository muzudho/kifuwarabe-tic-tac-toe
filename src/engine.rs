//! ThinkingEngine.  
//! 思考エンジン。  

use crate::{
    command_line_seek::CommandLineSeek,
    log::LogExt,
    look_and_model::{Position, Search},
};
use casual_logger::{Level, Log};

/// ThinkingEngine.  
/// 思考エンジン。  
pub struct Engine {
    /// Starting position.  
    /// 初期局面。  
    pos: Position,
}
impl Default for Engine {
    fn default() -> Self {
        Engine {
            // Starting position.
            // 初期局面。
            pos: Position::default(),
        }
    }
}
impl Engine {
    /// Enter the command line.  
    /// コマンドラインを与えてください。  
    ///
    /// # Arguments
    ///
    /// * `line` - Command line.  
    ///             コマンドライン。  
    ///
    /// # Returns
    ///
    /// If this response quit, exit the your application.  
    /// Quitならアプリケーションを終了してください。  
    pub fn enter(&mut self, line: &str) -> Option<Response> {
        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineSeek::new(&line);

        // It is in alphabetical order because it is easy to find.
        // 探しやすいからアルファベット順です。
        if p.starts_with("do") {
            p.go_next_to("do ");
            if let Some(rest) = p.rest() {
                self.pos.do_(rest);
            }
        } else if p.starts_with("go") {
            let mut search = Search::new(self.pos.pieces_num);
            let (sq, result) = search.go(&mut self.pos);
            Log::print_info(&format!(
                "info string result={:?} nps={}",
                result,
                search.nps()
            ));

            Log::print_notice(&format!(
                "bestmove {}",
                if let Some(sq) = sq {
                    sq.to_string()
                } else {
                    "resign".to_string()
                }
            ));
        } else if p.starts_with("info-off") {
            Log::set_level(Level::Notice);
        } else if p.starts_with("info-on") {
            Log::set_level(Level::Info);
        } else if p.starts_with("position") {
            p.go_next_to("position ");
            if let Some(rest) = p.rest() {
                if let Some(pos_val) = Position::from_xfen(rest) {
                    self.pos = pos_val;
                }
            }
        } else if p.starts_with("pos") {
            Log::print_notice(&self.pos.pos());
        } else if p.starts_with("quit") {
            return Some(Response::Quit);
        } else if p.starts_with("undo") {
            self.pos.undo();
        } else if p.starts_with("uxi") {
            Log::print_notice("uxiok tic-tac-toe v20200718.0.0");
        } else if p.starts_with("xfen") {
            Log::print_notice(&format!("{}", self.pos.to_xfen()));
        } else {
            Log::print_debug(&format!("Debug   | Invalid command=|{:?}|", p));
        }

        None
    }
}

/// Engine response.
/// エンジンの応答。
pub enum Response {
    /// Quit.
    /// 終了。
    Quit,
}
