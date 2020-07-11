use chrono::{Date, Duration, Local, TimeZone};
use regex::Regex;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use std::process;
use std::sync::Mutex;
use std::thread;

// For multi-platform. Windows, or not.
#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";
#[cfg(windows)]
const NEW_LINE_SEQUENCE: &'static str = "\\r\\n";
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";

/// ログ・レベル
pub enum LogLevel {
    /// Not included in the distribution.
    /// Remove this level from the source after using it for debugging.
    /// If you want to find a bug in the program, write a lot.
    Trace,
    /// It should be in a place with many accidents.
    /// This level is disabled in production environments.
    /// Leave it in the source and enable it for troubleshooting.
    /// Often, this is the production level of a desktop operating environment.
    Debug,
    /// Report highlights.
    /// Everything that needs to be reported regularly in the production environment.
    Info,
    /// It must be enabled in the server production environment.
    /// Record of passing important points correctly.
    /// I am monitoring while confirming that it operates normally.
    Notice,
    /// It will be abnormal soon, but there is no problem and you can ignore it.
    /// For example, he reported that it took longer to access than expected.
    /// For example, report that capacity is approaching the limit.
    Warn,
    /// Why you didn't get the results you expected.
    /// It may continue, such as trying other means.
    Error,
    /// Cannot continue.
    /// If the program could not be implemented due to force majeure.
    Fatal,
}
pub const LOG_ENABLE: bool = true;

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /// ロガーのミューテックス（排他制御）
    pub static ref LOGGER: Mutex<Logger> = {
        Mutex::new(Logger::default())
    };
}
// スレッド・ローカル
//
// * 参考
//      * [ミュータブルなスレッドローカルデータを thread_local!() マクロと RefCell で実現する](https://qiita.com/tatsuya6502/items/bed3702517b36afbdbca)
//
// ログの連番だぜ☆（＾～＾）
thread_local!(pub static SEQ: RefCell<u128> = {
    RefCell::new(1)
});

pub struct Log {}
impl Log {
    /// コンソール表示とともに、ファイルへ書き込みます。末尾に改行は付けません。
    #[allow(dead_code)]
    pub fn info(s: &str) {
        print!("{}", s);
        Log::write(s, "Info")
    }

    /// コンソール表示とともに、ファイルへ書き込みます。末尾に改行は付けます。
    #[allow(dead_code)]
    pub fn infoln(s: &str) {
        println!("{}", s);
        Log::writeln(s, "Info");
    }

    /// コンソール表示せず、ファイルへ書き込みます。末尾に改行は付けません。
    #[allow(dead_code)]
    pub fn trace(s: &str) {
        Log::write(s, "Trace")
    }

    /// コンソール表示せず、ファイルへ書き込みます。末尾に改行は付けます。
    #[allow(dead_code)]
    pub fn traceln(s: &str) {
        Log::writeln(s, "Trace");
    }

    /// 無視してよい異常。
    #[allow(dead_code)]
    pub fn warn(s: &str) {
        Log::write(s, "Warn")
    }

    /// 無視してよい異常。
    #[allow(dead_code)]
    pub fn warnln(s: &str) {
        Log::writeln(s, "Warn");
    }

    /// 記録するべき誤り。
    #[allow(dead_code)]
    pub fn error(s: &str) {
        Log::write(s, "Error")
    }

    /// 記録するべき誤り。
    #[allow(dead_code)]
    pub fn errorln(s: &str) {
        Log::writeln(s, "Error");
    }

    /// panic! の第一引数に渡せだぜ☆（＾～＾） 強制終了する前に、ヤケクソで読み筋欄に表示できないかトライするぜ☆（＾～＾）
    #[allow(dead_code)]
    pub fn panic(s: &str) -> String {
        // コンピューター将棋の USIプロトコル で 'info string' というのがあって真似ている☆（＾～＾）嫌なら変えろだぜ☆（＾～＾）
        let t = format!("info string panic! {}", s).to_string();
        Log::writeln(&t, "Fatal");
        println!("{}", t);
        t
    }

    /// ファイルに書き込みます。末尾に改行を付けます。
    #[allow(dead_code)]
    fn writeln(s: &str, level: &str) {
        let s = &format!("{}{}", s, NEW_LINE);
        Log::write(s, level);
    }
    /// ファイルに書き込みます。末尾に改行は付けません。
    #[allow(dead_code)]
    fn write(s: &str, level: &str) {
        if LOG_ENABLE {
            // 末尾の 改行コード は、エスケープするぜ☆（＾～＾）
            let mut body = if s[s.len() - NEW_LINE.len()..] == *NEW_LINE {
                // 末尾に 改行コード が有った☆（＾～＾）
                format!("{}{}", s.trim_end(), NEW_LINE_SEQUENCE)
            } else {
                // 末尾に 改行コード が無かった☆（＾～＾）
                s.to_string()
            };
            // ダブル・クォーテーションはエスケープするぜ☆（＾～＾）
            body = body.replace("\"", "\\\"");
            /*
            println!(
                "Body    =|{}| Len=|{}| LinesCount=|{}|",
                body,
                body.len(),
                s.lines().count()
            );
            */
            SEQ.with(move |seq| {
                // TOMLの書式を真似る。
                let toml = format!(
                    "[\"Now={}&Pid={}&Thr={:?}&Seq={}\"]
{} = {}

",
                    // ISO8601 なら "%Y-%m-%dT%H:%M:%S%z" なんだが、「UTCは止めてくれ！」と言うビューとモデルを切り分けられない人はいるので
                    // 書式は あなたの置かれている状況に合わせて自由とするぜ☆（＾～＾）
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    process::id(),
                    // Rustは 「ThreadId(1)」という分けわかんない文字列を返してくるんで、何が返ってくるか分かんないでそのままにしとこうぜ☆（＾～＾）？
                    thread::current().id(),
                    seq.borrow(),
                    level,
                    // 複数行か、単一行かを区別するぜ☆（＾～＾）
                    if 1 < s.lines().count() {
                        // 複数行で出力しようぜ☆（＾～＾）？
                        format!(
                            "\"\"\"
{}
\"\"\"",
                            body
                        )
                        .to_string()
                    } else {
                        // 単一行で出力できるぜ☆（＾～＾）
                        format!("\"{}\"", body).to_string()
                    }
                );
                *seq.borrow_mut() += 1;
                // write_allメソッドを使うには use std::io::Write; が必要
                if let Ok(mut logger) = LOGGER.lock() {
                    if let Err(_why) = logger.current_file().write_all(toml.as_bytes()) {
                        // 大会向けに、ログ書き込み失敗は出力しないことにする
                        // panic!("(Err.148) couldn't write log. : {}",Error::description(&why)),
                    }
                }
            });
        }
    }
}

pub struct LogFile {
    // 現在使用対象のログファイルの年月日だぜ☆（＾～＾）
    pub start_date: Date<Local>,
    pub file: File,
}
impl LogFile {
    pub fn new(start_date: Date<Local>, file: File) -> Self {
        LogFile {
            start_date: start_date,
            file: file,
        }
    }
}

pub struct Logger {
    /// 何のログか分かるように名前を付けてやれだぜ☆（＾～＾）
    file_prefix: String,
    /// このファイルは消してもいい、という符号としてファイル名の一部に '.log' と付けるのに使うことを想定しているぜ☆（＾～＾）
    file_suffix: String,
    /// 拡張子だぜ☆（＾～＾） '.toml' と付けるのを想定しているが、「TOML？何それ！」みたいに煙たがられたら、 suffix を空文字にして拡張子を '.log' にでもしとけだぜ☆（＾～＾）
    file_extention: String,
    /// ファイルを保持する日数だぜ☆（＾～＾）これを超えたら消す☆（＾～＾）
    pub retention_days: i64,
    /// ファイルをつかむのに使うぜ☆（＾～＾）
    log_file: Option<LogFile>,
}
impl Default for Logger {
    fn default() -> Self {
        let prefix = "default";
        let suffix = ".log";
        let extention = ".toml";
        Logger {
            file_prefix: prefix.to_string(),
            file_suffix: suffix.to_string(),
            file_extention: extention.to_string(),
            retention_days: 7,
            log_file: None,
        }
    }
}
impl Logger {
    #[allow(dead_code)]
    pub fn get_file_prefix(&self) -> &str {
        &self.file_prefix
    }
    #[allow(dead_code)]
    pub fn get_file_suffix(&self) -> &str {
        &self.file_suffix
    }
    #[allow(dead_code)]
    pub fn get_file_extention(&self) -> &str {
        &self.file_extention
    }
    /// ログ・ファイル名の日付以外のところを決めろだぜ☆（＾～＾）
    pub fn set_file_name(&mut self, prefix: &str, suffix: &str, extention: &str) {
        self.file_prefix = prefix.to_string();
        self.file_suffix = suffix.to_string();
        self.file_extention = extention.to_string();
    }
    /// 今日の日付が付いたログ・ファイルをつかむぜ☆（＾～＾）
    fn new_today_file(
        file_prefix: &str,
        file_suffix: &str,
        file_extention: &str,
    ) -> (Date<Local>, File) {
        // 'default-2020-07-11.log.toml' みたいな感じのファイル名を作るぜ☆（＾～＾）
        let start_date = Local::today();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(&format!(
                "{}-{}{}{}",
                file_prefix,
                start_date.format("%Y-%m-%d"),
                file_suffix,
                file_extention
            )))
            .unwrap();
        (start_date, file)
    }
    /// 古いログを削除しようぜ☆（＾～＾）？
    /// なんだこのクソむずかしい日付処理は☆（＾～＾）！？
    pub fn remove_old_logs(&self) -> usize {
        // 削除したファイル数だぜ☆（＾～＾）
        let mut count = 0;
        // Example:
        //      all = './tic-tac-toe-2020-07-11.log.toml'
        //      prefix = "tic-tac-toe"
        //      now = "-2020-07-11"
        //      suffix = ".log"
        //      extention = ".toml"
        let re = if let Ok(x) = Regex::new(&format!(
            "./{}-{}{}{}",
            self.file_prefix, r"(\d{4})-(\d{2})-(\d{2})", self.file_suffix, self.file_extention
        )) {
            x
        } else {
            return 0;
        };
        // file, directory paths:
        let paths = if let Ok(x) = fs::read_dir("./") {
            x
        } else {
            return 0;
        };
        for path in paths {
            let name = if let Ok(x) = path {
                x.path().display().to_string()
            } else {
                continue;
            };
            // println!("Name: {}", name);
            // File name pattern match:
            if let Some(caps) = re.captures(&name) {
                // println!("CapsLen: {}", caps.len());
                // 年月日を抽出するぜ☆（＾～＾）
                let year: i32 = if let Some(cap) = caps.get(1) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                let month: u32 = if let Some(cap) = caps.get(2) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                let day: u32 = if let Some(cap) = caps.get(3) {
                    if let Ok(n) = cap.as_str().parse() {
                        n
                    } else {
                        0
                    }
                } else {
                    0
                };
                // println!("Ymd=|{}|{}|{}|", year, month, day);
                if month != 0 && day != 0 {
                    let file_date = Local.ymd(year, month, day);

                    // ファイルの保存期間を超えていれば
                    if file_date.add(Duration::days(self.retention_days)) < Local::today() {
                        // そのファイルを削除するぜ☆（＾～＾）
                        // println!("Delete: {}", name);
                        if let Ok(_why) = fs::remove_file(name) {
                            // ファイルの削除に失敗したときの理由が、大会で送信されても嫌だしな☆（＾～＾）表示しないぜ☆（＾～＾）
                            // println!("! {:?}", why.kind());
                        } else {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// 日付を跨いだら新しいファイルに乗り換える仕組みだけど、テストできてない☆（＾～＾）知らね☆（＾～＾）
    fn current_file(&mut self) -> &File {
        // 日付が変わってたら☆（＾～＾）
        let date_changed = if let Some(log_file) = &self.log_file {
            log_file.start_date < Local::today()
        } else {
            false
        };
        // 新しいファイルに乗り換えようぜ☆（＾～＾）
        if date_changed {
            self.log_file = None;
        }

        // 未設定なら新規作成☆（＾～＾）
        if let None = self.log_file {
            let (start_date, file) =
                Logger::new_today_file(&self.file_prefix, &self.file_suffix, &self.file_extention);
            self.log_file = Some(LogFile::new(start_date, file));
        }

        &self.log_file.as_ref().unwrap().file
    }
}
