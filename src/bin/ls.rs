use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target = if args.len() < 2 { "." } else { &args[1] };

    let e = fs::read_dir(target).expect("Cannot Read Dir!");

    for er in e {
        let entry = er.expect("Cannot Read Entry!");
        let name = entry.file_name();
        println!("{:?}", name);
    }
}
