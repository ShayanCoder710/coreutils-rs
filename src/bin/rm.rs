use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Example: rm [File_Name] OR rm -r [Dir_Name]");
    } else if args[1] == "-r" {
        if args.len() < 3 {
            eprintln!("Error: You need to specify a directory name.");
            return;
        }
        fs::remove_dir_all(&args[2]).expect("Cannot Remove Dir!");
    } else {
        fs::remove_file(&args[1]).expect("Cannot Remove File!");
    }
}
