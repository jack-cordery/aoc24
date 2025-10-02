use std::collections::HashMap;

/// We are given some topographical maps that represent some height [0,9]
/// we have a concept of a hiking trail that is a route that uses
/// left, right, up, and down moves and uses all of the posible heights
/// For each zero (trailhead) we need to count how many hiking routes
/// there are and sum them over the whole map
///

/// A Move represents an action on a map
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Route {
    positions: Vec<Position>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidMove,
}

impl Route {
    /// initalises with a trailhead position (height=0)
    pub fn new(trailhead: Position) -> Self {
        let positions = vec![trailhead];
        Route { positions }
    }

    /// actions a move
    pub fn action_move(&self, m: &Move, map: &Map) -> Result<Self, Error> {
        let last_pos = self.positions.last().unwrap().to_owned();
        let current_value = map.grid.get(&last_pos).unwrap().value();
        let mut result = Self {
            positions: self.positions.clone(),
        };
        match m {
            Move::Left => {
                if last_pos.x == 0 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x - 1;
                let new_y = last_pos.y;
                let new_pos = Position::new(new_x, new_y);
                let next_value = map.grid.get(&new_pos).unwrap().value();
                println!("current {} and next {}", current_value, next_value);
                if (next_value > current_value) && (next_value - current_value == 1) {
                    result.positions.push(new_pos);
                    Ok(result)
                } else {
                    Err(Error::InvalidMove)
                }
            }
            Move::Right => {
                if last_pos.x == map.max_x - 1 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x + 1;
                let new_y = last_pos.y;
                let new_pos = Position::new(new_x, new_y);
                let next_value = map.grid.get(&new_pos).unwrap().value();
                if (next_value > current_value) && (next_value - current_value == 1) {
                    result.positions.push(new_pos);
                    Ok(result)
                } else {
                    Err(Error::InvalidMove)
                }
            }
            Move::Up => {
                if last_pos.y == 0 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x;
                let new_y = last_pos.y - 1;
                let new_pos = Position::new(new_x, new_y);
                let next_value = map.grid.get(&new_pos).unwrap().value();
                if (next_value > current_value) && (next_value - current_value == 1) {
                    result.positions.push(new_pos);
                    Ok(result)
                } else {
                    Err(Error::InvalidMove)
                }
            }
            Move::Down => {
                if last_pos.y == map.max_y - 1 {
                    return Err(Error::InvalidMove);
                }
                let new_x = last_pos.x;
                let new_y = last_pos.y + 1;
                let new_pos = Position::new(new_x, new_y);
                let next_value = map.grid.get(&new_pos).unwrap().value();
                if (next_value > current_value) && (next_value - current_value == 1) {
                    result.positions.push(new_pos);
                    Ok(result)
                } else {
                    Err(Error::InvalidMove)
                }
            }
        }
    }
}

/// A map is a grid with heights filling each position
#[derive(Debug, PartialEq)]
pub struct Map {
    grid: HashMap<Position, Height>,
    trailheads: Vec<Position>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    /// read in a map from a vec grid which is the input
    pub fn read(vec_grid: Vec<Vec<u8>>) -> Self {
        let mut hmap: HashMap<Position, Height> = HashMap::new();
        let mut trailheads = Vec::new();

        for (j, row) in vec_grid.iter().enumerate() {
            for (i, element) in row.iter().enumerate() {
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
            max_y: vec_grid.len(),
            max_x: vec_grid.first().unwrap().len(),
        }
    }
}

/// take a vector of routes and calculate the next step for each
/// if the next step is invalid, we drop the whole route.
/// this is done inplace
pub fn iterate_routes(routes: Vec<Route>, map: &Map) -> Vec<Route> {
    let mut result = vec![];
    let steps = [Move::Left, Move::Right, Move::Up, Move::Down];
    for route in routes.iter() {
        for s in steps.iter() {
            match route.action_move(s, map) {
                Ok(r) => result.push(r),
                Err(_) => continue,
            }
        }
    }
    result
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Position {
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
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Height(u8);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_height() {
        let expected = Some(Height(3));
        let actual = Height::new(&3);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_new_invalid_height() {
        let expected = None;
        let actual = Height::new(&10);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_new_position() {
        let expected = Position { x: 1, y: 2 };
        let actual = Position::new(1, 2);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_new_route() {
        let expected = Route {
            positions: vec![Position::new(3, 4)],
        };
        let actual = Route::new(Position::new(3, 4));
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_action_move_left() {
        let map = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ]);
        let actual = Route::new(Position::new(2, 2)).action_move(&Move::Left, &map);
        let expected_positons = vec![Position::new(2, 2), Position::new(1, 2)];
        let expected = Route {
            positions: expected_positons,
        };
        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn test_action_move_right() {
        let map = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ]);
        let actual = Route::new(Position::new(1, 1)).action_move(&Move::Right, &map);
        let expected_positons = vec![Position::new(1, 1), Position::new(2, 1)];
        let expected = Route {
            positions: expected_positons,
        };
        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn test_action_move_down() {
        let map = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ]);
        let actual = Route::new(Position::new(1, 0)).action_move(&Move::Down, &map);
        let expected_positons = vec![Position::new(1, 0), Position::new(1, 1)];
        let expected = Route {
            positions: expected_positons,
        };
        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn test_action_move_up() {
        let map = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 8, 6, 5],
            vec![9, 7, 7, 6],
        ]);
        let actual = Route::new(Position::new(1, 3)).action_move(&Move::Up, &map);
        let expected_positons = vec![Position::new(1, 3), Position::new(1, 2)];
        let expected = Route {
            positions: expected_positons,
        };
        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn test_action_move_out_of_bounds() {
        let map = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ]);
        let actual = Route::new(Position::new(3, 0)).action_move(&Move::Right, &map);
        assert_eq!(Err(Error::InvalidMove), actual)
    }

    #[test]
    fn test_map_read() {
        let actual = Map::read(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ]);
        let mut expected_hmap = HashMap::new();
        expected_hmap.insert(Position::new(0, 0), Height(0));
        expected_hmap.insert(Position::new(1, 0), Height(1));
        expected_hmap.insert(Position::new(2, 0), Height(2));
        expected_hmap.insert(Position::new(3, 0), Height(3));

        expected_hmap.insert(Position::new(0, 1), Height(1));
        expected_hmap.insert(Position::new(1, 1), Height(2));
        expected_hmap.insert(Position::new(2, 1), Height(3));
        expected_hmap.insert(Position::new(3, 1), Height(4));

        expected_hmap.insert(Position::new(0, 2), Height(8));
        expected_hmap.insert(Position::new(1, 2), Height(7));
        expected_hmap.insert(Position::new(2, 2), Height(6));
        expected_hmap.insert(Position::new(3, 2), Height(5));

        expected_hmap.insert(Position::new(0, 3), Height(9));
        expected_hmap.insert(Position::new(1, 3), Height(8));
        expected_hmap.insert(Position::new(2, 3), Height(7));
        expected_hmap.insert(Position::new(3, 3), Height(6));

        let expected = Map {
            grid: expected_hmap,
            trailheads: vec![Position::new(0, 0)],
            max_x: 4,
            max_y: 4,
        };

        assert_eq!(expected, actual);
    }

    // TODO:
    // [X] Map Read
    // [X] Route action move
    // [] iterate steps
}
