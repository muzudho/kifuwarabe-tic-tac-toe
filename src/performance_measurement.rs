use crate::look_and_model::Search;

impl Search {
    fn sec(&self) -> u64 {
        self.stopwatch.elapsed().as_secs()
    }

    /// Node per second.
    pub fn nps(&self) -> u64 {
        let sec = self.sec();
        if 0 < sec {
            self.nodes as u64 / sec
        } else {
            0
        }
    }
}
