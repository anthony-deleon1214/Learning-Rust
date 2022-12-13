// Importing standard library functions
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
