use crate::board::Board;
use crate::piece::Piece;
use std::fmt;

impl Board {
    fn cell(&self, index: usize) -> String {
        if let Some(piece) = self.squares[index] {
            format!("{}", piece)
        } else {
            "   ".to_string()
        }
    }
    pub fn println(&self) {
        println!(
            "\
+---+---+---+
|{6}|{7}|{8}| マスを選んでください
+---+---+---+
|{3}|{4}|{5}|    7 8 9
+---+---+---+    4 5 6
|{0}|{1}|{2}|    1 2 3
+---+---+---+
",
            self.cell(6),
            self.cell(7),
            self.cell(8),
            self.cell(3),
            self.cell(4),
            self.cell(5),
            self.cell(0),
            self.cell(1),
            self.cell(2)
        );
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::piece::Piece::*;
        match self {
            Nought => write!(f, " X "),
            Cross => write!(f, " O "),
        }
    }
}
