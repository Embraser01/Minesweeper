// Third party lib
extern crate rand;

use std::error::Error;

mod field;

use field::Field;

pub struct Config {
    pub difficulty: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let difficulty: usize = match args[1].parse() {
            Ok(diff) => diff,
            Err(_) => return Err("Not a number!")
        };

        Ok(Config { difficulty })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Hello, world! Difficulty set to {}", config.difficulty);

    let main_field: Field = match Field::new(5, 5, 10) {
        Ok(field) => field,
        Err(e) => return Err(From::from(e)),
    };
    print!("{}", main_field);

    //    main_field.on_click(1,1);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}