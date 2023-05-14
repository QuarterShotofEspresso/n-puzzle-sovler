mod game;
mod search;

//use game::*;
//use search::Heuristic;
//use search::Node;

fn main() {
    let my_game = NPuzzle::new();
    my_game.print();
    let empty_tile = my_game.find_tile_by_value(0);
    println!("Empty Tile @ {:?}", empty_tile);
    let games_valid_moves = my_game.valid_moves();
    println!("Valid Moves: {:?}", games_valid_moves);
    
    let mut new_games: Vec<game::NPuzzle> = Vec::new();
    for tile in games_valid_moves.iter() {
        new_games.push(my_game.move_empty_tile(tile));
    }

    for game in new_games {
        game.print();
        println!("Empty Tile @ {:?}", game.find_tile_by_value(0));
    }

    //let my_node = Node::new(my_game, 0);
    //let expansions = my_node.expand_node();
    //for expanded_node in expansions {
    //    expanded_node.print();
    //}

}
