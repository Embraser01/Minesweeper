use std::error::Error;

pub struct Config {
    pub difficulty: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // ...snip...
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let difficulty: u32 = match args[1].parse() {
            Ok(diff) => diff,
            Err(_) => return Err("Not a number!")
        };

        Ok(Config { difficulty })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Hello, world! Difficulty set to {}", config.difficulty);
    Ok(())
}