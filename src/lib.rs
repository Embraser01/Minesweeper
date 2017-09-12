use std::error::Error;


pub struct Config {
    pub query: String,
    pub difficulty: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // ...snip...
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let difficulty = args[2].parse().expect("Not a number !");

        Ok(Config { query, difficulty })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Hello, world!");
    Ok(())
}