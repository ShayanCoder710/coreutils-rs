use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: sleep [Seconds]");
        return;
    }

    let s = args[1].parse().expect("Please Enter a Number!");

    thread::sleep(Duration::from_secs(s));
}
