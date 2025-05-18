use std::io;
use std::time::Instant;

fn check_safety(input: Vec<u8>) -> bool {
    // need to check that the vector is either strictly increasing or or decreasing
    // each step can only have a delta of 1,2,3
    let mut delta;
    let mut prev_delta = 0;
    let l = input.len();
    for i in 1..l {
        delta = (input[i] as i8) - (input[i - 1] as i8);
        if delta.abs() > 3 || delta.abs() < 1 {
            return false;
        }
        if i > 1 && delta * prev_delta < 0 {
            return false;
        }
        prev_delta = delta;
    }
    true
}

/// Returns -1 if the input is safe and otherwise returns the pos of the first offending number
fn check_safety_with_pos(input: &[u8]) -> i8 {
    // need to check that the vector is either strictly increasing or or decreasing
    // each step can only have a delta of 1,2,3
    let mut delta;
    let mut prev_delta = 0;
    let l = input.len();
    for i in 1..l {
        delta = (input[i] as i8) - (input[i - 1] as i8);
        if delta.abs() > 3 || delta.abs() < 1 {
            return i as i8;
        }
        if i > 1 && delta * prev_delta < 0 {
            return i as i8;
        }
        prev_delta = delta;
    }
    -1
}

fn check_safety_dampner(input: &[u8]) -> bool {
    // ok this time we are allowed one mistake so what we will do
    // we can use check with pos to try twice with removing the offending pos and then retrying
    // so we have an edge case where in the first two postiion if it looks like its decreasing but
    // then increases it could fail eg. 12, 10, 13, 16, 19, 2
    // thats because it deletes the first value that looks wrong which currently would be 13, when
    // we would actually want to delete 10
    // so we need to account for the first three characters
    let mut first_try = check_safety_with_pos(input);
    if first_try == -1 {
        return true;
    }
    let mut second_input = input.to_vec();
    second_input.remove(first_try as usize);
    let second_try = check_safety_with_pos(&second_input);
    if second_try == -1 {
        return true;
    }
    if first_try < 3 {
        while first_try > 0 {
            let last_try = first_try - 1;
            let mut last_input = input.to_vec();
            last_input.remove(last_try as usize);
            let last_try = check_safety_with_pos(&last_input);
            if last_try == -1 {
                return true;
            }
            first_try -= 1;
        }
    }
    false
}

fn parse_to_check_safety(input: &str) -> bool {
    let v: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    check_safety(v)
}
fn parse_to_check_safety_with_dampner(input: &str) -> bool {
    let v: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    check_safety_dampner(&v)
}

pub fn day_two(path: &str) -> io::Result<()> {
    let now = Instant::now();
    let contents = std::fs::read_to_string(path)?;
    let mut count = 0;
    let mut count_damp = 0;
    let mut count_unsafe = 0;
    for line in contents.lines() {
        if parse_to_check_safety(line) {
            count += 1;
            count_damp += 1;
        } else if parse_to_check_safety_with_dampner(line) {
            count_damp += 1;
        } else {
            println!("Unsafe!: {line}");
            count_unsafe += 1;
        }
    }
    println!(
        "the count of safe rows is {count}, and with dampner its {count_damp}, leaving {count_unsafe} lines and it took {} us",
        now.elapsed().as_micros()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_safety() {
        assert!(check_safety(vec![7, 6, 4, 2, 1]));
        assert!(!check_safety(vec![1, 2, 7, 8, 9]));
        assert!(!check_safety(vec![9, 7, 6, 2, 1]));
        assert!(!check_safety(vec![1, 3, 2, 4, 5]));
        assert!(!check_safety(vec![8, 6, 4, 4, 1]));
        assert!(check_safety(vec![1, 3, 6, 7, 9]));
        assert!(!check_safety(vec![16, 17, 18, 21, 24, 21]));
    }

    #[test]
    fn test_check_safety_damper() {
        assert!(check_safety_dampner(&[7, 6, 4, 2, 1]));
        assert!(!check_safety_dampner(&[1, 2, 7, 8, 9]));
        assert!(!check_safety_dampner(&[9, 7, 6, 2, 1]));
        assert!(check_safety_dampner(&[1, 3, 2, 4, 5]));
        assert!(check_safety_dampner(&[8, 6, 4, 4, 1]));
        assert!(check_safety_dampner(&[1, 3, 6, 7, 9]));
        assert!(!check_safety_dampner(&[16, 17, 18, 2, 1, 24, 21]));
        assert!(check_safety_dampner(&[12, 10, 13, 16, 19, 21, 22]));
        assert!(check_safety_dampner(&[88, 90, 88, 86, 84, 82, 80]));
    }

    #[test]
    fn test_parse_safety() {
        assert!(parse_to_check_safety("1 2 3 4 5 6 7 8"));
        assert!(parse_to_check_safety("7 6 4 2 1"));
        assert!(!parse_to_check_safety("9 7 6 2 1"));
    }
}
