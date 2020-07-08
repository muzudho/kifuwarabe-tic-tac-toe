use crate::look_and_model::Search;

impl Search {
    pub fn sec(&self) -> u64 {
        self.stopwatch.elapsed().as_secs()
    }

    /// Node per second.
    pub fn nps(&self) -> u64 {
        let sec = self.sec();
        if 0 < sec {
            self.nodes as u64 / sec
        } else {
            // 1秒未満で全部探索してしまった☆（＾～＾） 本当は もっと多いと思うんだが☆（＾～＾）
            self.nodes as u64
        }
    }
}
