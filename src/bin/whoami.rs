use std::env;

fn main() {
    let u = env::var("USER").expect("USER Not Found!");
    println!("{}", u);
}
