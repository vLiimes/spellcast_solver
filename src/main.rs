use spellcast_solver::word_tree::WordTree;
use spellcast_solver::board::Board;
use std::io;

fn main() {
    let tree = WordTree::build_from_file("./data/dictionaries/dictionary.txt");
    let mut board = Board::build_board_from_file("./data/boards/basic_board.txt");

    println!("Number of swaps?");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read");
    let num: usize = num.trim().parse().unwrap();
    board.set_swaps(num);

    let longest = board.get_longest_word(&tree);
    let best = board.get_best_word(&tree);
    let best_words = board.get_best_words(&tree, 10);

    println!("Longest: {} for {} points", longest.word(), longest.points());
    println!("Best: {} for {} points", best.word(), best.points());

    if best.points() < 30 {
        println!("Recommendation: Reshuffle.");
    }

    println!("\n10 Best words:");
    for word in best_words {
        println!("{} for {} points", word.word(), word.points());
    }


    println!("Press Enter to exit.");

    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).expect("failed to exit");
}



