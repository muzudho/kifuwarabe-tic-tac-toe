use crate::position::Position;

/// 最善の番地を返すぜ☆（＾～＾）
pub fn go(pos: &mut Position) -> (Option<u8>, i8) {
    // もう全部置いていて、置き場所がないときだぜ☆（＾～＾）
    node(pos)
}

fn node(pos: &mut Position) -> (Option<u8>, i8) {
    let mut best_addr = None;
    let mut best_mate = 0;

    for addr in 1..9 {
        // 空きマスがあれば
        if let None = pos.board[addr] {
            // とりあえず置いてみようぜ☆（＾～＾）
            pos.board[addr] = Some(pos.friend);

            // 勝ったかどうか判定しようぜ☆（＾～＾）？
            if pos.is_win() {
                // 勝ったなら☆（＾～＾）
                // 置いたところを戻そうぜ☆（＾～＾）？
                pos.board[addr] = None;

                // 探索終了だぜ☆（＾～＾）
                return (Some(addr as u8), 1);
            }

            pos.change_phase();

            // 相手の番だぜ☆（＾～＾）
            let (address, mut mate) = node(pos);
            if 0 < mate {
                // 相手がメートしたら、こっちはメートされてるんだぜ☆（＾～＾）
                mate = -mate - 1;
            } else if mate < 0 {
                // 相手がメートされてるんなら、こっちはメートしてるんだぜ☆（＾～＾）
                mate = -mate + 1;
            }

            if best_addr == None || mate < best_mate {
                // 最初に見つけた手か、より短手数のメートを見つけていたら、更新だぜ☆（＾～＾）
                best_addr = address;
                best_mate = mate;
            }

            // 置いたところを戻そうぜ☆（＾～＾）？
            pos.board[addr] = None;
            pos.change_phase();
        }
    }

    (best_addr, best_mate)
}
