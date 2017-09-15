use std::fmt;

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
        if self.visible {
            write!(f, "{}", self.value)
        } else {
            write!(f, "-")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_clone() {
        let mut first_cell = Cell {
            value: 2,
            visible: false
        };

        let second_cell = first_cell.clone();

        assert_eq!(first_cell.value, second_cell.value);
        assert_eq!(first_cell.visible, second_cell.visible);
        first_cell.value += 1;

        assert_ne!(first_cell.value, second_cell.value);
    }

    #[test]
    fn value_must_be_invisible() {
        let cell = Cell {
            value: 9,
            visible: false
        };

        assert_eq!(format!("{}", cell), "-")
    }

    #[test]
    fn value_must_be_visible() {
        let cell = Cell {
            value: 9,
            visible: true
        };

        assert_eq!(format!("{}", cell), "9")
    }
}