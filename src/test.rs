use crate::position::Position;

/// 動作テストだぜ☆（＾～＾）
pub fn test() {
    // O 勝ち局面チェック☆（＾～＾）
    debug_assert!(Position::from_xfen("xfen ooo/3/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/ooo/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/3/ooo x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen o2/o2/o2 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 1o1/1o1/1o1 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2o/2o/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen o2/1o1/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2o/1o1/o2 x")
        .unwrap()
        .is_opponent_win());
    // O 勝ってない局面チェック☆（＾～＾）
    debug_assert!(!Position::from_xfen("xfen xoo/3/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/xoo/3 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/3/xoo x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen x2/o2/o2 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 1x1/1o1/1o1 x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2x/2o/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen x2/1o1/2o x")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2x/1o1/o2 x")
        .unwrap()
        .is_opponent_win());
    // X 勝ち局面チェック☆（＾～＾）
    debug_assert!(Position::from_xfen("xfen xxx/3/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/xxx/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 3/3/xxx o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen x2/x2/x2 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 1x1/1x1/1x1 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2x/2x/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen x2/1x1/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(Position::from_xfen("xfen 2x/1x1/x2 o")
        .unwrap()
        .is_opponent_win());
    // X 勝ってない局面チェック☆（＾～＾）
    debug_assert!(!Position::from_xfen("xfen oxx/3/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/oxx/3 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 3/3/oxx o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen o2/x2/x2 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 1o1/1x1/1x1 o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2o/2x/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen o2/1x1/2x o")
        .unwrap()
        .is_opponent_win());
    debug_assert!(!Position::from_xfen("xfen 2o/1x1/x2 o")
        .unwrap()
        .is_opponent_win());
    // 引き分けチェック☆（＾～＾）
    {
        debug_assert!(Position::from_xfen("xfen xox/xox/oxo x").unwrap().is_draw());
        debug_assert!(!Position::from_xfen("xfen xox/xox/oxo x")
            .unwrap()
            .is_opponent_win());
    }

    // TODO 次の１手チェック
    // xfen 3/3/3 o moves 5 1 2 8 3 7
    // 受け手なし
    // xfen 3/3/3 o moves 5 1 2 8
    // 3 ならo負け。4,7,9しか無いか？
    //
    // xfen 3/3/3 o moves 5 1 2 8 3 7
    // メート -2
}
