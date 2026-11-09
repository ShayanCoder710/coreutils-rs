use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: touch [File-Name]");
    } else {
        fs::File::create(&args[1]).expect("Cannot Create File!");
    }
}
