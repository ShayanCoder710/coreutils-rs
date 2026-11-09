use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Example: cp [Source] [Destination]");
        return;
    } else if args[1] == "-r" {
        if args.len() < 4 {
            eprintln!("Example: cp -r [Source] [Destination]");
            return;
        }
        let src = Path::new(&args[2]);
        let dest = Path::new(&args[3]);

        if !dest.exists() {
            fs::create_dir(dest).expect("Cannot Create Dir!");
        }

        for entry in fs::read_dir(src).expect("Cannot Read Dir!") {
            let entry = entry.expect("Cannot Read Entry!");
            let src_file = entry.path();
            let dest_file = dest.join(entry.file_name());

            if src_file.is_file() {
                fs::copy(&src_file, &dest_file).expect("Cannot Copy File!");
            }
        }
    } else {
        fs::copy(&args[1], &args[2]).expect("Cannot Copy File!");
    }
}
