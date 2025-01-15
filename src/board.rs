use std::fmt;
use crate::{letter::{Letter, Modifier}, word_tree::WordTree};
use std::fs::read_to_string;
use crate::word_tree::*;
use crate::letter;
use std::thread;
use crossbeam::{self, thread::ScopedJoinHandle};

pub struct Board {
    size: usize,
    grid: Vec<Vec<Letter>>,
    swaps: usize
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
    col: usize,
    swaps: usize
}


impl Board{
    pub fn build_board_from_file(filename: &str) -> Board {

        Board::build_board_from_str(&read_to_string(filename).unwrap())
    }

    // TODO: Remove any panics.
    pub fn build_board_from_str(board: &str) -> Result<Board, String>  {
        let mut board_vec: Vec<Vec<Letter>> = Vec::new();
        let size = board.lines().count();

        for line in board.lines() {
            let mut new_row = Vec::new();
            for space in line.split_ascii_whitespace() {
                match Letter::build_letter_from_input_word(space) {
                    Ok(letter) => new_row.push(letter),
                    // For now just pretend it's fine unless this causes
                    // huge issues. On user to notice if it's wrong
                    Err(e) => ()
                }
            }
            board_vec.push(new_row);
        }

        Ok(Board {size, grid: board_vec, swaps: 0})
    }

    pub fn get_longest_word(&self, tree: &WordTree) -> (String, usize) {

        let words = self.get_all_possible_words(tree);
        let mut longest = String::from("ap");
        let mut longest_cells: &Vec<LetterSpace> = &Vec::new();

        for word in &words {
            if word.len() > longest.len() {
                longest = get_word_from_letter_spaces(&word);
                longest_cells = &word;
            }
        }

        let total = self.get_point_total(longest_cells);

        (longest, total)
    }

    pub fn get_best_word(&self, tree: &WordTree) -> (String, usize) {
        let best = self.get_best_word_spaces(tree);
        let word = best.0;
        let word_str = get_word_from_letter_spaces(&word);
        let grid = &self.grid;

        let mut result = format!("Best: {0} for {1} points\n", word_str, best.1);


        print_letter_spaces_for_word(&word);
        
        for letter in word {
            let original = grid[letter.row][letter.col].character;
            
            if letter.character() != original {
                result.push_str(&format!("Replacement of {0} with {1} at [{2}, {3}]\n", original, letter.character(), letter.row + 1, letter.col + 1));
            }
        }

        

        (result, best.1)
    }

    pub fn get_best_word_string(&self, tree: &WordTree) -> (String, usize) {
        let result = self.get_best_word_spaces(tree);

        (get_word_from_letter_spaces(&result.0), result.1)
    }

    fn get_best_word_spaces(&self, tree: &WordTree) -> (Vec<LetterSpace>, usize) {
        let words = self.get_all_possible_words_threaded(tree);
        
        let mut highest_point_total = 0;
        let mut point_total_temp;
        let mut cur_word: &Vec<LetterSpace> = &Vec::new();

        for word in &words {
            point_total_temp = self.get_point_total(&word);

            if point_total_temp > highest_point_total {
                highest_point_total = point_total_temp;
                cur_word = &word;
            }
        }

        (cur_word.to_vec(), highest_point_total)
    }


    pub fn set_swaps(&mut self, swaps: usize) {
        self.swaps = swaps;
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

    pub fn get_all_possible_words(&self, tree: &WordTree) -> Vec<Vec<LetterSpace>> {
        let mut word_list: Vec<Vec<LetterSpace>> = Vec::new();


        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                word_list.append(&mut self.get_all_words_from_pos(tree, i, j, self.swaps));
            }
        }

        return word_list;
    }

    /*
        Kind of complicated. Iterate through each cell on the board,
        and start a traversal through all possible letter combinations.
        The way this works is as follows: 
     */
    pub fn get_all_possible_words_threaded(&self, tree: &WordTree) -> Vec<Vec<LetterSpace>> {
        let mut word_list: Vec<Vec<LetterSpace>> = Vec::new();

        crossbeam::scope(|scope| {
            let mut handles: Vec<ScopedJoinHandle<Vec<Vec<LetterSpace>>>> = Vec::new();

            for i in 0..self.grid.len() {
                for j in 0..self.grid[i].len() {
                    handles.push(scope.spawn(move |_| {
                        self.get_all_words_from_pos(tree, i, j, self.swaps)
                    }));
                }
            }

            for handle in handles {
                word_list.append(&mut handle.join().unwrap());
            }


        }).unwrap();


        
        //word_list.append(&mut self.get_all_words_from_pos(tree, 0,  0));

        return word_list;
    }

    /*
        First: simple and slow implementation, use is_word_in_tree to check on all words,
        then change to tree traversal with stack

        This really needs to be refactored
     */
    fn get_all_words_from_pos(&self, tree: &WordTree, start_row: usize, start_col: usize, swaps: usize) -> Vec<Vec<LetterSpace>> {
        let mut stack: DoubleStack<StackElement> = DoubleStack::new();
        let mut words: Vec<Vec<LetterSpace>> = Vec::new();
        let mut cur_word_grid: Vec<LetterSpace> = Vec::new();
        let grid = &self.grid;
        let mut cur_node = tree.get_root_node();


        stack.push_new_layer(StackElement::RemoveOp(cur_node));

        stack.push_simple(StackElement::LetterStep( LetterSpace {
            character: self.grid[start_row][start_col].character,
            row: start_row,
            col: start_col,
            swaps
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
                        col: cell.col,
                        swaps: cell.swaps()
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
                                col: new_col,
                                swaps: cell.swaps
                            });

                            let mut is_in_so_far = false;

                            for letter in &cur_word_grid {
                                if letter.row == new_row && letter.col == new_col {
                                    is_in_so_far = true;
                                }
                            }

                            /*
                                Meat of letter swaps. For every neighbor, if we have a swap on this cell, add every child
                                of the current node to the stack at the position of the neighbor, simulating traversal
                                as if we made that swap.
                             */
                            if cell.swaps > 0 && !is_in_so_far{
                                add_swap_elements(cur_node, &mut stack, new_row, new_col, cell.swaps - 1);
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

    pub fn swaps(&self) -> usize {
        self.swaps
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

fn add_swap_elements(node: &LetterNode, stack: &mut DoubleStack<StackElement>, row: usize, col: usize, swaps: usize) {
    let children = node.children();

    for child in children {
        stack.push_simple(StackElement::LetterStep(LetterSpace {
            character: match child.letter() {
                LetterState::Present(character) => character.clone(),
                LetterState::Root => panic!("This should never happen.")
            },
            row,
            col,
            swaps
        }));
    }
}

fn print_letter_spaces_for_word(word: &Vec<LetterSpace>) {
    for letter in word {
        println!("{0} at {1}, {2}", letter.character(), letter.row() + 1, letter.col() + 1);
    }
}
