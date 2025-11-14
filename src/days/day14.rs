use core::f64;
use std::{
    f64::consts::{self, PI},
    fs::read_to_string,
    time::Instant,
};

use regex::Regex;

/// ok so this seems pretty straight forward
/// we have some sort of grid that multiple robots traverse
/// each robot has a position and a velocity and then must perform n steps
/// at the end of n steps we count how many robots are in each of
/// the four quadrants and multiply those numbers together  
///

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    pos: Position,
    velocity: Velocity,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    size_x: usize,
    size_y: usize,
    robots: Vec<Robot>,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl Velocity {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Robot {
    pub fn new(pos: Position, velocity: Velocity) -> Self {
        Self { pos, velocity }
    }

    pub fn step(&mut self, n: &usize, size_x: &usize, size_y: &usize) {
        // a robot just steps at velocity * n wrapping around the grid
        let mut new_x = ((self.pos.x as i16) + ((*n as i16) * self.velocity.x)) % (*size_x as i16);
        let mut new_y = ((self.pos.y as i16) + ((*n as i16) * self.velocity.y)) % (*size_y as i16);

        if new_x < 0 {
            new_x += *size_x as i16;
        }
        if new_y < 0 {
            new_y += *size_y as i16;
        }

        self.pos = Position::new(new_x as usize, new_y as usize);
    }
}
impl Grid {
    pub fn new(size_x: usize, size_y: usize, robots: Vec<Robot>) -> Self {
        Self {
            size_x,
            size_y,
            robots,
        }
    }

    pub fn print(&self) {
        let mut print_grid = vec![vec!['.'; self.size_x]; self.size_y];
        for r in &self.robots {
            print_grid[r.pos.y][r.pos.x] = 'X';
        }
        for row in print_grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    pub fn simulate(&mut self, steps: usize) {
        self.robots
            .iter_mut()
            .for_each(|r| r.step(&steps, &self.size_x, &self.size_y));
    }

    pub fn calculate_kde(&self, robot: &Robot, h: f64) -> f64 {
        let mut result = 0.0;
        for r in &self.robots {
            result += (((robot.pos.x - r.pos.x) * (robot.pos.x - r.pos.x)
                + (robot.pos.y - r.pos.y) * (robot.pos.y - r.pos.y)) as f64)
                / (2.0 * h);
        }
        result / (2.0 * PI)
    }

    pub fn calculate_entropy(&self, h: f64) -> f64 {
        let mut result = 0.0;
        for r in &self.robots {
            let p = self.calculate_kde(r, h);
            result -= p * p.log2();
        }
        result
    }

    pub fn calculate_quad_score(&self) -> usize {
        // so this draws a line down the middle in both x and y and then counts the number of
        // robots in each quad. the robots on the line dont count

        let quad_bounds_x = match self.size_x % 2 {
            0 => (self.size_x / 2 - 1, self.size_x / 2 + 1),
            1 => (self.size_x / 2 - 1, self.size_x / 2 + 1),
            _ => panic!("value is neither even nor odd"),
        };
        let quad_bounds_y = match self.size_y % 2 {
            0 => (self.size_y / 2 - 1, self.size_y / 2 + 1),
            1 => (self.size_y / 2 - 1, self.size_y / 2 + 1),
            _ => panic!("value is neither even nor odd"),
        };

        let mut lower_left_count = 0;
        let mut lower_right_count = 0;
        let mut upper_left_count = 0;
        let mut upper_right_count = 0;

        for r in &self.robots {
            // take each robot figure out if its in a quad then add it to the count
            let is_lower = r.pos.y >= quad_bounds_y.1;
            let is_upper = r.pos.y <= quad_bounds_y.0;
            let is_left = r.pos.x <= quad_bounds_x.0;
            let is_right = r.pos.x >= quad_bounds_x.1;

            if is_lower && is_left {
                lower_left_count += 1;
            }
            if is_lower && is_right {
                lower_right_count += 1;
            }
            if is_upper && is_right {
                upper_right_count += 1;
            }
            if is_upper && is_left {
                upper_left_count += 1;
            }
        }
        lower_left_count * lower_right_count * upper_left_count * upper_right_count
    }
}

pub fn day_fourteen(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;
    let groups: Vec<&str> = content.lines().collect();

    let re = Regex::new(r"=([+-]?\d+),([+-]?\d+)").unwrap();
    let robots: Vec<Robot> = groups
        .iter()
        .map(|g| {
            let mut capt = re.captures_iter(g).map(|c| c.extract::<2>()).map(|c| {
                (
                    c.1.first().unwrap().parse().unwrap(),
                    c.1.get(1).unwrap().parse().unwrap(),
                )
            });

            let pos: (i64, i64) = capt.next().unwrap();
            let vel: (i64, i64) = capt.next().unwrap();

            let r = Robot::new(
                Position::new(pos.0 as usize, pos.1 as usize),
                Velocity::new(vel.0 as i16, vel.1 as i16),
            );
            r
        })
        .collect();

    let mut grid = Grid::new(101, 103, robots);

    let mut entropies = vec![];

    for i in 1..10000 {
        grid.simulate(1);
        entropies.push((i, grid.calculate_entropy(2.0)));
    }

    entropies.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("entropies {:?}", entropies);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_new() {
        let x = 1;
        let y = 2;
        let actual = Position::new(x, y);
        let expected = Position { x, y };

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_velocity_new() {
        let x = 1;
        let y = -2;
        let actual = Velocity::new(x, y);
        let expected = Velocity { x, y };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_robot_new() {
        let pos = Position::new(0, 0);
        let velocity = Velocity::new(-1, 1);
        let actual = Robot::new(pos, velocity);
        let expected = Robot { pos, velocity };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_grid_new() {
        let size_x = 10;
        let size_y = 20;
        let robots = vec![Robot::new(Position::new(1, 2), Velocity::new(-1, 1))];
        let robots_copy = vec![Robot::new(Position::new(1, 2), Velocity::new(-1, 1))];
        let actual = Grid::new(size_x, size_y, robots);
        let expected = Grid {
            size_x,
            size_y,
            robots: robots_copy,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_robot_step() {
        let mut robot = Robot::new(Position::new(0, 0), Velocity::new(1, 2));
        robot.step(&1, &10, &10);

        assert_eq!(robot.pos, Position::new(1, 2));

        let mut robot = Robot::new(Position::new(0, 0), Velocity::new(1, 2));
        robot.step(&5, &10, &10);

        assert_eq!(robot.pos, Position::new(5, 0));

        let mut robot = Robot::new(Position::new(0, 0), Velocity::new(-1, 2));
        robot.step(&5, &10, &10);

        assert_eq!(robot.pos, Position::new(5, 0));
    }

    #[test]
    fn test_grid_calc() {
        let robots = vec![
            Robot::new(Position::new(0, 4), Velocity::new(3, -3)),
            Robot::new(Position::new(6, 3), Velocity::new(-1, -3)),
            Robot::new(Position::new(10, 3), Velocity::new(-1, 2)),
            Robot::new(Position::new(2, 0), Velocity::new(2, -1)),
            Robot::new(Position::new(0, 0), Velocity::new(1, 3)),
            Robot::new(Position::new(3, 0), Velocity::new(-2, -2)),
            Robot::new(Position::new(7, 6), Velocity::new(-1, -3)),
            Robot::new(Position::new(3, 0), Velocity::new(-1, -2)),
            Robot::new(Position::new(9, 3), Velocity::new(2, 3)),
            Robot::new(Position::new(7, 3), Velocity::new(-1, 2)),
            Robot::new(Position::new(2, 4), Velocity::new(2, -3)),
            Robot::new(Position::new(9, 5), Velocity::new(-3, -3)),
        ];

        let mut grid = Grid::new(11, 7, robots);
        grid.simulate(100);
        let actual = grid.calculate_quad_score();
        let expected = 12;
        assert_eq!(actual, expected);
    }
}
