use std::env;
use std::io::{self, BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let stdout = io::stdout();
    let mut w = BufWriter::new(stdout);

    if args.len() < 2 {
        loop {
            if writeln!(w, "y").is_err() {
                break;
            }
        }
    } else {
        let n = args[1..].join(" ");
        loop {
            if writeln!(w, "{}", n).is_err() {
                break;
            }
        }
    }
}
