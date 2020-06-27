use crate::piece::Piece;

pub struct Board {
    pub squares: [Option<Piece>; 9],
}
impl Default for Board {
    fn default() -> Self {
        Board { squares: [None; 9] }
    }
}
