use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout  = stdout();
    let message = String::from("Welcome to my Rust project repository!");
    let width   = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
