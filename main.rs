mod game;

fn main() {
    let my_game = game::make_game();
    my_game.print_game();
    let empty_tile = my_game.find_tile_by_value(0);
    println!("Empty Tile @ {:?}", empty_tile);
    let games_valid_moves = my_game.valid_moves();
    println!("Valid Moves: {:?}", games_valid_moves);
    //let tile_a = (0, 0);
    //let tile_b = (1, 0);
    //my_game.swap_tiles(&tile_a, &tile_b).print_game();
    
    let mut new_games: Vec<game::Game> = Vec::new();
    for tile in games_valid_moves.iter() {
        new_games.push(my_game.move_empty_tile(tile));
    }

    for game in new_games {
        game.print_game();
        println!("Empty Tile @ {:?}", game.find_tile_by_value(0));
    }
    
    
}
