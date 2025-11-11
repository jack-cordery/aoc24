use std::{fs::read_to_string, time::Instant};

use regex::Regex;

/// ok so we need to solve a linear problem
/// i have the cost function J(alpha,beta) = 3 * alpha + beta
/// i have two buttons A and B which move some thing in x-y space
/// and i have a target t
/// i essesntially need to find how many of each presses it takes to get me to t
/// this means i have either no solution, many solutions, or one
/// i can first calculate if i have a solution i.e. det(a) != 0  and if each component is integer
/// i can then check if a and b are co-linear if they are
/// i  then just need to calculate how many steps it takes to get there with beta
/// else i have one solution which i can calculate with a formula
///
/// ok so i essentially need a Game object with
/// a, b and t defined  and then go from there
#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Game {
    a: Vector,
    b: Vector,
    t: Vector,
    det: i64,
}

impl Vector {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Game {
    fn new(a: Vector, b: Vector, t: Vector) -> Self {
        let det = (a.x * b.y) - (b.x * a.y);
        Self { a, b, t, det }
    }

    /// this is w.r.t. to integer solutions
    fn is_invertible(&self) -> bool {
        if self.det == 0 {
            return false;
        };
        let alpha = self.a.x * self.t.y - self.a.y * self.t.x;
        let beta = self.b.y * self.t.x - self.b.x * self.t.y;

        if alpha % (self.det.abs()) != 0 {
            return false;
        }
        if (beta) % (self.det.abs()) != 0 {
            return false;
        }
        if (alpha) / (self.det) < 0 {
            return false;
        }
        if (beta) / (self.det) < 0 {
            return false;
        }

        true
    }

    fn is_colinear(&self) -> bool {
        let ratio_x = (self.a.x as f64) / (self.b.x as f64);
        let ratio_y = (self.a.y as f64) / (self.b.y as f64);
        ratio_x.abs() == ratio_y.abs()
    }

    fn get_opt_cost(&self) -> u64 {
        if !self.is_invertible() {
            return 0;
        }
        if self.is_colinear() {
            let ratio_b_x = (self.t.x) / (self.b.x);
            let ratio_a_x = (self.t.x) / (self.a.x);
            if 3 * ratio_a_x < ratio_b_x {
                return 3 * ratio_a_x as u64;
            }
            return ratio_b_x as u64;
        }
        let beta = (self.a.x * self.t.y - self.a.y * self.t.x) / self.det;
        let alpha = (self.b.y * self.t.x - self.b.x * self.t.y) / self.det;

        3 * (alpha as u64) + (beta as u64)
    }
}

pub fn day_thirteen(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;
    let groups: Vec<&str> = content.split("\n\n").collect();

    let cost: u64 = groups
        .iter()
        .map(|g| {
            let rows: Vec<&str> = g.split("\n").collect();
            let a = rows.first().unwrap();
            let b = rows.get(1).unwrap();
            let t = rows.get(2).unwrap();

            let a: Vec<i64> = Regex::new(r"[+]?\d+")
                .unwrap()
                .find_iter(a)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let b: Vec<i64> = Regex::new(r"[+]?\d+")
                .unwrap()
                .find_iter(b)
                .map(|m| m.as_str().parse().unwrap())
                .take(2)
                .collect();
            let t: Vec<i64> = Regex::new(r"=(\d+)")
                .unwrap()
                .find_iter(t)
                .map(|m| m.as_str()[1..].parse().unwrap())
                .take(2)
                .collect();

            let a = Vector::new(*a.first().unwrap(), *a.get(1).unwrap());
            let b = Vector::new(*b.first().unwrap(), *b.get(1).unwrap());
            let t = Vector::new(
                *t.first().unwrap() + 10000000000000,
                *t.get(1).unwrap() + 10000000000000,
            );

            let h = Game::new(a, b, t);

            h.get_opt_cost()
        })
        .sum();

    println!(
        "the cost of the tokens is {}  and it took {}",
        cost,
        now.elapsed().as_micros(),
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_vector() {
        let actual = Vector::new(1, 2);
        let expected = Vector { x: 1, y: 2 };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_new_game() {
        let a = Vector::new(1, 2);
        let b = Vector::new(3, 4);
        let t = Vector::new(5, 6);
        let det = -2;
        let actual = Game::new(a, b, t);
        let expected = Game { a, b, t, det };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_is_colinear() {
        let a = Vector::new(1, 2);
        let b = Vector::new(3, 4);
        let t = Vector::new(5, 6);
        let g = Game::new(a, b, t);
        let actual = g.is_colinear();
        let expected = false;
        assert_eq!(actual, expected);

        let a = Vector::new(1, 2);
        let b = Vector::new(2, 4);
        let t = Vector::new(5, 6);
        let g = Game::new(a, b, t);
        let actual = g.is_colinear();
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_invertible() {
        let a = Vector::new(1, 2);
        let b = Vector::new(3, 4);
        let t = Vector::new(5, 6);
        let g = Game::new(a, b, t);
        let actual = g.is_invertible();
        let expected = false;
        assert_eq!(actual, expected);

        let a = Vector::new(1, 2);
        let b = Vector::new(3, 4);
        let t = Vector::new(4, 6);
        let g = Game::new(a, b, t);
        let actual = g.is_invertible();
        let expected = true;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_opt_cost() {
        let a = Vector::new(94, 34);
        let b = Vector::new(22, 67);
        let t = Vector::new(8400, 5400);
        let g = Game::new(a, b, t);
        let actual = g.get_opt_cost();
        let expected = 280;
        assert_eq!(actual, expected);

        let a = Vector::new(26, 66);
        let b = Vector::new(67, 21);
        let t = Vector::new(12748, 12176);
        let g = Game::new(a, b, t);
        let actual = g.get_opt_cost();
        let expected = 0;
        assert_eq!(actual, expected);
    }
}
