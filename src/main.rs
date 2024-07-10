use spellcaster_solver::word_tree::WordTree;
use spellcaster_solver::board::Board;

fn main() {
    let tree = WordTree::new("./data/dictionaries/dictionary.txt");
    let board = Board::build_board_from_file("./data/boards/basic_board.txt");

    println!("{}", board);

    let longest = board.get_longest_word(&tree);
    let best = board.get_best_word(&tree);

    println!("Longest: {} for {} points", longest.0, longest.1);
    println!("Best: {} for {} points", best.0, best.1);

    if best.1 < 30 {
        println!("\nRecommendation: Reshuffle.");
    } 
}



