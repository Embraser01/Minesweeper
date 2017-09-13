use std::fmt;
use std::cmp::{min, max};
use rand::{Rng, thread_rng};

const MINE_VALUE: usize = 9;

pub struct Cell {
    pub value: usize,
    pub visible: bool
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            value: self.value,
            visible: self.visible
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
        // TODO Check visible
    }
}

pub struct Field {
    rows: usize,
    cols: usize,
    mines: Vec<Cell>,
    nb_mines: usize
}

impl Field {
    pub fn new(rows: usize, cols: usize, nb_mines: usize) -> Result<Field, &'static str> {
        if nb_mines > rows * cols {
            return Err("Too much mines!");
        }

        let mut field = Field {
            rows,
            cols,
            mines: vec![Cell { value: 0, visible: false }; rows * cols],
            nb_mines
        };

        // Set the random mines
        field.initialize_mines();

        Ok(field)
    }

    pub fn is_included(&self, x: usize, y: usize) -> bool {
        x < self.cols && y < self.rows
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&Cell> {
        if !self.is_included(x, y) {
            return None;
        }

        Some(&self.mines[self.rows * y + x])
    }

    fn at_mutable(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.mines[self.rows * y + x]
    }

    pub fn on_click(&mut self, x: usize, y: usize) -> bool {
        if !self.is_included(x, y) {
            return false;
        }

        let cell = self.at_mutable(x, y);

        if cell.visible {
            return false;
        }

        cell.visible = true;

        // If there is no mine near me, show neighbour's cell
        if cell.value == 0 {
            self.on_click(x + 1, y);
            self.on_click(x - 1, y);
            self.on_click(x, y + 1);
            self.on_click(x, y - 1);
        }

        return cell.value == MINE_VALUE;
    }

    fn initialize_mines(&mut self) {
        let mut rng = thread_rng();
        let mut remaining = self.nb_mines;

        while remaining > 0 {
            let x: usize = rng.gen_range(0, self.cols);
            let y: usize = rng.gen_range(0, self.rows);

            // Unwrap safe, fail only if x or y is not in correct ranges (not possible)
            if self.at(x, y).unwrap().value == MINE_VALUE {
                continue;
            }

            // Increment neighbours cells and set mine
            for i in max(0, x as isize - 1) as usize..min(self.cols, x + 2) {
                for j in max(0, y as isize - 1) as usize..min(self.rows, y + 2) {
                    let mut cell = self.at_mutable(i, j);
                    if i == x && j == y {
                        cell.value = MINE_VALUE;
                    } else if cell.value != MINE_VALUE {
                        cell.value += 1;
                    }
                }
            }

            remaining -= 1;
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
