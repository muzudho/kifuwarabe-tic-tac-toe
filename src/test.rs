use crate::position::Position;

/// 動作テストだぜ☆（＾～＾）
pub fn test() {
    // O 勝ちチェック☆（＾～＾）
    debug_assert!(Position::from_xfen("xfen ooo/3/3 o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 3/ooo/3 o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 3/3/ooo o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen o2/o2/o2 o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 1o1/1o1/1o1 o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 2o/2o/2o o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen o2/1o1/2o o").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 2o/1o1/o2 o").unwrap().is_win());
    // X 勝ちチェック☆（＾～＾）
    debug_assert!(Position::from_xfen("xfen xxx/3/3 x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 3/xxx/3 x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 3/3/xxx x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen x2/x2/x2 x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 1x1/1x1/1x1 x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 2x/2x/2x x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen x2/1x1/2x x").unwrap().is_win());
    debug_assert!(Position::from_xfen("xfen 2x/1x1/x2 x").unwrap().is_win());
    // X 勝ってないチェック☆（＾～＾）
    debug_assert!(!Position::from_xfen("xfen oxx/3/3 x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen 3/oxx/3 x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen 3/3/oxx x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen o2/x2/x2 x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen 1o1/1x1/1x1 x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen 2o/2x/2x x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen o2/1x1/2x x").unwrap().is_win());
    debug_assert!(!Position::from_xfen("xfen 2o/1x1/x2 x").unwrap().is_win());
}
