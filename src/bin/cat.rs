use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: cat [File-Name]");
    } else {
        let c = fs::read_to_string(&args[1]).expect("Cannot Read File!");
        print!("{}", c);
    }
}
