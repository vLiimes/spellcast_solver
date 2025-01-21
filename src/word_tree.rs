use std::fmt;
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WordTree {
    root: LetterNode,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LetterNode{
    letter: LetterState,
    children: Vec<LetterNode>,
    state: NodeState,
    level: usize

}

#[derive(Serialize, Deserialize, Clone)]
pub enum LetterState{
    Present(char),
    Root,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NodeState{
    WordEnd,
    WordMiddle
}

impl WordTree {
    pub fn build_from_file(file_name: &str) -> WordTree {
        WordTree::build_from_str(&read_to_string(file_name).unwrap())
    }

    pub fn build_from_str(input: &str) -> WordTree {
        let root = LetterNode{
            letter: LetterState::Root,
            children: Vec::new(),
            state: NodeState::WordMiddle,
            level: 0
        };

        let mut tree  = WordTree{
            root: root
        };

        let words = input.lines();

        for word in words{
            tree.add_word_to_tree(&word.to_lowercase());
        }

        tree
    }

    /*
        Starting at root node, traverse character by character until
        at end of word
     */
    fn add_word_to_tree(&mut self, word: &str){
        let char_list = word.chars();

        let mut cur = &mut self.root;

        let mut char_counter = 1;

        let char_count = char_list.to_owned().count();
    
        let mut depth: usize = 1;

        /* 
            For every character in the source word,
            check if it's in the tree already. If not,
            add to the tree by constructing a new Node
            and pushing it to the current Node's
            children.

            If already in, go on to next character
        */
        for character in char_list{
            /*
                Is there a way I can avoid calling the is char in twice?
                Probably not much of a performance hit but I'm curious more
                than anything
             */
            match cur.get_char_in_children(character){
                // Some: Already in
                Some(_) => (),
                // None: Not in tree
                None => {
                    if char_counter == char_count{
                        cur.children.push(LetterNode { 
                            letter: LetterState::Present(character), 
                            children: Vec::new(), 
                            state: NodeState::WordEnd,
                            level: depth
                        });
                    }
                    else{
                        cur.children.push(LetterNode { 
                            letter: LetterState::Present(character), 
                            children: Vec::new(), 
                            state: NodeState::WordMiddle,
                            level: depth
                        });
                    }
                }
            }

            char_counter += 1;
            depth += 1;

            cur = cur.get_char_in_children(character).unwrap();
        }
    }

    pub fn is_word_in_tree(&self, word: &str) -> bool {
        // Get the children underneath the current root node
        let mut children = &self.root.children;

        let mut letter_found = false;
        let mut last_node = &children[0];
        
        
        for letter in word.chars(){
            for node in children {
                match node.letter {
                    LetterState::Present(character) => { 
                        if character == letter {
                            letter_found = true;
                            children = &node.children;
                            last_node = node;
                            break;
                        }
                    },
                    LetterState::Root => ()
                }
            }

            if !letter_found {
                return false;
            }
            letter_found = false;

        }

        match last_node.state {
            NodeState::WordEnd => true,
            NodeState::WordMiddle => false
        }

    }

    pub fn get_root_node(&self) -> &LetterNode {
        &self.root
    }

}

impl fmt::Display for WordTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let root = &self.root;

        let mut i = 1;

        let mut tree_str = String::new();

        // FIX THIS IT NEEDS TO PRINT CORRECTLY
        while root.children.len() > 0{
            for letter in &root.children{
                match letter.letter{
                    LetterState::Present(character) => { tree_str.push_str(&format!(" ({i}) {character}")) },
                    LetterState::Root => ()
                }
            }
            tree_str.push('\n');
            i = i + 1;
        }

        write!(f, "{tree_str}")

    }
}

impl LetterNode {

    pub fn letter(&self) -> &LetterState {
        &self.letter
    }

    pub fn children(&self) -> &Vec<LetterNode> {
        &self.children
    }

    pub fn state(&self) -> &NodeState {
        &self.state
    }

    pub fn level(&self) -> usize {
        self.level
    }

    /*
        Runs through the given node's children and searches for the provided
        character

        Takes a mutable reference to the node, and returns either a mutable
        reference to its child or None (no child with character found)

     */
    fn get_char_in_children(&mut self, character: char) -> Option<&mut LetterNode> {

        // For each child in the list of children
        for child in &mut self.children{
            // Match on the child's LetterState enum
            match child.letter{
                // If present, do some comparison
                LetterState::Present(this_char) => { 
                    if this_char == character{
                        return Some(child);
                    }
                    continue
                },
                // Otherwise, keep going
                LetterState::Root => continue
            };
        }

        None
    }

    pub fn get_child_from_letter(&self, character: char) -> Option<&LetterNode>{
        for child in &self.children {
            match child.letter {
                LetterState::Present(letter) => {
                    if letter == character {
                        return Some(&child);
                    }
                    continue
                },
                LetterState::Root => continue
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_in_tree_simple(){
        let tree = WordTree::build_from_file("./data/dictionaries/test_1.txt");
        

        assert!(tree.is_word_in_tree("obfuscate"));
        assert!(tree.is_word_in_tree("apple"));
        assert!(!tree.is_word_in_tree("creaturue"));
        assert!(!tree.is_word_in_tree("obf"));
    }

    #[test]
    fn test_words_big(){
        let tree =  WordTree::build_from_file("./data/dictionaries/dictionary.txt");

        assert!(tree.is_word_in_tree("finch"));
        assert!(tree.is_word_in_tree("gold"));
        assert!(tree.is_word_in_tree("oxidize"));
        assert!(tree.is_word_in_tree("xylophone"));
        assert!(!tree.is_word_in_tree("yotemyscrote"));
        assert!(!tree.is_word_in_tree("huiafafj"));
        assert!(!tree.is_word_in_tree("yugoslavia"));
        assert!(tree.is_word_in_tree("homer"));
        assert!(!tree.is_word_in_tree("o"));
    }
}