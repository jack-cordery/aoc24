use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::read;
use std::io::BufRead;
use std::time::Instant;

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Open,
    Obstacle,
    Visited,
    Current(Direction),
}

#[derive(PartialEq, Clone, Debug)]
enum Move {
    Rotate,
    Up,
    Down,
    Left,
    Right,
    Finish,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
enum TileError {
    InvalidTile,
}

impl Tile {
    fn from_char(tile_char: &char) -> std::result::Result<Self, TileError> {
        match tile_char {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Obstacle),
            'X' => Ok(Tile::Visited),
            '^' => Ok(Tile::Current(Direction::Up)),
            'v' => Ok(Tile::Current(Direction::Down)),
            '<' => Ok(Tile::Current(Direction::Left)),
            '>' => Ok(Tile::Current(Direction::Right)),
            _ => Err(TileError::InvalidTile),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Open => '.',
            Tile::Obstacle => '#',
            Tile::Visited => 'X',
            Tile::Current(Direction::Up) => '^',
            Tile::Current(Direction::Down) => 'v',
            Tile::Current(Direction::Left) => '<',
            Tile::Current(Direction::Right) => '>',
        }
    }
}

#[derive(Debug)]
enum MapError {
    NoGuard,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Tile>>,
    completed: bool,
    guard: Guard,
    positions_visited: u16,
}

impl Map {
    fn new(char_grid: Vec<Vec<char>>) -> Self {
        // we will want to construct this from the input string and find the guard to initialise
        // a guard on the map

        let grid = Self::from_char_grid(char_grid);

        let guard = Self::find_guard(grid.clone()).unwrap();

        Map {
            grid,
            completed: false,
            guard,
            positions_visited: 1,
        }
    }

    fn from_char_grid(char_grid: Vec<Vec<char>>) -> Vec<Vec<Tile>> {
        let mut result = Vec::new();
        for row in char_grid {
            let row_tiles: Vec<Tile> = row.iter().map(|c| Tile::from_char(c).unwrap()).collect();
            result.push(row_tiles);
        }
        result
    }

    fn find_guard(grid: Vec<Vec<Tile>>) -> std::result::Result<Guard, MapError> {
        for (i, row) in grid.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Current(direction) => {
                        let direction = direction.clone();
                        return Ok(Guard::new(j as u16, i as u16, direction));
                    }
                    _ => continue,
                }
            }
        }
        Err(MapError::NoGuard)
    }

    fn step(&mut self) {
        // here we will update the map with the new iteration
        let next_move = self.calculate_next_step();
        let current_x = self.guard.position.x;
        let current_y = self.guard.position.y;
        let current_dir = self.guard.position.direction.clone();

        match next_move {
            Move::Rotate => {
                self.guard.rotate();

                let new_dir = self.guard.position.direction.clone();
                self.grid[current_y as usize][current_x as usize] = Tile::Current(new_dir);
            }
            Move::Finish => {
                self.guard.history.insert(self.guard.position.clone());
                self.grid[current_y as usize][current_x as usize] = Tile::Visited;
                self.completed = true;
            }
            _ => {
                // the guard walks and the tiles of the map are updated and the postiitons is
                // updated if they havenet visited the new spot
                let (new_x, new_y) = self.guard.walk();
                if self.grid[new_y as usize][new_x as usize] == Tile::Open {
                    self.positions_visited += 1;
                }
                self.grid[current_y as usize][current_x as usize] = Tile::Visited;
                self.grid[new_y as usize][new_x as usize] = Tile::Current(current_dir);
            }
        }
    }

    fn calculate_next_step(&self) -> Move {
        // take the guards current direction and check if there is a
        // obstacle in that direction if not move in that dierecti0on
        // account for off map

        match self.guard.position.direction {
            Direction::Up => {
                if self.guard.position.y > 0 {
                    if self.grid[(self.guard.position.y - 1) as usize]
                        [self.guard.position.x as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Up;
                }
                Move::Finish
            }

            Direction::Down => {
                if (self.guard.position.y as usize) < self.grid.len() - 1 {
                    if self.grid[(self.guard.position.y + 1) as usize]
                        [self.guard.position.x as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Down;
                }
                Move::Finish
            }

            Direction::Left => {
                if self.guard.position.x > 0 {
                    if self.grid[self.guard.position.y as usize]
                        [(self.guard.position.x - 1) as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Left;
                }
                Move::Finish
            }
            Direction::Right => {
                if (self.guard.position.x as usize) < self.grid.first().unwrap().len() - 1 {
                    if self.grid[self.guard.position.y as usize]
                        [(self.guard.position.x + 1) as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Right;
                }
                Move::Finish
            }
        }
    }

    fn print_map(&self) {
        // will just print out the current tiles as a map
        println!("-----------------STEP-----------------------");
        for row in &self.grid {
            let row_chars: String = row.iter().map(|tile| tile.to_char().to_string()).collect();
            println!("{row_chars}");
        }
        println!("--------------------------------------------");
    }

    fn solve(&mut self) -> u16 {
        while !self.completed {
            self.step();
            if self.guard.looped {
                self.completed = true;
            }
        }
        self.positions_visited
    }

    fn check_looped(&self) -> bool {
        // will return whether this map results in a loop
        let mut map_clone = self.clone();

        map_clone.solve();

        map_clone.guard.looped
    }

    fn find_loop_obstacle_pos(&self) -> u16 {
        // will iterate through putting a obstacle at all open positions and
        // return the number of configs for which that results in a loop

        let mut result = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile == &Tile::Open {
                    let mut map_clone = self.clone();
                    map_clone.grid[i][j] = Tile::Obstacle;
                    if map_clone.check_looped() {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: u16,
    y: u16,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Guard {
    position: Position,
    history: HashSet<Position>,
    looped: bool,
}

impl Guard {
    fn new(x: u16, y: u16, direction: Direction) -> Self {
        let history: HashSet<Position> = HashSet::new();
        let position = Position { x, y, direction };
        Guard {
            position,
            history,
            looped: false,
        }
    }

    fn walk(&mut self) -> (u16, u16) {
        self.history.insert(self.position.clone());
        match self.position.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        };
        match self.history.get(&self.position) {
            Some(_) => self.looped = true,
            None => self.looped = false,
        };
        (self.position.x, self.position.y)
    }

    fn rotate(&mut self) {
        self.history.insert(self.position.clone());
        self.position.direction.rotate();
        match self.history.get(&self.position) {
            Some(_) => self.looped = true,
            None => self.looped = false,
        };
    }
}

pub fn day_six(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let contents = read(path)?;
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut map = Map::new(grid);
    let map_clone = map.clone();
    let unique_pos = map.solve();
    let num_pos = map_clone.find_loop_obstacle_pos();
    println!(
        "unique postions {} and num_obstacles {} in {}us",
        unique_pos,
        num_pos,
        now.elapsed().as_micros(),
    );
    map.print_map();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_guard_new() {
        let guard = Guard::new(10, 20, Direction::Up);
        let expected_history: HashSet<Position> = HashSet::new();
        assert_eq!(10, guard.position.x);
        assert_eq!(20, guard.position.y);
        assert_eq!(Direction::Up, guard.position.direction);
        assert_eq!(expected_history, guard.history);
        assert!(!guard.looped);
    }

    #[test]
    fn test_guard_rotate() {
        let mut guard = Guard::new(10, 20, Direction::Up);

        guard.rotate();
        assert_eq!(Direction::Right, guard.position.direction);
        let expected_position = Position {
            x: 10,
            y: 20,
            direction: Direction::Up,
        };
        assert_eq!(
            guard.history.get(&expected_position),
            Some(&expected_position)
        );
        assert_eq!(guard.history.len(), 1);
        assert!(!guard.looped);

        guard.rotate();
        assert_eq!(Direction::Down, guard.position.direction);
        let expected_position = Position {
            x: 10,
            y: 20,
            direction: Direction::Right,
        };
        assert_eq!(
            guard.history.get(&expected_position),
            Some(&expected_position)
        );
        assert_eq!(guard.history.len(), 2);
        assert!(!guard.looped);

        guard.rotate();
        assert_eq!(Direction::Left, guard.position.direction);
        let expected_position = Position {
            x: 10,
            y: 20,
            direction: Direction::Down,
        };
        assert_eq!(
            guard.history.get(&expected_position),
            Some(&expected_position)
        );
        assert_eq!(guard.history.len(), 3);
        assert!(!guard.looped);

        guard.rotate();
        assert_eq!(Direction::Up, guard.position.direction);
        let expected_position = Position {
            x: 10,
            y: 20,
            direction: Direction::Left,
        };
        assert_eq!(
            guard.history.get(&expected_position),
            Some(&expected_position)
        );
        assert_eq!(guard.history.len(), 4);
        assert!(guard.looped);
    }

    #[test]
    fn test_test_guard_walk() {
        let mut guard = Guard::new(2, 2, Direction::Up);

        // up to 2, 1
        guard.walk();

        let mut expected_history = HashSet::new();
        let prev_pos = Position {
            x: 2,
            y: 2,
            direction: Direction::Up,
        };
        expected_history.insert(prev_pos);
        assert_eq!(expected_history, guard.history);
        assert_eq!(2, guard.position.x);
        assert_eq!(1, guard.position.y);

        // rotate to right 2,1
        guard.rotate();

        expected_history.insert(Position {
            x: 2,
            y: 1,
            direction: Direction::Up,
        });

        // rotate to down 2,1
        guard.rotate();

        expected_history.insert(Position {
            x: 2,
            y: 1,
            direction: Direction::Right,
        });

        // rotate to left 2, 1
        guard.rotate();

        expected_history.insert(Position {
            x: 2,
            y: 1,
            direction: Direction::Down,
        });

        // walk left to 1,1
        guard.walk();
        expected_history.insert(Position {
            x: 2,
            y: 1,
            direction: Direction::Left,
        });

        assert_eq!(1, guard.position.x);
        assert_eq!(1, guard.position.y);
        assert_eq!(Direction::Left, guard.position.direction);
        assert_eq!(expected_history, guard.history);
    }

    #[test]
    fn test_map_new() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];
        let map = Map::new(grid);

        let expected_grid = vec![
            vec![Tile::Open, Tile::Obstacle, Tile::Open],
            vec![Tile::Open, Tile::Current(Direction::Up), Tile::Obstacle],
            vec![Tile::Open, Tile::Open, Tile::Open],
        ];
        let expected_guard = Guard::new(1, 1, Direction::Up);
        let expected_completed = false;
        let expected_positions_visited = 1;

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);
    }

    #[test]
    fn test_map_from_char() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];
        let tile_grid = Map::from_char_grid(grid);

        let expected_grid = vec![
            vec![Tile::Open, Tile::Obstacle, Tile::Open],
            vec![Tile::Open, Tile::Current(Direction::Up), Tile::Obstacle],
            vec![Tile::Open, Tile::Open, Tile::Open],
        ];
        assert_eq!(expected_grid, tile_grid);
    }

    #[test]
    fn test_map_find_guard() {
        let tile_grid = vec![
            vec![Tile::Open, Tile::Obstacle, Tile::Open],
            vec![Tile::Open, Tile::Current(Direction::Up), Tile::Obstacle],
            vec![Tile::Open, Tile::Open, Tile::Open],
        ];
        let guard = Map::find_guard(tile_grid).unwrap();

        let expected_guard = Guard::new(1, 1, Direction::Up);
        assert_eq!(expected_guard, guard);
    }

    #[test]
    fn test_map_step_rotate() {
        //test that the step method rotates as expected
        let start_grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];
        let mut map = Map::new(start_grid);

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', '>', '#'],
            vec!['.', '.', '.'],
        ]);
        let mut expected_guard = Guard::new(1, 1, Direction::Right);
        let prev_pos = Position {
            x: 1,
            y: 1,
            direction: Direction::Up,
        };
        let mut clean_history = HashSet::new();
        clean_history.insert(prev_pos);
        expected_guard.history = clean_history.clone();
        let expected_completed = false;
        let expected_positions_visited = 1;

        map.step();

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);
    }

    #[test]
    fn test_map_step_walk() {
        let start_grid = vec![
            vec!['.', '#', '.'],
            vec!['.', 'v', '#'],
            vec!['.', '.', '.'],
        ];
        let mut map = Map::new(start_grid);

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', 'X', '#'],
            vec!['.', 'v', '.'],
        ]);
        let mut expected_guard = Guard::new(1, 2, Direction::Down);
        let prev_pos = Position {
            x: 1,
            y: 1,
            direction: Direction::Down,
        };
        let mut clean_history = HashSet::new();
        clean_history.insert(prev_pos);
        expected_guard.history = clean_history.clone();
        let expected_completed = false;
        let expected_positions_visited = 2;

        map.step();

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);
    }

    #[test]
    fn test_map_step_stores_history() {
        let start_grid = vec![
            vec!['.', '#', '.'],
            vec!['>', '.', '#'],
            vec!['.', '.', '.'],
        ];
        let mut map = Map::new(start_grid);

        let first_pos = Position {
            x: 0,
            y: 1,
            direction: Direction::Right,
        };
        let second_pos = Position {
            x: 1,
            y: 1,
            direction: Direction::Right,
        };
        let third_pos = Position {
            x: 1,
            y: 1,
            direction: Direction::Down,
        };
        let fourth_pos = Position {
            x: 1,
            y: 2,
            direction: Direction::Down,
        };
        let mut clean_history = HashSet::new();
        clean_history.insert(first_pos);
        clean_history.insert(second_pos);
        clean_history.insert(third_pos);
        clean_history.insert(fourth_pos);

        map.step();
        map.step();
        map.step();
        map.step();

        assert_eq!(clean_history, map.guard.history);
    }

    #[test]
    fn test_map_step_finish() {
        let start_grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['.', 'v', '.'],
        ];
        let mut map = Map::new(start_grid);

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['.', 'X', '.'],
        ]);
        let mut expected_guard = Guard::new(1, 2, Direction::Down);
        let prev_pos = Position {
            x: 1,
            y: 2,
            direction: Direction::Down,
        };
        let mut clean_history = HashSet::new();
        clean_history.insert(prev_pos);
        expected_guard.history = clean_history.clone();
        let expected_completed = true;
        let expected_positions_visited = 1;

        map.step();

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);
    }

    #[test]
    fn test_map_step_looped() {
        let start_grid = vec![
            vec!['.', '#', '.'],
            vec!['#', '^', '#'],
            vec!['.', '#', '.'],
        ];
        let mut map = Map::new(start_grid);

        map.step();
        assert!(!map.guard.looped);
        map.step();
        assert!(!map.guard.looped);
        map.step();
        assert!(!map.guard.looped);
        map.step();

        assert!(map.guard.looped);
    }

    #[test]
    fn test_map_calculate_step() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];
        let map = Map::new(grid);
        let m = map.calculate_next_step();

        let expected_m = Move::Rotate;

        assert_eq!(expected_m, m);

        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', 'v', '#'],
            vec!['.', '.', '.'],
        ];

        let map = Map::new(grid);
        let m = map.calculate_next_step();

        let expected_m = Move::Down;

        assert_eq!(expected_m, m);

        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['.', 'v', '.'],
        ];

        let map = Map::new(grid);
        let m = map.calculate_next_step();

        let expected_m = Move::Finish;

        assert_eq!(expected_m, m);
    }

    #[test]
    fn test_map_solve() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];

        let mut map = Map::new(grid);
        let expected_solved = 2;
        assert_eq!(expected_solved, map.solve());
        assert!(!map.guard.looped);
        assert!(map.completed);

        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '.'],
            vec!['#', '.', '<'],
        ];

        let mut map = Map::new(grid);
        let expected_solved = 4;
        assert_eq!(expected_solved, map.solve());
        assert!(!map.guard.looped);
        assert!(map.completed);
    }

    #[test]
    fn test_map_solve_looped() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['#', '^', '#'],
            vec!['.', '#', '.'],
        ];

        let mut map = Map::new(grid);
        let expected_solved = 1;
        assert_eq!(expected_solved, map.solve());
        assert!(map.guard.looped);
        assert!(map.completed);
    }

    #[test]
    fn test_map_check_looped() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['#', '^', '#'],
            vec!['.', '#', '.'],
        ];
        let grid_not = vec![
            vec!['.', '.', '.'],
            vec!['#', '^', '#'],
            vec!['.', '#', '.'],
        ];

        let map = Map::new(grid);
        let is_looped = map.check_looped();
        assert!(is_looped);

        let map = Map::new(grid_not);
        let is_looped = map.check_looped();
        assert!(!is_looped);
    }

    #[test]
    fn test_map_find_loop_obstacle_pos() {
        let grid = vec![
            vec!['.', '.', '.'],
            vec!['#', '^', '#'],
            vec!['.', '#', '.'],
        ];

        let map = Map::new(grid);
        let pos = map.find_loop_obstacle_pos();
        assert_eq!(1, pos);
    }

    #[test]
    fn test_map_find_loop_obstacle_pos_full() {
        let grid = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];

        let map = Map::new(grid);
        let pos = map.find_loop_obstacle_pos();
        assert_eq!(6, pos);
    }
}
