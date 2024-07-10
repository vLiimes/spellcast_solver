use spellcast_solver::word_tree::WordTree;
use spellcast_solver::board::Board;
use std::io;

fn main() {
    let tree = WordTree::new("./data/dictionaries/dictionary.txt");
    let mut board = Board::build_board_from_file("./data/boards/basic_board.txt");

    println!("Number of swaps?");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read");
    let num: usize = num.trim().parse().unwrap();
    board.set_swaps(num);

    let longest = board.get_longest_word(&tree);
    let best = board.get_best_word(&tree);

    println!("Longest: {} for {} points", longest.0, longest.1);
    println!("Best: {} for {} points\n", best.0, best.1);

    if best.1 < 30 {
        println!("Recommendation: Reshuffle.");
    } 
}



