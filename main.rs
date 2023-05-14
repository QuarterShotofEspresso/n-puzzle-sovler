mod game;

use game::*;

fn main() {
    // FormatL game_<DEPTH>
    let game_8  = [1, 3, 6, 5, 0, 2, 4, 7, 8];
    let game_12 = [1, 3, 6, 5, 0, 7, 4, 8, 2];
    let game_16 = [1, 6, 7, 5, 0, 3, 4, 8, 2];
    let game_20 = [7, 1, 2, 4, 8, 5, 6, 3, 0];
    let game_24 = [0, 7, 2, 4, 6, 1, 3, 5, 8];

    let my_game = NPuzzle::new(&game_20);
    let new_node = Node::new(my_game, 0);
    println!("Initialized Puzzle:");
    new_node.print();
    println!("Commencing search...");

    let search_result = new_node.solve();
    match search_result {
        Some(solved_game) => {
            println!("Solution:");
            solved_game.print();
        },
        None => println!("No solution!"),
    };

    // Unit Tests
    /*my_game.print();
    let empty_tile = my_game.find_tile_by_value(0);
    println!("Empty Tile @ {:?}", empty_tile);
    let games_valid_moves = my_game.valid_moves();
    println!("Valid Moves: {:?}", games_valid_moves);
    
    let mut new_games: Vec<NPuzzle> = Vec::new();
    for tile in &games_valid_moves {
        new_games.push(my_game.move_empty_tile(tile));
    }

    for game in new_games {
        game.print();
        println!("Empty Tile @ {:?}", game.find_tile_by_value(0));
    }

    let my_node = Node::new(my_game, 0);
    let expansions = my_node.expand_node();
    for expanded_node in &expansions {
        expanded_node.print();
        let secondary_expansions = expanded_node.expand_node();
        for secondary_expanded in &secondary_expansions {
            secondary_expanded.print();
        }
    }*/
}
