use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WordResult {
    word: String,
    points: usize,
    spaces: Vec<Space>,
    swaps: Vec<Swap>
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    original_char: char,
    new_char: char,
    row: usize,
    col: usize
}

#[derive(Serialize, Deserialize)]
pub struct Space {
    char: char,
    row: usize,
    col: usize
}

impl WordResult {
    pub fn new(word: String, points: usize, swaps: Vec<Swap>, spaces: Vec<Space>) -> WordResult {
        WordResult {
            word,
            points,
            spaces,
            swaps
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn spaces(&self) -> &Vec<Space> {
        &self.spaces
    }

    pub fn swaps(&self) -> &Vec<Swap> {
        &self.swaps
    }
}

impl Swap {
    pub fn new(original_char: char, new_char: char, row: usize, col: usize) -> Swap {
        Swap {
            original_char,
            new_char,
            row,
            col
        }
    }
}

impl fmt::Display for Swap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Swap {} for {} at [{}, {}]", self.original_char, self.new_char, self.row, self.col)
    }
}

impl Space {
    pub fn new(char: char, row: usize, col: usize) -> Space {
        Space {
            char,
            row,
            col
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

