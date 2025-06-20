use std::cmp::PartialEq;

#[derive(PartialEq)]
enum Tile {
    Open,
    Obstacle,
    Visited,
    Current(Direction),
}

enum Move {
    Rotate,
    Up,
    Down,
    Left,
    Right,
    Finish,
}

#[derive(PartialEq)]
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

impl From<Tile> for char {
    fn from(tile: Tile) -> char {
        match tile {
            Tile::Open => '.',
            Tile::Obstacle => '#',
            Tile::Visited => 'X',
            Tile::Current(Direction::Up) => '^',
            Tile::Current(Direction::Down) => 'v',
            Tile::Current(Direction::Left) => '>',
            Tile::Current(Direction::Right) => '<',
        }
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    completed: bool,
    guard: Guard,
    positions_visited: u16,
}

impl Map {
    // TODO:
    // [] create a method that reads in the grid and initialises the guard
    // [] create a method that steps through until the guard moves off the board (until completed
    // is true)

    fn new(grid: Vec<Vec<Tile>>) -> Self {
        // we will want to construct this from the input string and find the guard to initialise
        // a guard on the map
        Map {
            grid,
            completed: false,
            guard: Guard::new(0, 0, Direction::Up),
            positions_visited: 0,
        }
    }

    fn step(&mut self) {
        // here we will update the map with the new iteration
        let next_move = self.calculate_next_step();
        let current_x = self.guard.x;
        let current_y = self.guard.y;

        match next_move {
            Move::Rotate => self.guard.rotate(),
            Move::Finish => self.completed = true,
            _ => {
                // the guard walks and the tiles of the map are updated and the postiitons is
                // updated if they havenet visited the new spot
                let (new_x, new_y) = self.guard.walk();
                if self.grid[new_x as usize][new_y as usize] == Tile::Open {
                    self.positions_visited += 1;
                    self.grid[new_x as usize][new_y as usize] = Tile::Visited;
                }
            }
        }
    }

    fn calculate_next_step(&self) -> Move {
        // take the guards current direction and check if there is a
        // obstacle in that direction if not move in that dierecti0on
        // account for off map

        match self.guard.direction {
            Direction::Up => {
                if self.guard.y > 1 {
                    return Move::Up;
                }
                Move::Finish
            }

            Direction::Down => {
                if (self.guard.y as usize) < self.grid.len() - 1 {
                    return Move::Down;
                }
                Move::Finish
            }

            Direction::Left => {
                if self.guard.x > 1 {
                    return Move::Left;
                }
                Move::Finish
            }
            Direction::Right => {
                if (self.guard.x as usize) < self.grid.first().unwrap().len() {
                    return Move::Right;
                }
                Move::Finish
            }
        }
    }
}

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
        // will move the guard one position in the direction it is facing
        match self.direction {
            Direction::Up => self.y - 1,
            Direction::Down => self.y + 1,
            Direction::Left => self.x - 1,
            Direction::Right => self.y + 1,
        };
        (self.x, self.y)
    }

    fn rotate(&mut self) {
        // will rotate the guard clockwise once
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }
}

// the next thing to do is take a step on a map.
// a step will take a map and then calculate the new map based on where the guard is and its
// surroundings
