use casual_logger::Log;
use std::fmt;

/// Parses the command to make it easier to use.  
/// コマンドを解析して、使いやすくします。  
pub struct CommandLineParser {
    line: String,
    len: usize,
    pub starts: usize,
}
impl CommandLineParser {
    pub fn new(line: &str) -> Self {
        // Erase the trailing newline.
        // 末尾の改行を削除します。
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!(Log::fatal(&format!("(Err.38)  Failed to parse. / {}", e))),
        };
        // character count.
        // 文字数。
        let len = line.chars().count();
        CommandLineParser {
            line: line,
            len: len,
            starts: 0,
        }
    }

    /// Does the character match from the beginning?  
    /// 文字は先頭から一致していますか？  
    pub fn starts_with(&self, expected: &str) -> bool {
        let len2 = expected.len();
        len2 <= self.len && &self.line[self.starts..len2] == expected
    }

    /// Advance the scanning position.
    /// 読み取り位置を進めます。
    pub fn go_next_to(&mut self, expected: &str) {
        self.starts += expected.len();
    }

    /// The rest of the string.
    /// 文字列の残りの部分。
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
        // Tips. It is convenient to make a table by enclosing it with vertical bars.
        // Example: value=|apple|banana|cherry|
        // テクニック。 '|' で囲んでテーブルを作成すると便利です。
        // 例: value=|りんご|バナナ|さくらんぼ|
        "line=|{}| len={} starts={}",
            self.line, self.len, self.starts
        )
    }
}
