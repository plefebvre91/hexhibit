
use std::env;
mod hx;

fn main() {

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // One argument is required
    if args.len() != 2 {
        println!("Usage: hexhibit <file>")
    } else {
        hx::dump(&args[1]);
    }
}
