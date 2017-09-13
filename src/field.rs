use std::fmt;
use rand::{Rng, thread_rng};

pub struct Field {
    rows: usize,
    cols: usize,
    mines: Vec<usize>,
    nb_mines: usize,
    remaining_mines: usize
}

impl Field {
    pub fn new(rows: usize, cols: usize, nb_mines: usize) -> Result<Field, &'static str> {
        if nb_mines > rows * cols {
            return Err("Too much mines!");
        }

        let mut field = Field {
            rows,
            cols,
            mines: vec![0; rows * cols],
            nb_mines,
            remaining_mines: nb_mines
        };

        // Set the random mines
        field.initialize_mines();

        Ok(field)
    }

    pub fn is_included(&self, x: usize, y: usize) -> bool {
        x < self.cols && y < self.rows
    }

    pub fn at(&self, x: usize, y: usize) -> Option<usize> {
        if !self.is_included(x, y) {
            return None;
        }

        Some(self.mines[self.rows * y + x])
    }

    pub fn select(&mut self, x: usize, y: usize) -> Option<bool> {
        if !self.is_included(x, y) {
            return None;
        }

        let is_ready_to_explode = self.mines[self.rows * y + x] == 9;
        if is_ready_to_explode {
            // Compute new map
            self.remaining_mines -= 1;
        }
        Some(is_ready_to_explode)
    }

    fn set(&mut self, x: usize, y: usize, value: usize) -> Result<(), &'static str> {
        if !self.is_included(x, y) {
            return Err("x and y outside the mine field!");
        }
        self.mines[self.rows * y + x] = value;
        Ok(())
    }

    fn initialize_mines(&mut self) {
        let mut rng = thread_rng();

        for _ in 0..self.nb_mines {
            let x: usize = rng.gen_range(0, self.cols);
            let y: usize = rng.gen_range(0, self.rows);
            if let Err(_) = self.set(x, y, 9) {
                panic!("Unable to initialize field!")
            };
        }
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "Field:".to_string();

        for (i, mine) in self.mines.iter().enumerate() {
            if i % self.rows == 0 {
                str.push('\n');
            }
            str.push_str(&format!("{} ", mine));
        }

        write!(f, "{}", str)
    }
}
