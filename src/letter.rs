use std::collections::HashMap;

pub struct Letter{
    pub character: char,
    modifiers: Vec<Modifier>,
}

pub struct LetterPos {
    row: usize,
    col: usize
}

#[derive(Clone)]
pub enum Modifier{
    Default,
    DoubleLetter,
    TripleLetter,
    DoubleWord
}



impl Letter{
    pub fn new(character: char, modifiers: &Vec<Modifier>) -> Letter{
        let new_vec: Vec<Modifier> = modifiers.to_vec();
        Letter { character, modifiers: new_vec}
    }

    pub fn modifiers(&self) -> &Vec<Modifier> {
        &self.modifiers
    }

    pub fn build_letter_from_input_word(word: &str) -> Letter {
        let first_char = word.chars().nth(0).unwrap();
        if !(word.chars().count() > 1) {
            return Letter { character: first_char, modifiers: Vec::new() };
        }

        let mut modifiers: Vec<Modifier> = Vec::new();

        let vals = word.split('|');


        for val in vals.skip(1) {
            if (val == "dl") {
                modifiers.push(Modifier::DoubleLetter);
            }

            if (val == "tl") {
                modifiers.push(Modifier::TripleLetter);
            }

            if (val == "dw") {
                modifiers.push(Modifier::DoubleWord);
            }

        }

        Letter { character: first_char, modifiers }
    }
}

pub fn get_letter_value_map() -> HashMap<char, usize> {
    HashMap::from([
        ('a', 1),
        ('b', 4),
        ('c', 5),
        ('d', 3),
        ('e', 1),
        ('f', 5),
        ('g', 3),
        ('h', 4),
        ('i', 1),
        ('j', 7),
        ('k', 6),
        ('l', 3),
        ('m', 4),
        ('n', 2),
        ('o', 1),
        ('p', 4),
        ('q', 8),
        ('r', 2),
        ('s', 2),
        ('t', 2),
        ('u', 4),
        ('v', 5),
        ('w', 5),
        ('x', 7),
        ('y', 4),
        ('z', 8)
    ])
}