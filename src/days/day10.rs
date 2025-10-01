use std::collections::HashMap;

/// We are given some topographical maps that represent some height [0,9]
/// we have a concept of a hiking trail that is a route that uses
/// left, right, up, and down moves and uses all of the posible heights
/// For each zero (trailhead) we need to count how many hiking routes
/// there are and sum them over the whole map
///

/// A Move represents an action on a map
enum Move {
    Left,
    Right,
    Up,
    Down,
}

struct Route {
    positions: Vec<Position>,
    max_x: usize,
    max_y: usize,
}

enum Error {
    InvalidMove,
}

impl Route {
    /// initalises with a trailhead position (height=0)
    pub fn new(trailhead: Position, max_x: usize, max_y: usize) -> Self {
        let positions = vec![trailhead];
        Route {
            positions,
            max_x,
            max_y,
        }
    }

    /// actions a move
    pub fn action_move(&mut self, m: Move) -> Result<(), Error> {
        let last_pos = self.positions.last().unwrap().to_owned();
        match m {
            Move::Left => {
                if last_pos.x == 0 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x - 1;
                let new_y = last_pos.y;
                let new_pos = Position::new(new_x, new_y);
                self.positions.push(new_pos);
                Ok(())
            }
            Move::Right => {
                if last_pos.x == self.max_x {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x + 1;
                let new_y = last_pos.y;
                let new_pos = Position::new(new_x, new_y);
                self.positions.push(new_pos);
                Ok(())
            }
            Move::Up => {
                if last_pos.y == 0 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x;
                let new_y = last_pos.y - 1;
                let new_pos = Position::new(new_x, new_y);
                self.positions.push(new_pos);
                Ok(())
            }
            Move::Down => {
                if last_pos.y == self.max_y {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x;
                let new_y = last_pos.y + 1;
                let new_pos = Position::new(new_x, new_y);
                self.positions.push(new_pos);
                Ok(())
            }
        }
    }
}

/// A map is a grid with heights filling each position
struct Map {
    grid: HashMap<Position, Height>,
    trailheads: Vec<Position>,
}

impl Map {
    /// read in a map from a vec grid which is the input
    pub fn read(vec_grid: Vec<Vec<u8>>) -> Self {
        let mut hmap: HashMap<Position, Height> = HashMap::new();
        let mut trailheads = Vec::new();
        for (i, row) in vec_grid.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                let pos = Position::new(i, j);
                let height = Height::new(element).unwrap();
                hmap.insert(pos, height);
                if *element == 0 {
                    trailheads.push(pos);
                }
            }
        }
        Self {
            grid: hmap,
            trailheads,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// Position initialisation from two usizes
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Height is a possible value on the map it can be [0-9]
struct Height(u8);

impl Height {
    /// Create a height value ensuring it is 0-9
    pub fn new(value: &u8) -> Option<Self> {
        if *value < 10 {
            Some(Self(*value))
        } else {
            None
        }
    }

    /// Get the underlying height value
    pub fn value(self) -> u8 {
        self.0
    }
}
