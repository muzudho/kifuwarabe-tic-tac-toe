use crate::piece::Piece;

pub struct Board {
    /// 盤のマス☆（＾～＾） [0] は未使用☆（＾～＾）
    pub squares: [Option<Piece>; 10],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            squares: [None; 10],
        }
    }
}
