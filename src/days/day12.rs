// ok so we will have an array which will represent some
// kind of letter A, B,C etc.
// that is a "plant"
// if a plant borders with another plant of the same type
// its in a group
// a group has a perimeter and an area which depends
// on the numbr of borders (i.e. non group neighbours) and the count respectively
//
// we basically ened to go through each position and work out which
// group it belongs to and calculate the number of non-neighbours
// by moving left-right top-down it means by looking above and then left we
// can check if there is a group that already exists for that plant

use std::{fs::read, io::BufRead, time::Instant};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Plant {
    label: char,
    i: usize,
    j: usize,
    group: Option<usize>,
    borders: Option<usize>, // this will just be number of non-group edges
    edges: Option<usize>,
}

impl Plant {
    pub fn new(c: char, i: usize, j: usize) -> Self {
        Self {
            label: c,
            i,
            j,
            group: None,
            borders: None,
            edges: None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Map {
    plants: Vec<Vec<Plant>>,
    n_groups: Option<usize>,
    size_i: usize,
    size_j: usize,
}

impl Map {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let plants: Vec<Vec<Plant>> = grid
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.iter()
                    .enumerate()
                    .map(|(i, c)| Plant::new(*c, i, j))
                    .collect()
            })
            .collect();
        let size_i = plants.first().unwrap().len();
        let size_j = plants.len();
        Self {
            plants,
            n_groups: None,
            size_i,
            size_j,
        }
    }

    pub fn print(&self) {
        for row in self.plants.iter() {
            row.iter()
                .for_each(|e| print!("{}:{} ", e.label, e.group.unwrap()));
            println!();
        }
    }
    fn count_edges(&mut self, i: usize, j: usize) {
        // so we want to take the plant at i and j and check all the corners
        let mut is_neighbour_left = false;
        let max_i = self.size_i - 1;
        let max_j = self.size_j - 1;
        let plant_grid = self.plants.clone();

        if i > 0
            && plant_grid.get(j).unwrap().get(i).unwrap().label
                == plant_grid.get(j).unwrap().get(i - 1).unwrap().label
        {
            is_neighbour_left = true;
        }
        let mut is_neighbour_right = false;
        if i < max_i
            && plant_grid.get(j).unwrap().get(i).unwrap().label
                == plant_grid.get(j).unwrap().get(i + 1).unwrap().label
        {
            is_neighbour_right = true;
        }
        let mut is_neighbour_below = false;
        if j < max_j
            && plant_grid.get(j).unwrap().get(i).unwrap().label
                == plant_grid.get(j + 1).unwrap().get(i).unwrap().label
        {
            is_neighbour_below = true;
        }
        let mut is_neighbour_above = false;
        if j > 0
            && plant_grid.get(j).unwrap().get(i).unwrap().label
                == plant_grid.get(j - 1).unwrap().get(i).unwrap().label
        {
            is_neighbour_above = true;
        }

        let mut edges = 0;

        if !is_neighbour_above && !is_neighbour_right {
            edges += 1;
        }
        if !is_neighbour_below && !is_neighbour_right {
            edges += 1;
        }
        if !is_neighbour_below && !is_neighbour_left {
            edges += 1;
        }
        if !is_neighbour_above && !is_neighbour_left {
            edges += 1;
        }

        let current_plant = self.plants.get_mut(j).unwrap().get_mut(i).unwrap();
        current_plant.edges = Some(edges);
    }
    // this calculates the cost of Map by area * perimeter of each group summed
    pub fn get_cost(&self) -> u64 {
        let mut cost = 0;
        for i in 0..self.n_groups.unwrap() {
            let group = self
                .plants
                .iter()
                .flatten()
                .filter(|p| p.group.unwrap() == i);
            let perimeter: u64 = group.clone().map(|p| p.borders.unwrap() as u64).sum();
            let area: u64 = group.count() as u64;
            cost += area * perimeter;
        }
        cost
    }

    pub fn get_cost_2(&self) -> u64 {
        let mut cost = 0;
        for i in 0..self.n_groups.unwrap() {
            let group = self
                .plants
                .iter()
                .flatten()
                .filter(|p| p.group.unwrap() == i);
            let number_external_edges: u64 = group.clone().map(|p| p.edges.unwrap() as u64).sum();
            let number_sides = 4 + 2 * (number_external_edges - 4);
            let area: u64 = group.count() as u64;
            cost += area * number_sides;
        }
        cost
    }

    pub fn find_neighbours_and_borders(&mut self) {
        // ok so im going to want to recursively look for neighbours
        // so for each element i want to then calculate the
        // borders and the neighbours and then
        // call that function on the neighbours
        // i probably want some kind of flag
        let mut starting_index = 0;

        for j in 0..self.size_j {
            for i in 0..self.size_i {
                let is_new = Self::search((i, j), None, None, self, Some(starting_index));
                if !is_new {
                    starting_index += 1;
                }
            }
        }
        self.n_groups = Some(starting_index);
    }

    fn search(
        start_plant_pos: (usize, usize),
        parent_plant_pos: Option<(usize, usize)>,
        parent_group: Option<usize>,
        plant_grid: &mut Self,
        starting_group_index: Option<usize>,
    ) -> bool {
        let mut positions: Vec<(usize, usize)> = vec![];
        let (i, j) = (start_plant_pos.0, start_plant_pos.1);
        if plant_grid
            .plants
            .get(j)
            .unwrap()
            .get(i)
            .unwrap()
            .group
            .is_some()
        {
            return true;
        }
        // left
        if i > 0 {
            positions.push((i - 1, j));
        }
        // up
        if j > 0 {
            positions.push((i, j - 1));
        }

        // right
        if i < plant_grid.size_i - 1 {
            positions.push((i + 1, j));
        }
        // down
        if j < plant_grid.size_j - 1 {
            positions.push((i, j + 1));
        }

        let neighbours: Vec<&(usize, usize)> = positions
            .iter()
            .filter(|(k, l)| {
                // neightbours with same label
                plant_grid.plants.get(*l).unwrap().get(*k).unwrap().label
                    == plant_grid.plants.get(j).unwrap().get(i).unwrap().label
            })
            .collect();

        let num_neighbours = neighbours.len();

        plant_grid.count_edges(i, j);

        let borders = 4 - num_neighbours;
        let mut new_parent_group = None;
        {
            {
                let current_plant = plant_grid.plants.get_mut(j).unwrap().get_mut(i).unwrap();
                current_plant.borders = Some(borders);
            }
            {}
            match parent_group {
                Some(p) => {
                    let current_plant = plant_grid.plants.get_mut(j).unwrap().get_mut(i).unwrap();
                    current_plant.group = Some(p);
                    new_parent_group = Some(p)
                }
                None => {
                    let current_plant = plant_grid.plants.get_mut(j).unwrap().get_mut(i).unwrap();
                    // this is panicking as its come up as None
                    current_plant.group = Some(starting_group_index.unwrap());
                    new_parent_group = Some(starting_group_index.unwrap());
                }
            }
        }
        let neighbours_new: Vec<&(usize, usize)> = neighbours
            .iter()
            .copied()
            .filter(|n| {
                // only keep neighbours that dont have a group already
                plant_grid
                    .plants
                    .get(n.1)
                    .unwrap()
                    .get(n.0)
                    .unwrap()
                    .group
                    .is_none()
            })
            .collect();
        for n in neighbours_new {
            Self::search(
                *n,
                Some(start_plant_pos),
                parent_group,
                plant_grid,
                new_parent_group,
            );
        }
        false
    }
}

pub fn day_twelve(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read(path)?;
    let data: Vec<Vec<char>> = content
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut map = Map::new(data);
    map.find_neighbours_and_borders();
    let result = map.get_cost();
    let result_2 = map.get_cost_2();

    println!(
        "the cost of the fence is {} or with discount {} and it took {}",
        result,
        result_2,
        now.elapsed().as_micros(),
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_new() {
        let plant_a = Plant {
            label: 'a',
            i: 0,
            j: 0,
            group: None,
            borders: None,
            edges: None,
        };
        let plant_b = Plant {
            label: 'b',
            i: 1,
            j: 0,
            group: None,
            borders: None,
            edges: None,
        };
        let plant_c = Plant {
            label: 'c',
            i: 2,
            j: 0,
            group: None,
            borders: None,
            edges: None,
        };
        let plant_d = Plant {
            label: 'd',
            i: 3,
            j: 0,
            group: None,
            borders: None,
            edges: None,
        };
        let expected = Map {
            plants: vec![vec![plant_a, plant_b, plant_c, plant_d]],
            size_i: 4,
            size_j: 1,
            n_groups: None,
        };
        let actual = Map::new(vec![vec!['a', 'b', 'c', 'd']]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_search() {
        let mut map = Map::new(vec![vec!['a', 'b', 'b', 'd']]);
        let is_new = Map::search((0, 0), None, None, &mut map, Some(0));
        let actual_borders = map.plants.get(0).unwrap().get(0).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(0).unwrap().group;
        let actual_edges = map.plants.get(0).unwrap().get(0).unwrap().edges;
        assert_eq!(Some(4), actual_borders);
        assert_eq!(Some(0), actual_group);
        assert_eq!(Some(4), actual_edges);
        let expected_group = map.plants.get(0).unwrap().get(1).unwrap().group;
        assert_eq!(expected_group, None);
        let expected_group = map.plants.get(0).unwrap().get(2).unwrap().group;
        assert_eq!(expected_group, None);
        let expected_group = map.plants.get(0).unwrap().get(3).unwrap().group;
        assert_eq!(expected_group, None);
    }
    #[test]
    fn test_map_search_again() {
        let mut map = Map::new(vec![vec!['a', 'b', 'b', 'd']]);
        Map::search((1, 0), None, None, &mut map, Some(0));
        let actual_borders = map.plants.get(0).unwrap().get(1).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(1).unwrap().group;
        let actual_edges = map.plants.get(0).unwrap().get(1).unwrap().edges;
        assert_eq!(Some(3), actual_borders);
        assert_eq!(Some(0), actual_group);
        assert_eq!(Some(2), actual_edges);
        let actual_borders = map.plants.get(0).unwrap().get(2).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(2).unwrap().group;
        let actual_edges = map.plants.get(0).unwrap().get(2).unwrap().edges;
        assert_eq!(Some(3), actual_borders);
        assert_eq!(Some(0), actual_group);
        assert_eq!(Some(2), actual_edges);
        let expected_group = map.plants.get(0).unwrap().get(0).unwrap().group;
        assert_eq!(expected_group, None);
        let expected_group = map.plants.get(0).unwrap().get(3).unwrap().group;
        assert_eq!(expected_group, None);
    }
    #[test]
    fn test_map_find_neighbours_and_borders() {
        let mut map = Map::new(vec![vec!['a', 'b', 'b', 'd']]);
        map.find_neighbours_and_borders();
        let actual_borders = map.plants.get(0).unwrap().get(0).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(0).unwrap().group;
        assert_eq!(Some(4), actual_borders);
        assert_eq!(Some(0), actual_group);
        let actual_borders = map.plants.get(0).unwrap().get(1).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(1).unwrap().group;
        assert_eq!(Some(3), actual_borders);
        assert_eq!(Some(1), actual_group);
        let actual_borders = map.plants.get(0).unwrap().get(2).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(2).unwrap().group;
        assert_eq!(Some(3), actual_borders);
        assert_eq!(Some(1), actual_group);
        let actual_borders = map.plants.get(0).unwrap().get(3).unwrap().borders;
        let actual_group = map.plants.get(0).unwrap().get(3).unwrap().group;
        assert_eq!(Some(4), actual_borders);
        assert_eq!(Some(2), actual_group);

        assert_eq!(Some(3), map.n_groups);
    }

    #[test]
    fn test_map_count_edges() {
        let input = vec![vec!['a', 'b', 'c']];
        let mut map = Map::new(input);

        let (pos_i, pos_j) = (0, 0);
        map.count_edges(pos_i, pos_j);
        let actual = map.plants.get(pos_j).unwrap().get(pos_i).unwrap().edges;
        let expected = Some(4);
        assert_eq!(expected, actual);
        let (pos_i, pos_j) = (1, 0);
        map.count_edges(pos_i, pos_j);
        let actual = map.plants.get(pos_j).unwrap().get(pos_i).unwrap().edges;
        let expected = Some(4);
        assert_eq!(expected, actual);
        let (pos_i, pos_j) = (2, 0);
        map.count_edges(pos_i, pos_j);
        let actual = map.plants.get(pos_j).unwrap().get(pos_i).unwrap().edges;
        let expected = Some(4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_get_cost() {
        let input = vec![vec!['a', 'b', 'c']];
        let expected = 12;

        let mut map = Map::new(input);
        map.find_neighbours_and_borders();
        let actual = map.get_cost();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_get_cost_again() {
        let input = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['b', 'b', 'c', 'd'],
            vec!['b', 'b', 'c', 'c'],
            vec!['e', 'e', 'e', 'c'],
        ];
        let expected = 140;
        let mut map = Map::new(input);
        map.find_neighbours_and_borders();
        let actual = map.get_cost();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_map_get_cost_2_again() {
        let input = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['b', 'b', 'c', 'd'],
            vec!['b', 'b', 'c', 'c'],
            vec!['e', 'e', 'e', 'c'],
        ];
        let expected = 80;
        let mut map = Map::new(input);
        map.find_neighbours_and_borders();
        let actual = map.get_cost_2();
        assert_eq!(expected, actual);
    }
}
