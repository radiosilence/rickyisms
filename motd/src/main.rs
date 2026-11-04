use std::io::IsTerminal;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

const PAGE: &str = include_str!("../../index.html");

#[derive(Deserialize)]
struct Rickyism {
    s: String,
    a: String,
    #[serde(default)]
    e: String,
}

fn main() {
    let json = PAGE
        .lines()
        .find_map(|l| l.trim().strip_prefix("const DATA = "))
        .and_then(|l| l.strip_suffix(';'))
        .expect("index.html has no DATA line");
    let data: Vec<Rickyism> = serde_json::from_str(json).expect("DATA is not valid JSON");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let index = if std::env::args().any(|a| a == "-r" || a == "--random") {
        now.subsec_nanos() as usize
    } else {
        (now.as_secs() / 86_400) as usize
    } % data.len();
    let r = &data[index];

    let (bold, dim, reset) =
        if std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
            ("\x1b[1;33m", "\x1b[2m", "\x1b[0m")
        } else {
            ("", "", "")
        };

    println!("{bold}\u{201c}{}\u{201d}{reset}", r.s);
    println!("  {dim}= {}{reset}", r.a);
    if !r.e.is_empty() {
        println!("  {dim}({}){reset}", r.e);
    }
}
