use std::io::IsTerminal;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use serde::Deserialize;

const DATA: &str = include_str!("../../rickyisms.json");

/// Prints a Rickyism of the day.
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Pick a random Rickyism instead of today's
    #[arg(short, long)]
    random: bool,

    /// Print a shell completion script and exit
    #[arg(long, value_name = "SHELL")]
    completions: Option<Shell>,
}

#[derive(Deserialize)]
struct Rickyism {
    s: String,
    a: String,
    #[serde(default)]
    e: String,
}

fn main() {
    let cli = Cli::parse();
    if let Some(shell) = cli.completions {
        clap_complete::generate(
            shell,
            &mut Cli::command(),
            "rickyism",
            &mut std::io::stdout(),
        );
        return;
    }

    let data: Vec<Rickyism> = serde_json::from_str(DATA).expect("rickyisms.json is not valid JSON");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let index = if cli.random {
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
