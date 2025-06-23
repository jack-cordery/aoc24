use std::cmp::PartialEq;
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

#[derive(PartialEq, Clone, Debug)]
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
        let current_x = self.guard.x;
        let current_y = self.guard.y;
        let current_dir = self.guard.direction.clone();

        match next_move {
            Move::Rotate => {
                self.guard.rotate();

                let new_dir = self.guard.direction.clone();
                self.grid[current_y as usize][current_x as usize] = Tile::Current(new_dir);
            }
            Move::Finish => {
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

        match self.guard.direction {
            Direction::Up => {
                if self.guard.y > 0 {
                    if self.grid[(self.guard.y - 1) as usize][self.guard.x as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Up;
                }
                Move::Finish
            }

            Direction::Down => {
                if (self.guard.y as usize) < self.grid.len() - 1 {
                    if self.grid[(self.guard.y + 1) as usize][self.guard.x as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Down;
                }
                Move::Finish
            }

            Direction::Left => {
                if self.guard.x > 0 {
                    if self.grid[self.guard.y as usize][(self.guard.x - 1) as usize]
                        == Tile::Obstacle
                    {
                        return Move::Rotate;
                    }
                    return Move::Left;
                }
                Move::Finish
            }
            Direction::Right => {
                if (self.guard.x as usize) < self.grid.first().unwrap().len() - 1 {
                    if self.grid[self.guard.y as usize][(self.guard.x + 1) as usize]
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
        }
        self.positions_visited
    }
}

#[derive(Debug, PartialEq)]
struct Guard {
    x: u16,
    y: u16,
    direction: Direction,
}

impl Guard {
    fn new(x: u16, y: u16, direction: Direction) -> Self {
        Guard { x, y, direction }
    }

    fn walk(&mut self) -> (u16, u16) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
        (self.x, self.y)
    }

    fn rotate(&mut self) {
        self.direction.rotate();
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
    let unique_pos = map.solve();
    println!(
        "unique postions {} in {}us",
        unique_pos,
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
        assert_eq!(10, guard.x);
        assert_eq!(20, guard.y);
        assert_eq!(Direction::Up, guard.direction);
    }

    #[test]
    fn test_guard_rotate() {
        let mut guard = Guard::new(10, 20, Direction::Up);
        guard.rotate();
        assert_eq!(Direction::Right, guard.direction);
        guard.rotate();
        assert_eq!(Direction::Down, guard.direction);
        guard.rotate();
        assert_eq!(Direction::Left, guard.direction);
        guard.rotate();
        assert_eq!(Direction::Up, guard.direction);
    }

    #[test]
    fn test_test_guard_walk() {
        let mut guard = Guard::new(2, 2, Direction::Up);
        guard.walk();
        assert_eq!(2, guard.x);
        assert_eq!(1, guard.y);
        guard.rotate();
        guard.rotate();
        guard.rotate();
        guard.walk();
        assert_eq!(1, guard.y);
        assert_eq!(1, guard.y);
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
    fn test_map_step() {
        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '#'],
            vec!['.', '.', '.'],
        ];
        let mut map = Map::new(grid);

        map.step();

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', '>', '#'],
            vec!['.', '.', '.'],
        ]);
        let expected_guard = Guard::new(1, 1, Direction::Right);
        let expected_completed = false;
        let expected_positions_visited = 1;

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);

        map.step();

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', 'v', '#'],
            vec!['.', '.', '.'],
        ]);
        let expected_guard = Guard::new(1, 1, Direction::Down);
        let expected_completed = false;
        let expected_positions_visited = 1;

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);

        map.step();

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', 'X', '#'],
            vec!['.', 'v', '.'],
        ]);
        let expected_guard = Guard::new(1, 2, Direction::Down);
        let expected_completed = false;
        let expected_positions_visited = 2;

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);

        map.step();

        let expected_grid = Map::from_char_grid(vec![
            vec!['.', '#', '.'],
            vec!['.', 'X', '#'],
            vec!['.', 'X', '.'],
        ]);
        let expected_guard = Guard::new(1, 2, Direction::Down);
        let expected_completed = true;
        let expected_positions_visited = 2;

        assert_eq!(expected_grid, map.grid);
        assert_eq!(expected_guard, map.guard);
        assert_eq!(expected_completed, map.completed);
        assert_eq!(expected_positions_visited, map.positions_visited);
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

        let grid = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '.'],
            vec!['#', '.', '<'],
        ];

        let mut map = Map::new(grid);
        let expected_solved = 4;
        assert_eq!(expected_solved, map.solve());
    }
}
