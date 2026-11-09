use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: mkdir [Dir-Name]");
    } else {
        fs::create_dir(&args[1]).expect("Cannot Create Dir!");
    }
}
