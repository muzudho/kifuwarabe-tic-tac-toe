use crate::piece::Piece;
use crate::position::Position;
use std::fmt;

impl Position {
    /*
    pub fn debug_write(&self) {
        for (i, sq) in self.board.iter().enumerate() {
            println!(
                "{}: {}",
                i,
                if let Some(sq_val) = sq {
                    format!("{}", sq_val)
                } else {
                    "None".to_string()
                }
            );
        }
    }
    */
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.board[index] {
            format!("{}", piece)
        } else {
            "   ".to_string()
        }
    }
    pub fn pos(&self) {
        // println!("Debug   | position={:?}", self);
        println!(
            "[{} move(s) | Go {}]
",
            self.moves_num, self.friend
        );
        // 書式を指定したりで、桁がずれるのは仕方ないぜ☆（＾～＾）
        println!(
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
        );
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::piece::Piece::*;
        match self {
            Nought => write!(f, "O"),
            Cross => write!(f, "X"),
        }
    }
}
