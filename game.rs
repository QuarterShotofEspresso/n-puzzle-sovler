use std::vec::Vec;

pub enum Heuristic {
    UniformCost,
    Manhattan,
    MisplacedTile,
}

const HEURISTIC: Heuristic = Heuristic::Manhattan;
const N: usize = 3;

pub struct NPuzzle {
    pub data: [[i32; N]; N],
}

impl NPuzzle {

    pub fn new(data_to_map: &[i32]) -> Self {
        
        if data_to_map.len() != (N * N) {
            panic!("Can't map the provided data!");
        }

        let mut new_game_data = [[0; N]; N];
        //let mut value = 1;
        let mut data_map_idx = 0;

        for i in 0..N {
            for j in 0..N {
                //new_game_data[i][j] = value;
                new_game_data[i][j] = data_to_map[data_map_idx];
                data_map_idx += 1;
                //value += 1;
            }
        }

        //new_game_data[N-1][N-1] = 0;

        NPuzzle {data: new_game_data}
    }

    pub fn valid_moves(&self) -> Vec<(usize, usize)> {
        // Take the current game state and return a vec of (usize, usize)
        // that describe all valid moves that can be taken
        // NOTE: The tuple of ints represent the indices of the game board
        let empty_tile_idx: (usize, usize) = self.find_tile_by_value(0);
        let mut swappable_idx: Vec<(usize, usize)> = Vec::new();
        // if we can move the tile up
        if empty_tile_idx.0 as i32 - 1 >= 0 {
            // then generate a tuple and add it to the ret slice
            swappable_idx.push((empty_tile_idx.0 - 1, empty_tile_idx.1));
        }
        // check if we can move the tile down
        if empty_tile_idx.0 as i32 + 1 < N as i32 {
            // then generate a tuple and add it to the ret slice
            swappable_idx.push((empty_tile_idx.0 + 1, empty_tile_idx.1));
        }
        // if we can move the tile right
        if empty_tile_idx.1 as i32 + 1 < N as i32 {
            // then generate a tuple and add it to the ret slice
            swappable_idx.push((empty_tile_idx.0, empty_tile_idx.1 + 1));
        }
        // check if we can move the tile left
        if empty_tile_idx.1 as i32 - 1 >= 0 {
            // then generate a tuple and add it to the ret slice
            swappable_idx.push((empty_tile_idx.0, empty_tile_idx.1 - 1));
        }

        swappable_idx
    }

    pub fn find_tile_by_value(&self, target: i32) -> (usize, usize) {
        // Given a game state, return the index at where a value is
        for (i, row) in self.data.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == target {
                    return (i, j);
                }
            }
        }

        println!("Could not find index for cell {}", &target);
        return (0, 0);
    }

    pub fn swap_tiles(&self, tile_a: &(usize, usize), tile_b: &(usize, usize)) -> NPuzzle {
        // create a new NPuzzle instance with respective tiles swapped
        let mut new_game_data = [[-1; N]; N];
        for (i, row) in new_game_data.iter_mut().enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                // if we're at the index as that of tile A
                if (i, j) == *tile_a {
                    *tile = self.data[tile_b.0][tile_b.1];
                } else if (i, j) == *tile_b {
                    *tile = self.data[tile_a.0][tile_a.1];
                } else {
                    *tile = self.data[i][j];
                }
            }
        }

        NPuzzle {data: new_game_data}
    }

    pub fn move_empty_tile(&self, tile_a: &(usize, usize)) -> NPuzzle {
        let empty_tile = self.find_tile_by_value(0);
        let new_game = self.swap_tiles(&empty_tile, &tile_a);
        new_game
    }

    pub fn print(&self) {
        // Print the game state
        for row in self.data {
            println!("{:?}", row);
        }
    }

    pub fn calc_heuristic(&self) -> i32 {
        // take the current NPuzzle instance and return its distance to
        // the goal state
        match HEURISTIC {
            Heuristic::Manhattan => {
                let mut expected_index = (0, 0);
                let mut expected_tile  = 1;
                let mut heur = 0;

                for row in self.data {
                    for tile in row {
                        //println!("{} (tile) != {} (expec)", tile, expected_tile);
                        if expected_tile != 0 && tile != expected_tile {
                        //if tile != expected_tile {
                            let index_actual = self.find_tile_by_value(expected_tile);
                            let x_delta = (index_actual.0 as i32 - expected_index.0 as i32).abs();
                            //println!("{} (act) - {} (exp) = {} (x)", index_actual.0, expected_index.0, x_delta);
                            let y_delta = (index_actual.1 as i32 - expected_index.1 as i32).abs();
                            //println!("{} (act) - {} (exp) = {} (y)", index_actual.1, expected_index.1, y_delta);
                            heur += x_delta + y_delta;
                            //println!("heur: {}", heur);
                        }
                        expected_index.1 = (expected_index.1 + 1) % self.data.len();
                        expected_tile    = (expected_tile + 1)    % (self.data.len() * self.data.len()) as i32;
                    }
                    expected_index.0 += 1;
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
}

pub struct Node {
    state: NPuzzle,
    dist: i32,
    cost: i32,
}

impl Node {

    // Transform a node into a game
    pub fn new(target_game: NPuzzle, dist: i32) -> Self {
        //target_game.print();
        let cost = target_game.calc_heuristic() + dist.clone();
        Node {
            state: target_game,
            dist: dist.clone(),
            cost: cost, 
        }
    }


    // Take the current node and expand it by
    // considering all its operations
    // and generating new nodes for each instance
    pub fn expand_node(&self) -> Vec<Node> {
        let mut new_nodes: Vec<Node> = Vec::new();
        let valid_moves = self.state.valid_moves();
        for valid_move in &valid_moves {
            let new_game = self.state.move_empty_tile(valid_move);
            new_nodes.push(Node::new(new_game, self.dist + 1));
        }

        new_nodes
    }

    // Take a node and solve the puzzle from its current state
    pub fn solve(self) -> Option<Node> {
        let mut queue: Vec<Node> = Vec::new();
        let mut total_nodes_visited: u32 = 0;
        queue.push(self);
        while !queue.is_empty() {
            // find the node with the lowest cost
            let min_idx = queue.iter()
                .enumerate()
                .min_by_key(|(_, s)| s.cost)
                .map(|(i, _)| i)
                .expect("Couldn't find a node with the lowest cost");
            // collect that node
            let selected_node = queue.swap_remove(min_idx);
            selected_node.print();
            total_nodes_visited += 1;
            // If the cost of the current node is 0
            if (selected_node.cost - selected_node.dist) == 0 {
                // It is the goal node
                println!("Found the goal state!\nTotal nodes visited: {}", total_nodes_visited);
                return Some(selected_node); // so return the goal node
            }
            // otherwise expand the node and push the children onto the queue
            queue.append(&mut selected_node.expand_node());
        }

        println!("No solution found!");
        None
    }

    pub fn print(&self) {
        self.state.print();
        println!("Dist: {}", self.dist);
        println!("Heur: {}", self.cost - self.dist);
        println!("Cost: {}", self.cost);
        println!();
    }
}
