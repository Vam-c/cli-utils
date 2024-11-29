use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).expect("File name is required as first argument.");
    let string = args.get(2).expect("Provide a string to search as second argument.");
    println!("{file}, {string}");
}
