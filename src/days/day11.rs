use std::{
    fs::{read, read_to_string},
    time::Instant,
};

/// ok so we have a setup on a line (an array of integers)
/// on each blink (step)
/// each integer can do 1 of 3 things
/// - if the integer is 0 it becomes 1
/// - if the number of digits is even then it splits into two numbers the first half of the
///     digits followed by the second half i.e. 10 -> 1, 0.
/// - if none of the above two apply then the digit gets multiplied by 2024
///
///
/// so initial thoughts is have a Line struct which is an array of Stones
/// and implement a blink step which moves left to right evolving a Stone

/// stone represents a stone with an integer carved on it that can transform in
/// three ways zeroToOne, Split, Multiply
#[derive(Debug, PartialEq)]
struct Stone(u64);

/// A line of stones which evolves on each blink into a new Line
#[derive(Debug, PartialEq)]
struct Line(Vec<Stone>);

impl Line {
    pub fn new(stones: Vec<u64>) -> Self {
        Self(stones.iter().map(|n| Stone(*n)).collect())
    }
    pub fn evolve(&mut self) {
        self.0 = self.0.iter().flat_map(|s| s.blink()).collect();
    }
    pub fn evolve_n(&mut self, n: usize) {
        for _ in 0..n {
            self.evolve();
        }
    }
}

impl Stone {
    pub fn blink(&self) -> Vec<Self> {
        if self.0 == 0 {
            vec![Self(1)]
        } else if self.is_even_digits() {
            self.split()
        } else {
            vec![Self(self.0 * 2024)]
        }
    }

    fn is_even_digits(&self) -> bool {
        if self.0 < 1 {
            return false;
        }
        let digits = self.0.ilog10() + 1;
        if digits % 2 == 0 {
            return true;
        }
        false
    }

    fn split(&self) -> Vec<Self> {
        let digits = self.0.ilog10() + 1;
        let base: u64 = 10;
        let denom = base.pow(digits / 2);

        let first_set = self.0 % denom;
        let second_set = (self.0 - first_set) / denom;
        vec![Self(second_set), Self(first_set)]
    }
}

pub fn day_eleven(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read_to_string(path)?;
    let digits: Vec<u64> = content
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut line = Line::new(digits);
    println!("the line is {:?}", line);

    line.evolve_n(25);

    println!(
        "the score is {} and  and calculated in {:?}",
        line.0.len(),
        now.elapsed().as_micros()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stone_blink() {
        let input_zero = Stone(0);
        let input_even = Stone(9999);
        let input_even_zeros = Stone(10);
        let input_else = Stone(5);

        let expected_zero = vec![Stone(1)];
        let expected_even = vec![Stone(99), Stone(99)];
        let expected_even_zeros = vec![Stone(1), Stone(0)];
        let expected_else = vec![Stone(10120)];

        assert_eq!(expected_zero, input_zero.blink());
        assert_eq!(expected_even, input_even.blink());
        assert_eq!(expected_even_zeros, input_even_zeros.blink());
        assert_eq!(expected_else, input_else.blink());
    }
    #[test]
    fn test_is_even_digits() {
        let input_zero = Stone(0);
        let input_even = Stone(9999);
        let input_even_zeros = Stone(10);
        let input_else = Stone(5);

        let expected_zero = false;
        let expected_even = true;
        let expected_even_zeros = true;
        let expected_else = false;

        assert_eq!(expected_zero, input_zero.is_even_digits());
        assert_eq!(expected_even, input_even.is_even_digits());
        assert_eq!(expected_even_zeros, input_even_zeros.is_even_digits());
        assert_eq!(expected_else, input_else.is_even_digits());
    }
    #[test]
    fn test_stone_split() {
        let input_even = Stone(9999);
        let input_even_zeros = Stone(10);
        let input_even_unsym = Stone(1599);

        let expected_even = vec![Stone(99), Stone(99)];
        let expected_even_zeros = vec![Stone(1), Stone(0)];
        let expected_even_unsym = vec![Stone(15), Stone(99)];

        assert_eq!(expected_even, input_even.split());
        assert_eq!(expected_even_zeros, input_even_zeros.split());
        assert_eq!(expected_even_unsym, input_even_unsym.split());
    }
    #[test]
    fn test_line_new() {
        let input = vec![1, 2, 3, 4];
        let expected = Line(vec![Stone(1), Stone(2), Stone(3), Stone(4)]);

        let actual = Line::new(input);

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_line_evolve() {
        let mut line = Line::new(vec![125, 17]);
        let expected = Line::new(vec![253000, 1, 7]);
        let expected_second = Line::new(vec![253, 0, 2024, 14168]);

        line.evolve();
        assert_eq!(expected, line);
        line.evolve();
        assert_eq!(expected_second, line);
    }

    #[test]
    fn test_line_evolve_n() {
        let mut line = Line::new(vec![125, 17]);
        let expected_second = Line::new(vec![253, 0, 2024, 14168]);

        line.evolve_n(2);
        assert_eq!(expected_second, line);
    }

    #[test]
    fn test_toy() {
        let mut line = Line::new(vec![125, 17]);
        let expected = 55312;

        line.evolve_n(25);
        assert_eq!(expected, line.0.len());
    }
}
