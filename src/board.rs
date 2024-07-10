use std::fmt;
use crate::{letter::{Letter, Modifier}, word_tree::WordTree};
use std::fs::read_to_string;
use crate::word_tree::*;
use crate::letter;


pub struct Board {
    size: usize,
    grid: Vec<Vec<Letter>>
}

struct DoubleStack<T : Copy> {
    stack: Vec<Vec<T>>
}

#[derive(Clone, Copy)]
enum StackElement<'a> {
    LetterStep(LetterSpace),
    RemoveOp(&'a LetterNode)
}

#[derive(Clone, Copy)]
pub struct LetterSpace {
    character: char,
    row: usize,
    col: usize
}


impl Board{
    pub fn make_example_board() -> Board{
        let letters = vec![
            vec!['i', 'i', 'e', 'k',  'm'],
            vec!['a', 'n', 'l', 'e', 'r'],
            vec!['t', 'e', 'c', 'a', 'z'],
            vec!['n', 'g', 'o', 's', 'n'],
            vec!['r', 'o', 'r', 'l', 'f']
        ];
    
        let mut letter_data = Vec::new();
    
        for row in letters{
    
            let mut new_row = Vec::new();
    
            for letter in row{
                new_row.push(Letter::new(letter, &vec![Modifier::Default]));
            }
    
            letter_data.push(new_row);
        }
    
        Board{size: 5, grid: letter_data}
    }

    pub fn build_board_from_file(filename: &str) -> Board {
        let mut board_vec: Vec<Vec<Letter>> = Vec::new();
        let read = read_to_string(filename).unwrap();
        let size = read.lines().count();

        for line in read.lines() {
            let mut new_row = Vec::new();
            for space in line.split(' ') {
                new_row.push(Letter::build_letter_from_input_word(space));
            }
            board_vec.push(new_row);
        }

        Board {size, grid: board_vec}
    }

    pub fn get_longest_word(&self, tree: &WordTree) -> (String, usize) {

        let words = self.get_all_possible_words(tree);
        let mut longest = String::from("ap");

        for word in words {
            if word.len() > longest.len() {
                longest = get_word_from_letter_spaces(&word);
            }
        }

        let total = get_point_total_str(&longest);

        (longest, total)
    }

    pub fn get_best_word(&self, tree: &WordTree) -> (String, usize) {
        let words = self.get_all_possible_words(tree);
        
        let mut highest_point_total = 0;
        let mut point_total_temp;
        let mut cur_word = String::from("ap");

        for word in words {
            point_total_temp = self.get_point_total(&word);

            if point_total_temp > highest_point_total {
                highest_point_total = point_total_temp;
                cur_word = get_word_from_letter_spaces(&word);
            }
        }

        (cur_word, highest_point_total)
    }

    fn get_point_total(&self, word: &Vec<LetterSpace>) -> usize {
        let letter_score_map = letter::get_letter_value_map();
        let mut points: usize = 0;
        let mut double_word = false;
    
        for letter in word {
            let mut to_add = *letter_score_map.get(&letter.character.to_ascii_lowercase()).unwrap();

            let grid_letter = &self.grid[letter.row()][letter.col()];
            for modifier in grid_letter.modifiers() {
                match modifier {
                    Modifier::DoubleLetter => {
                        to_add = to_add * 2;
                    },

                    Modifier::TripleLetter => {
                        to_add = to_add * 3;
                    }

                    Modifier::DoubleWord => {
                        double_word = true;
                    }

                    _ => ()
                }
            }

            points += to_add;
        }

        if double_word {
            points *= 2;
        }

        // Points for long word are not doubled
        if word.len() >= 6 {
            points += 10;
        }
    
        points
    }

    /*
        Kind of complicated. Iterate through each cell on the board,
        and start a traversal through all possible letter combinations.
        The way this works is as follows: 
     */
    pub fn get_all_possible_words(&self, tree: &WordTree) -> Vec<Vec<LetterSpace>> {
        let mut word_list: Vec<Vec<LetterSpace>> = Vec::new();


        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                word_list.append(&mut self.get_all_words_from_pos(tree, i, j));
            }
        }
        //word_list.append(&mut self.get_all_words_from_pos(tree, 0,  0));

        return word_list;
    }

    /*
        First: simple and slow implementation, use is_word_in_tree to check on all words,
        then change to tree traversal with stack
     */
    fn get_all_words_from_pos(&self, tree: &WordTree, start_row: usize, start_col: usize) -> Vec<Vec<LetterSpace>> {
        let mut stack: DoubleStack<StackElement> = DoubleStack::new();
        let mut words: Vec<Vec<LetterSpace>> = Vec::new();
        let mut cur_word_grid: Vec<LetterSpace> = Vec::new();
        let grid = &self.grid;
        let mut cur_node = tree.get_root_node();


        stack.push_new_layer(StackElement::RemoveOp(cur_node));

        stack.push_simple(StackElement::LetterStep( LetterSpace {
            character: self.grid[start_row][start_col].character,
            row: start_row,
            col: start_col
        }));

        let pos_mods: Vec<isize> = vec![-1, 0, 1];

        while !stack.is_empty() {
            match stack.pop() {
                /*
                    If letter step, update our current word so far, both in 
                    string form and grid locations form. Then add any all neighbors
                    to this space that are within range and NOT already in the
                    cur_word_grid.
                 */
                StackElement::LetterStep(cell) => { 

                    let old_cur = cur_node;
                    cur_word_grid.push( LetterSpace {
                        character: cell.character,
                        row: cell.row,
                        col: cell.col
                    });

                    let temp_word = &get_word_from_letter_spaces(&cur_word_grid);

                    if tree.is_word_in_tree(temp_word) {
                        words.push(cur_word_grid.clone());
                    }

                    match cur_node.get_child_from_letter(cell.character) {
                        Some(new_node) => {
                            cur_node = new_node;
                        },
                        None => panic!("This shouldn't happen")
                    }
                    
                    let mut frame_flag = false;

                    for pos_mod_row in &pos_mods {
                        for pos_mod_col in &pos_mods {
                            let row = cell.row;
                            let col = cell.col;

                            let possib_row: isize = row as isize + pos_mod_row;
                            let possib_col: isize = col as isize + pos_mod_col;

                            if possib_row < 0 || possib_row > self.size as isize - 1 {
                                continue
                            }

                            if possib_col < 0 || possib_col > self.size as isize - 1 {
                                continue
                            }

                            let new_row: usize = usize::try_from(row as isize + pos_mod_row).unwrap();
                            let new_col: usize = usize::try_from(col as isize + pos_mod_col).unwrap();

                            if !frame_flag {
                                stack.push_new_layer(StackElement::RemoveOp(old_cur));
                                frame_flag = true;
                            }

                            match cur_node.get_child_from_letter(grid[new_row][new_col].character) {
                                Some(_) => (),
                                None => continue
                            }

                            let to_add = StackElement::LetterStep(LetterSpace {
                                character: grid[new_row][new_col].character,
                                row: new_row,
                                col: new_col
                            });

                            let mut is_in_so_far = false;

                            for letter in &cur_word_grid {
                                if letter.character() == grid[new_row][new_col].character
                                && letter.row == new_row && letter.col == new_col {
                                    is_in_so_far = true;
                                }
                            }

                            if !is_in_so_far {
                                stack.push_simple(to_add);
                            }
                        }
                    }
                },

                StackElement::RemoveOp(parent) => {
                    cur_word_grid.pop();
                    cur_node = parent;
                }
            }
        }

        words
    }
}

impl<T: Copy> DoubleStack<T> {
    pub fn new() -> DoubleStack<T> {
        DoubleStack {
            stack: vec![Vec::new()]
        }
    }

    /*
        Simplest case, push onto whatever is the most recent
        stack frame
     */
    pub fn push_simple(&mut self, value: T) {
        let top_index = self.stack.len() - 1;
        
        let top = &mut self.stack[top_index];

        top.push(value);
    }

    /*
        Push the value by creating a new stack frame, and 
        add it as an element
     */
    pub fn push_new_layer(&mut self, value: T) {
        let new_layer = vec![value];

        if self.stack.len() == 1 && self.stack[0].len() == 0 {
            self.stack.pop();
        }

        self.stack.push(new_layer);
    }

    /*
        No case for wanting to pop an entire frame at once,
        since it may lose data.

        When a frame loses all elements, want to destroy
        the stack frame of empty elements.
     */
    pub fn pop(&mut self) -> T {
        let top_index = self.stack.len() - 1;

        let val = self.stack[top_index].pop().unwrap();

        // If that made stack frame empty, remove it
        if self.stack[top_index].len() == 0 {
            self.stack.pop();
        }

        val
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() <= 0
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl LetterSpace {
    pub fn character(&self) -> char {
        self.character
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl fmt::Display for Board{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut board_str = String::new();

        for row in &self.grid{
            for letter in row{
                board_str.push_str(&format!(" {}", letter.character.to_ascii_uppercase()));
                for modifier in letter.modifiers() {
                    match modifier {
                        Modifier::DoubleLetter => {
                            board_str.push_str(&format!("({}) ", "DL"));
                        },

                        Modifier::TripleLetter => {
                            board_str.push_str(&format!("({}) ", "TL"));
                        },

                        Modifier::DoubleWord => {
                            board_str.push_str(&format!("({}) ", "DW"));
                        },

                        Modifier::Default => ()
                    }
                }
            }

            board_str.push('\n');

        }

        write!(f, "{board_str}")
        
    }
}

impl<T: Copy + fmt::Display> fmt::Display for DoubleStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack_str = String::new();

        let mut i = 0;
        for frame in &self.stack {
            stack_str.push_str(&format!("LAYER: {i} ["));
            for item in frame {
                stack_str.push_str(&format!("{item}, "));
            }
            stack_str.push_str("] \n");

            i = i + 1;
        }

        write!(f, "{stack_str}")
    }
}

impl fmt::Display for StackElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackElement::LetterStep(letter) => write!(f, "[{} {} {}]", letter.character(), letter.row(), letter.col()),
            StackElement::RemoveOp(_) => write!(f, "[EMPTY]")
        }
    }
}

fn get_word_from_letter_spaces(letters: &Vec<LetterSpace>) -> String{
    let mut result = String::new();

    for letter in letters {
        result.push(letter.character);
    }
    result
}

fn get_point_total_str(word: &str) -> usize {
    let letter_score_map = letter::get_letter_value_map();
    let mut points: usize = 0;

    for letter in word.chars() {
        points += letter_score_map.get(&letter).unwrap();
    }

    points
}