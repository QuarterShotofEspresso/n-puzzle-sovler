mod game;

use game::NPuzzle;

pub enum Heuristic {
    UniformCost,
    Manhattan,
    MisplacedTile,
}

pub fn calc_heuristic(target_game: &NPuzzle,
        heuristic: Heuristic) -> i32 {
    // take the current NPuzzle instance and return its distance to
    // the goal state
    match heuristic {
        Heuristic::Manhattan => {
            let mut expected_index = (0, 0);
            let mut expected_tile  = 1;
            let mut heur = 0;

            for row in target_game.data {
                for tile in row {
                    println!("{} (tile) != {} (expec)", tile, expected_tile);
                    if tile != 0 && tile != expected_tile {
                    //if tile != expected_tile {
                        let index_actual = target_game.find_tile_by_value(expected_tile);
                        let x_delta = (index_actual.0 as i32 - expected_index.0 as i32).abs();
                        println!("{} (act) - {} (exp) = {} (x)", index_actual.0, expected_index.0, x_delta);
                        let y_delta = (index_actual.1 as i32 - expected_index.1 as i32).abs();
                        println!("{} (act) - {} (exp) = {} (y)", index_actual.1, expected_index.1, y_delta);
                        heur += x_delta + y_delta;
                        println!("heur: {}", heur);
                    }
                    expected_index.1 = (expected_index.1 + 1) % target_game.data.len();
                    expected_tile    = (expected_tile + 1)    % (target_game.data.len() * target_game.data.len()) as i32;
                }
                expected_index.0 += 1;
            }
            heur
        },
        Heuristic::MisplacedTile => {
            let mut expected_tile = 1;
            let mut heur = -1; // Start at -1 because final element will by default be wrong 

            for row in target_game.data {
                for tile in row {
                    if tile != expected_tile {
                        heur += 1;
                    }
                    expected_tile += 1;
                }
            }
            heur
        },
        Heuristic::UniformCost => 0,
    }
}

pub struct Node {
    state: Option<NPuzzle>,
    dist: i32,
    cost: i32,
}

impl Node {

    // Transform a node into a game
    pub fn new(target_game: NPuzzle, dist: i32) -> Self {
        target_game.print();
        let cost = calc_heuristic(&target_game, Heuristic::Manhattan) + dist.clone();
        Node {
            state: Some(target_game),
            dist: dist.clone(),
            cost: cost, 
        }
    }


    // Take the current node and expand it by
    // considering all its operations
    // and generating new nodes for each instance
    pub fn expand_node(&self) -> Vec<Node> {
        let mut new_nodes: Vec<Node> = Vec::new();
        let valid_moves = match &self.state {
            Some(state) => state.valid_moves(),
            None => panic!("Empty Game has no moves!"),
        };
        for valid_move in &valid_moves {
            let new_game = match &self.state {
                Some(state) => state.move_empty_tile(valid_move),
                None => panic!("Empty Game has no moves!"),
            };
            new_nodes.push(
                Node::new(new_game, self.dist + 1)
            );
            
        }

        new_nodes
    }

    /*
    // Take a node and solve the puzzle from its current state
    pub fn solve(&self) -> Option<Node> {
        let mut queue: Vec<Node> = Vec::new();
        let starting_node = Node{state: None, dist: 0, cost: i32::MAX};
        while !queue.is_empty() {
            // find the node with the lowest cost
            let mut selected_node: &Node = queue.iter().fold(
                starting_node, |min, x| if x.cost < min.cost {x} else {min});
            let mut selected_node_idx: i32 = queue.iter().fold(
                0, |min_idx, x| if x.cost < queue[min_idx].cost {x} else {min_idx});
            //let selected_node = self.find_min();
            println!("Current game:");
            selected_node.print_node();
            // If the cost of the current node is 0
            if selected_node.cost == 0 {
                // It is the goal node
                println!("Found the goal state!\n");
                Some(selected_node) // so return the goal node
            }
            // otherwise expand the node and push the children onto the queue
            queue.push(selected_node.expand_node());
        }

        println!("No solution found!");
        None
    }
    */

    pub fn print(&self) {
        //self.state.unwrap().print();
        match &self.state {
            Some(state) => {
                println!("Dist: {}", self.dist);
                println!("Cost: {}", self.cost);
                state.print();
            },
            None => println!("Dummy Node!"),
        };
    }
        
}

/*
pub fn find_min(&self) -> i32 {
    let min_idx = 0;
    for (element, iter_idx) in self.space.iter().enumerate() {
        if (element.heur + element.dist) < (self.space[min_idx].heur + self.space[min_idx].dist) {
            min_idx = iter_idx;
        }
    }
    min_idx
}
}
*/

fn main() {
    let my_game = NPuzzle::new();
    my_game.print();
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
        /*let secondary_expansions = expanded_node.expand_node();
        for secondary_expanded in &secondary_expansions {
            secondary_expanded.print();
        }*/
    }
}
