use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let a: String = args[1..].join(" ");

    if args.len() > 2 {
        println!();
    } else {
        println!("{}", a)
    }
}
