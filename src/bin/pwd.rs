use std::env;

fn main() {
    let path = env::current_dir().expect("Error!");
    println!("{}", path.display());
}
