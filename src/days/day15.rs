// ok so here we want to read in a grid
// and a list of instructions with some rules of movement
// We have walls as #, robot is @ and box are 0
// box get pushed by robot
//
//

use std::{fs::read_to_string, time::Instant};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Robot,
    Box,
    Space,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Robot {
    pos: Position,
}

impl Robot {
    pub fn new(pos: Position) -> Self {
        Self { pos }
    }
}

#[derive(PartialEq, Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    robot: Robot,
}

impl Grid {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let mut pos = Position::new(0, 0);
        for (j, row) in tiles.iter().enumerate() {
            for (i, el) in row.iter().enumerate() {
                if el == &Tile::Robot {
                    pos = Position::new(i, j);
                }
            }
        }
        let robot = Robot::new(pos);
        Self { tiles, robot }
    }

    fn find_pos(&self, pos: Position, m: Move) -> Position {
        match m {
            Move::Up => Position::new(pos.x, pos.y - 1),
            Move::Down => Position::new(pos.x, pos.y + 1),
            Move::Left => Position::new(pos.x - 1, pos.y),
            Move::Right => Position::new(pos.x + 1, pos.y),
        }
    }

    pub fn make_move(&mut self, m: Move) {
        // so this needs to implement the logic that if its a space in that direction then we move
        // the robot there
        // if its a box, we check if the last box can move into a space so the box and robot move else nothing
        // moves
        // if its a wall nothing moves
        let curr_pos = self.robot.pos;
        let next = self.find_pos(curr_pos, m);
        let next_tile = self.tiles.get(next.y).unwrap().get(next.x).unwrap();

        match next_tile {
            Tile::Wall => {
                // do nothing
            }
            Tile::Space => {
                // move the robot from curr to next and make curr a space

                *self
                    .tiles
                    .get_mut(curr_pos.y)
                    .unwrap()
                    .get_mut(curr_pos.x)
                    .unwrap() = Tile::Space;
                *self.tiles.get_mut(next.y).unwrap().get_mut(next.x).unwrap() = Tile::Robot;
                self.robot.pos.x = next.x;
                self.robot.pos.y = next.y;
            }
            Tile::Box => {
                // look in the same direction until there is either a space or a wall
                // if wall dont do anything
                // if space move all the ways we have found across one
                // remember to make original a space if you move
                let mut tiles_buffer: Vec<(Position, Tile)> = vec![(next, Tile::Robot)];
                // so we are going to have [Robot, box, something]
                let mut next_next_pos = self.find_pos(next, m);
                let tile_clone = self.tiles.clone();
                let mut next_next_tile = tile_clone
                    .get(next_next_pos.y)
                    .unwrap()
                    .get(next_next_pos.x)
                    .unwrap();
                let mut prev_tile = Tile::Box;
                while next_next_tile != &Tile::Wall {
                    if next_next_tile == &Tile::Space {
                        // move the buffer and set original to space
                        tiles_buffer.push((next_next_pos, Tile::Box));
                        *self
                            .tiles
                            .get_mut(curr_pos.y)
                            .unwrap()
                            .get_mut(curr_pos.x)
                            .unwrap() = Tile::Space;

                        for (new_pos, new_tile) in tiles_buffer.iter() {
                            // move the robot and blocks in the direction
                            *self
                                .tiles
                                .get_mut(new_pos.y)
                                .unwrap()
                                .get_mut(new_pos.x)
                                .unwrap() = *new_tile;

                            if new_tile == &Tile::Robot {
                                self.robot.pos.x = new_pos.x;
                                self.robot.pos.y = new_pos.y;
                            }
                        }
                        return;
                    } else {
                        // continue til i hit a wall
                        tiles_buffer.push((next_next_pos, prev_tile));
                        prev_tile = *next_next_tile;
                        next_next_pos = self.find_pos(next_next_pos, m);
                        next_next_tile = tile_clone
                            .get(next_next_pos.y)
                            .unwrap()
                            .get(next_next_pos.x)
                            .unwrap();
                    }
                }
            }
            Tile::Robot => {
                panic!()
            }
        };
    }

    pub fn score(&self) -> usize {
        let mut score = 0;
        for (j, row) in self.tiles.iter().enumerate() {
            for (i, el) in row.iter().enumerate() {
                if el == &Tile::Box {
                    score += (100 * j) + i;
                }
            }
        }
        score
    }
}

pub fn day_fifteen(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;
    let content: Vec<&str> = content.split("\n\n").collect();
    let input_grid = content.first().unwrap();
    let input_moves = content.get(1).unwrap();

    let input_tiles: Vec<Vec<Tile>> = input_grid
        .split("\n")
        .map(|row| {
            let row_tiles: Vec<Tile> = row
                .chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '@' => Tile::Robot,
                    '.' => Tile::Space,
                    'O' => Tile::Box,
                    _ => panic!(),
                })
                .collect();
            row_tiles
        })
        .collect();

    let input_moves: Vec<Move> = input_moves
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '^' => Move::Up,
            '<' => Move::Left,
            '>' => Move::Right,
            'v' => Move::Down,
            _ => panic!(),
        })
        .collect();

    let mut grid = Grid::new(input_tiles);

    input_moves.iter().for_each(|m| grid.make_move(*m));

    let score = grid.score();

    println!("score {:?} in {:?}", score, now.elapsed().as_micros());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_postion_new() {
        let x = 1;
        let y = 2;
        assert_eq!(Position { x, y }, Position::new(x, y))
    }

    #[test]
    fn test_robot_new() {
        let pos = Position::new(1, 2);
        assert_eq!(Robot { pos }, Robot::new(pos))
    }

    #[test]
    fn test_grid_new() {
        let tiles = vec![vec![Tile::Robot, Tile::Space, Tile::Wall]];
        let robot = Robot::new(Position::new(0, 0));
        assert_eq!(
            Grid {
                tiles: tiles.clone(),
                robot
            },
            Grid::new(tiles)
        );
    }
    #[test]
    fn test_grid_find_pos() {
        let tiles = vec![vec![Tile::Wall, Tile::Space, Tile::Wall]];
        let grid = Grid::new(tiles);
        let pos = grid.find_pos(Position::new(0, 0), Move::Right);
        assert_eq!(pos, Position::new(1, 0));
        let pos = grid.find_pos(Position::new(0, 0), Move::Down);
        assert_eq!(pos, Position::new(0, 1));
        let pos = grid.find_pos(Position::new(0, 1), Move::Up);
        assert_eq!(pos, Position::new(0, 0));
        let pos = grid.find_pos(Position::new(1, 0), Move::Left);
        assert_eq!(pos, Position::new(0, 0));
    }

    #[test]
    fn test_grid_make_move() {
        let tiles = vec![vec![Tile::Robot, Tile::Space, Tile::Wall]];
        let expected_tiles = vec![vec![Tile::Space, Tile::Robot, Tile::Wall]];

        let mut grid = Grid::new(tiles);

        grid.make_move(Move::Right);
        assert_eq!(grid.tiles, expected_tiles);
    }

    #[test]
    fn test_grid_make_move_2() {
        let tiles = vec![vec![Tile::Robot, Tile::Box, Tile::Wall]];
        let expected_tiles = vec![vec![Tile::Robot, Tile::Box, Tile::Wall]];

        let mut grid = Grid::new(tiles);

        grid.make_move(Move::Right);
        assert_eq!(grid.tiles, expected_tiles);
    }

    #[test]
    fn test_grid_make_move_3() {
        let tiles = vec![vec![
            Tile::Robot,
            Tile::Box,
            Tile::Box,
            Tile::Box,
            Tile::Box,
            Tile::Space,
            Tile::Wall,
        ]];
        let expected_tiles = vec![vec![
            Tile::Space,
            Tile::Robot,
            Tile::Box,
            Tile::Box,
            Tile::Box,
            Tile::Box,
            Tile::Wall,
        ]];

        let mut grid = Grid::new(tiles);

        grid.make_move(Move::Right);
        assert_eq!(grid.tiles, expected_tiles);
    }

    #[test]
    fn test_grid_make_move_3_down() {
        let tiles = vec![
            vec![Tile::Robot],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Space],
            vec![Tile::Wall],
        ];
        let expected_tiles = vec![
            vec![Tile::Space],
            vec![Tile::Robot],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Wall],
        ];

        let mut grid = Grid::new(tiles);

        grid.make_move(Move::Down);
        assert_eq!(grid.tiles, expected_tiles);
    }

    #[test]
    fn test_grid_make_move_4() {
        let tiles = vec![vec![Tile::Robot, Tile::Wall, Tile::Wall]];
        let expected_tiles = vec![vec![Tile::Robot, Tile::Wall, Tile::Wall]];

        let mut grid = Grid::new(tiles);

        grid.make_move(Move::Right);
        assert_eq!(grid.tiles, expected_tiles);
    }

    #[test]
    fn test_grid_score() {
        let tiles = vec![
            vec![Tile::Robot],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Box],
            vec![Tile::Space],
            vec![Tile::Wall],
        ];

        let grid = Grid::new(tiles);

        let score = grid.score();
        assert_eq!(score, 1_000);
    }
}
