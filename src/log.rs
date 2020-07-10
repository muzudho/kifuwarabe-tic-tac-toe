use chrono::{Local, Utc};
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::sync::Mutex;
use std::thread;

// TODO Windows と Linuxディストリビューションの改行の違いどうにかならんのか☆（＾～＾）？
#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";
#[cfg(windows)]
const NEW_LINE_SEQUENCE: &'static str = "\\r\\n";
#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";
#[cfg(not(windows))]
const NEW_LINE_SEQUENCE: &'static str = "\\n";

/// ログ
pub const LOG_FILE_STEM: &str = "tic-tac-toe";
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
    /// ログ・ファイルのミューテックス（排他制御）
    pub static ref LOGFILE: Mutex<File> = {
        // ファイル名を作るぜ☆（＾～＾）
        let name = format!("{}-{}.log.toml",LOG_FILE_STEM,Utc::today().format("%Y-%m-%d").to_string());
        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        // 毎回新規作成するので、空っぽから始まります。
        Mutex::new(File::create(Path::new(&name)).unwrap())
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
            println!(
                "Body    =|{}| Len=|{}| LinesCount=|{}|",
                body,
                body.len(),
                s.lines().count()
            );
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
                if let Err(_why) = LOGFILE.lock().unwrap().write_all(toml.as_bytes()) {
                    // 大会向けに、ログ書き込み失敗は出力しないことにする
                    // panic!("(Err.148) couldn't write log. : {}",Error::description(&why)),
                }
            });
        }
    }
}
