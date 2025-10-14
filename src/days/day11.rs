use std::{
    collections::HashMap,
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

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Stone(u64);

/// A line of stones which evolves on each blink into a new Line
#[derive(Debug, PartialEq)]
pub struct Line {
    line: Vec<Stone>,
    memo: HashMap<(Stone, usize), u64>,
}

impl Line {
    pub fn new(stones: Vec<u64>) -> Self {
        let memo = HashMap::new();
        Self {
            line: stones.iter().map(|n| Stone(*n)).collect(),
            memo,
        }
    }
}

pub fn blink_stones_n(
    stones: Vec<Stone>,
    n: usize,
    memo: &mut HashMap<(Stone, usize), u64>,
) -> u64 {
    stones.iter().map(|s| blink_n(s.clone(), n, memo)).sum()
}

pub fn blink_n(stone: Stone, n: usize, memo: &mut HashMap<(Stone, usize), u64>) -> u64 {
    // ok so given a stone and steps i want to recursively count the
    // number of stones it generates
    //
    if n == 0 {
        return 1;
    }
    if let Some(cached) = memo.get(&(stone.clone(), n)) {
        return *cached;
    }
    let mut result = 0;

    let stones = stone.blink();
    if stones.len() == 1 {
        result += blink_n(stones.first().unwrap().clone(), n - 1, memo);
    } else {
        result += blink_n(stones.first().unwrap().clone(), n - 1, memo)
            + blink_n(stones.get(1).unwrap().clone(), n - 1, memo);
    }

    memo.insert((stone, n), result);

    result
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

    let result = blink_stones_n(line.line, 75, &mut line.memo);

    println!(
        "the score is {} and  and calculated in {:?}",
        result,
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
    fn test_blink_n() {
        let mut memo = HashMap::new();
        let actual = blink_stones_n(vec![Stone(125), Stone(17)], 6, &mut memo);
        let expected = vec![
            Stone(2097446912),
            Stone(14168),
            Stone(4048),
            Stone(2),
            Stone(0),
            Stone(2),
            Stone(4),
            Stone(40),
            Stone(48),
            Stone(2024),
            Stone(40),
            Stone(48),
            Stone(80),
            Stone(96),
            Stone(2),
            Stone(8),
            Stone(6),
            Stone(7),
            Stone(6),
            Stone(0),
            Stone(3),
            Stone(2),
        ];
        assert_eq!(expected.len() as u64, actual);
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
        let expected = Line {
            line: vec![Stone(1), Stone(2), Stone(3), Stone(4)],
            memo: HashMap::new(),
        };

        let actual = Line::new(input);

        assert_eq!(expected, actual);
    }
}
