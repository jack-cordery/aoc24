// So we have a map that contains antennas which are labelled
// by a digit or character, captal is disctinct i.e. a != A
// We have antinodes which have a position on the map and are
// created by a pair of the same antennas and are at a position
// in a line between the two and the distance away in the opposite
// direction. They exist as long as they are on the map
// WE just need to calculate how many exist on the map
//

use std::{collections::HashMap, fs::read, io::BufRead, time::Instant};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Map {
    grid: HashMap<Position, Tiles>,
    size: GridSize,
}

#[derive(Debug, PartialEq)]
struct GridSize {
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
enum Tiles {
    Empty,
    Antinode,
    Antenna(char),
    Overlapping(char),
}

#[derive(Debug)]
enum TileError {
    InvalidInputTile,
}

impl Tiles {
    fn from_char(c: char) -> std::result::Result<Self, TileError> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Antinode),
            c => Ok(Self::Antenna(c)),
        }
    }
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let mut hmap: HashMap<Position, Tiles> = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (row, v) in grid.iter().enumerate() {
            for (col, c) in v.iter().enumerate() {
                let pos = Position { x: col, y: row };
                let tile = Tiles::from_char(*c);
                match tile {
                    Ok(t) => hmap.insert(pos, t),
                    Err(_) => panic!("invalid input tile"),
                };
                width = col;
            }
            height = row;
        }
        Map {
            grid: hmap,
            size: GridSize {
                width: width + 1,
                height: height + 1,
            },
        }
    }

    fn count_antinodes(&self) -> usize {
        self.grid
            .values()
            .filter(|t| matches!(t, Tiles::Overlapping(_) | Tiles::Antinode))
            .count()
    }

    fn find_antinodes(&mut self) {
        // here we need to essentially group ANs into same types
        // pairwise calculate the antinode positions
        let cloned_grid = self.grid.clone();
        let mut antennas_grouped: HashMap<Tiles, Vec<Position>> = HashMap::new();
        for (key, chunk) in &cloned_grid.into_iter().chunk_by(|elt| {
            let (_, tile) = elt;
            *tile
        }) {
            if matches!(key, Tiles::Antenna(_)) {
                match antennas_grouped.get_mut(&key) {
                    None => {
                        antennas_grouped.insert(
                            key,
                            chunk
                                .map(|c| {
                                    let (pos, _) = c;
                                    pos
                                })
                                .collect(),
                        );
                    }
                    Some(a) => {
                        for (pos, _) in chunk {
                            a.push(pos);
                        }
                    }
                }
            }
        }
        //TODO: Fix below to now use the actual proper grouped sets
        for (_, positions) in antennas_grouped.into_iter().filter(|a| {
            let (t, _) = a;
            matches!(t, Tiles::Antenna(_))
        }) {
            let mut rest = positions.clone();
            for a in positions {
                rest.remove(0);
                for other in &rest {
                    // for each pair of antennas a, and other
                    // we want to calculate the two antinode positions
                    // to do that we need to calculate the delta x, delta y
                    // just check theyre in bounds
                    // TODO: Fix the calculation of new x and new y
                    // its actually that the point with the bigger x
                    // gets x added to and the point with the bigger y
                    // gets added to

                    let (pos_a, pos_other) = (a, other);

                    println!("antenna pair found at {:?} {:?} ", pos_a, pos_other);

                    let delta_x = pos_a.x as i32 - pos_other.x as i32;
                    let delta_y = pos_a.y as i32 - pos_other.y as i32;

                    // i now have a delta vector which points from  other to a
                    // so now i can minus that from other and add two to other
                    let (first_x, first_y) =
                        (pos_other.x as i32 - delta_x, pos_other.y as i32 - delta_y);
                    let (second_x, second_y) = (
                        pos_other.x as i32 + 2 * delta_x,
                        pos_other.y as i32 + 2 * delta_y,
                    );

                    println!(
                        "first antinode {:?} {:?} with deltas {} {}",
                        first_x, first_y, delta_x, delta_y
                    );

                    if (first_x < self.size.width.try_into().unwrap())
                        && (first_x >= 0)
                        && (first_y < self.size.height.try_into().unwrap())
                        && (first_y >= 0)
                    {
                        let updated_tile = match self.grid.get(&Position {
                            x: first_x.try_into().unwrap(),
                            y: first_y.try_into().unwrap(),
                        }) {
                            Some(Tiles::Empty) => Tiles::Antinode,
                            Some(Tiles::Antinode) => Tiles::Antinode,
                            Some(Tiles::Overlapping(t)) => Tiles::Overlapping(*t),
                            Some(Tiles::Antenna(t)) => Tiles::Overlapping(*t),
                            None => panic!("Unexpected position"),
                        };
                        println!("updating first with {:?}", updated_tile);
                        self.grid.insert(
                            Position {
                                x: first_x.try_into().unwrap(),
                                y: first_y.try_into().unwrap(),
                            },
                            updated_tile,
                        );
                    }

                    println!(
                        "second antinode {:?} {:?} with deltas {} {}",
                        second_x, second_y, delta_x, delta_y
                    );

                    if (second_x < self.size.width.try_into().unwrap())
                        && (second_x >= 0)
                        && (second_y < self.size.height.try_into().unwrap())
                        && (second_y >= 0)
                    {
                        let updated_tile = match self.grid.get(&Position {
                            x: second_x.try_into().unwrap(),
                            y: second_y.try_into().unwrap(),
                        }) {
                            Some(Tiles::Empty) => Tiles::Antinode,
                            Some(Tiles::Antinode) => Tiles::Antinode,
                            Some(Tiles::Overlapping(t)) => Tiles::Overlapping(*t),
                            Some(Tiles::Antenna(t)) => Tiles::Overlapping(*t),
                            None => panic!("Unexpected position"),
                        };
                        println!("updating second with {:?}", updated_tile);
                        self.grid.insert(
                            Position {
                                x: second_x.try_into().unwrap(),
                                y: second_y.try_into().unwrap(),
                            },
                            updated_tile,
                        );
                    }
                }
            }
        }
    }
}

pub fn day_eight(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read(path)?;
    let char_grid: Vec<Vec<char>> = content
        .lines()
        .map(|l| {
            let binding = l.unwrap();
            let chars: Vec<char> = binding.chars().collect();
            chars
        })
        .collect();
    let mut map = Map::new(char_grid);

    map.find_antinodes();
    let answer = map.count_antinodes();
    println!(
        "the total unique antinodes are {} and was calculated in {} us",
        answer,
        now.elapsed().as_micros()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // tests needed for Tile::from_char, X
    // Map::new, X
    // Map::count_antinodes,
    // find_antinodes
    //
    #[test]
    fn test_count_antinodes() {
        let char_grid = vec![
            vec!['.', '.', '.'],
            vec!['.', 'd', '.'],
            vec!['c', 'd', 'e'],
            vec!['.', '.', '.'],
        ];

        let mut map = Map::new(char_grid);
        map.find_antinodes();
        let actual = map.count_antinodes();
        let expected = 2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_antinodes_2() {
        let char_grid = vec![
            vec!['.', '.', '.'],
            vec!['.', 'd', 'd'],
            vec!['c', '.', 'e'],
            vec!['.', '.', 'e'],
        ];

        let mut map = Map::new(char_grid);
        map.find_antinodes();
        let actual = map.count_antinodes();
        let expected = 2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_antinodes_3() {
        let char_grid = vec![
            vec!['.', '.', '.'],
            vec!['.', 'd', 'd'],
            vec!['c', 'd', '.'],
            vec!['.', '.', '.'],
        ];

        let mut map = Map::new(char_grid);
        map.find_antinodes();
        let actual = map.count_antinodes();
        let expected = 4;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_new() {
        let char_grid = vec![
            vec!['.', '.', '.'],
            vec!['c', 'd', 'e'],
            vec!['.', '.', '.'],
        ];
        let mut expected_map = HashMap::new();
        expected_map.insert(Position { x: 0, y: 0 }, Tiles::Empty);
        expected_map.insert(Position { x: 1, y: 0 }, Tiles::Empty);
        expected_map.insert(Position { x: 2, y: 0 }, Tiles::Empty);
        expected_map.insert(Position { x: 0, y: 1 }, Tiles::Antenna('c'));
        expected_map.insert(Position { x: 1, y: 1 }, Tiles::Antenna('d'));
        expected_map.insert(Position { x: 2, y: 1 }, Tiles::Antenna('e'));
        expected_map.insert(Position { x: 0, y: 2 }, Tiles::Empty);
        expected_map.insert(Position { x: 1, y: 2 }, Tiles::Empty);
        expected_map.insert(Position { x: 2, y: 2 }, Tiles::Empty);
        let expected = Map {
            grid: expected_map,
            size: GridSize {
                height: 3,
                width: 3,
            },
        };

        let actual = Map::new(char_grid);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_antinodes() {
        let char_grid = vec![
            vec!['.', '.', '.'],
            vec!['.', 'd', '.'],
            vec!['c', 'd', 'e'],
            vec!['.', '.', '.'],
        ];
        let expected_grid = vec![
            vec!['.', '#', '.'],
            vec!['.', 'd', '.'],
            vec!['c', 'd', 'e'],
            vec!['.', '#', '.'],
        ];

        let mut actual = Map::new(char_grid);
        let expected = Map::new(expected_grid);

        actual.find_antinodes();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tile_from_char() {
        let a = '.';
        let b = 'a';
        let c = '1';

        assert_eq!(Tiles::from_char(a).unwrap(), Tiles::Empty);
        assert_eq!(Tiles::from_char(b).unwrap(), Tiles::Antenna('a'));
        assert_eq!(Tiles::from_char(c).unwrap(), Tiles::Antenna('1'));
    }
}
