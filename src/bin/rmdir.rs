use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: rmdir [Dir-Name]");
    } else {
        fs::remove_dir(&args[1]).expect("Cannot Remove Dir!");
    }
}
