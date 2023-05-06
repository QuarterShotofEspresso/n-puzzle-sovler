use std::vec::Vec;

const N: usize = 3;

pub enum Heuristic {
    UniformCost,
    Manhattan,
    MisplacedTile
}

pub fn make_game() -> Game {
    
    let mut new_game_data = [[0; N]; N];
    let mut value = 1;

    for i in 0..N {
        for j in 0..N {
            new_game_data[i][j] = value;
            value += 1;
        }
    }

    new_game_data[N-1][N-1] = 0;

    Game {data: new_game_data, dist: 0}
}

pub struct Game {
    data: [[i32; N]; N],
    dist: i32
}

impl Game {
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
            for (j, &tile) in row.iter().enumerate() {
                if tile == target {
                    return (i, j);
                }
            }
        }

        println!("Could not find index for cell {}", target);
        return (0, 0);
    }

    pub fn swap_tiles(&self, tile_a: &(usize, usize), tile_b: &(usize, usize)) -> Game {
        // create a new Game instance with respective tiles swapped
        let mut new_game_data = [[-1; N]; N];
        for (i, row) in self.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                // if we're at the index as that of tile A
                if (i, j) == *tile_a {
                    new_game_data[i][j] = self.data[tile_a.0][tile_a.1];
                } else if (i, j) == *tile_b {
                    new_game_data[i][j] = self.data[tile_b.0][tile_b.1];
                } else {
                    new_game_data[i][j] = self.data[i][j];
                }
            }
        }

        Game {data: new_game_data, dist: -1}
    }

    pub fn move_empty(&self, tile_a: &(usize, usize)) -> Game {
        let empty_tile = self.find_tile_by_value(0);
        let mut new_game = self.swap_tiles(&empty_tile, &tile_a);
        new_game.find_distance(Heuristic::UniformCost);
        new_game
    }

    pub fn find_distance(&mut self, heuristic: Heuristic) -> i32 {
        // take the current Game instance and return its distance to
        // the goal state
        self.dist = match heuristic {
            Heuristic::Manhattan => 0, // TODO: Implement manhattan heursitic
            Heuristic::MisplacedTile => 0, // TODO: Implement misplaced heuristic
            Heuristic::UniformCost => 0
        };

        self.dist
    }

    pub fn print_game(&self) {
        // Print the game state
        for row in self.data.iter() {
            for tile in row.iter() {
                print!("{} ", tile);
            }
            println!();
        }
        println!();
    }
}