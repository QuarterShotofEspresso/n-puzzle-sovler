mod game;

pub enum Heuristic {
    UniformCost,
    Manhattan,
    MisplacedTile
}

pub fn calc_heuristic(target_game: &game::Game,
        heuristic: Heuristic) -> i32 {
    // take the current Game instance and return its distance to
    // the goal state
    match heuristic {
        Heuristic::Manhattan => {
            let mut expected_index = (0, 0);
            let mut expected_tile  = 1;
            let mut heur = 0;

            for row in self.state.data {
                for tile in row {
                    if tile != expected_tile {
                        let index_actual = self.data.get_tile_by_value(tile);
                        heur += (index_actual.0 - expected_index.0).abs() +
                            (index_actual.1 - expected_index.1).abs();
                    }
                }
            }
            heur
        },
        Heuristic::MisplacedTile => {
            let mut expected_tile = 1;
            let mut heur = -1; // Start at -1 because final element will by default be wrong 

            for row in self.data {
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
    state: Option<game::Game>,
    dist: i32,
    cost: i32,
}

impl Node {

    // Transform a node into a game
    pub fn create_node_from_game(target_game: game::Game,
            dist_of_parent_node: i32) -> Node {
        Node {
            data: Some(target_game),
            dist: dist_of_parent_node.clone() + 1,
            cost: calc_heuristic(target_game, 
                heuristic::UniformCost) + dist_of_parent_node.clone() + 1,
        }
    }


    // Take the current node and expand it by
    // considering all its operations
    // and generating new nodes for each instance
    pub fn expand_node(&self) -> Vec<Node> {
        let new_nodes: Vec<Node> = Vec::new();
        let mut valid_moves = self.state.valid_moves();
        for valid_move in valid_moves {
            let new_game = self.state.move_empty_tile(valid_move);
            new_nodes.push(
                create_node_from_game(new_game, new_game.dist)
            );
        }

        new_nodes
    }

    // Take a node and solve the puzzle from its current state
    pub fn solve(&self) -> Option<Node> {
        let mut queue: Vec<Node> = Vec::new();
        let starting_node = Node{state: None, dist: 0, cost: i32::MAX};
        while !queue.is_empty() {
            // find the node with the lowest cost
            let mut selected_node: &Node = queue.iter().fold(
                starting_node, |min, x| if x.cost < min.cost {x} else {min});
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

    pub fn print_node(&self) {
        self.state.unwrap().print_game();
        match self.state {
            Some(state) => {
                state.print_game();
                println!("Cost: {}", state.cost);
            },
            None => println!("Dummy Node!"),
        }
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
