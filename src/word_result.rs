use std::fmt;

pub struct WordResult {
    word: String,
    points: usize,
    swaps: Vec<Swap>
}

pub struct Swap {
    original_char: char,
    new_char: char,
    row: usize,
    col: usize
}

impl WordResult {
    pub fn new(word: String, points: usize, swaps: Vec<Swap>) -> WordResult {
        WordResult {
            word,
            points,
            swaps
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn points(&self) -> usize {
        self.points
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

