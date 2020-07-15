//! 入力されたコマンドを、読み取る手伝いをするぜ☆（＾～＾）
use casual_logger::Log;
use std::fmt;

pub struct CommandLineParser {
    line: String,
    len: usize,
    pub starts: usize,
}
impl CommandLineParser {
    pub fn new(line: &str) -> Self {
        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!(Log::fatal(&format!("(Err.38)  Failed to parse. / {}", e))),
        };
        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        CommandLineParser {
            line: line,
            len: len,
            starts: 0,
        }
    }

    pub fn starts_with(&self, expected: &str) -> bool {
        let len2 = expected.len();
        len2 <= self.len && &self.line[self.starts..len2] == expected
    }

    pub fn go_next_to(&mut self, expected: &str) {
        self.starts += expected.len();
    }

    pub fn rest(&self) -> Option<&str> {
        if self.starts < self.line.len() {
            Some(&self.line[self.starts..])
        } else {
            None
        }
    }
}
impl fmt::Debug for CommandLineParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
        // 文字列を タテボウで クォートする(挟む)のは わたしの癖で、
        // |apple|banana|cherry| のように区切れる☆（＾～＾）
        // そのうち めんどくさくなったら お前もこうなる☆ｍ９（＾～＾）
        "line=|{}| len={} starts={}",
            self.line, self.len, self.starts
        )
    }
}
