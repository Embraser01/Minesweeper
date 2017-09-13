extern crate minesweeper;

use std::env;
use std::process;
use minesweeper::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minesweeper::run(config) {
        println!("Problem while running the program: {}", e);
    }
}
