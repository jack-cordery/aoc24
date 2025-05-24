use regex::Regex;
use std::fs;
use std::io::Result;
use std::time::Instant;

// ok so we need to be able to take in an input like the other days
// cant recall if we take an iterable or not but can check
// but then we can take each line and then use regex to match
// mult(x,y) no spaces and that. use regex101 to test the pattern
// ok so have a func that takes in per line and then read in the file
// and run that func over it
pub fn day_three(path: &str) -> Result<()> {
    let now = Instant::now();
    let contents = fs::read_to_string(path)?;
    let mut total = 0;
    for line in contents.lines() {
        total += sum_line(line);
    }
    let t = now.elapsed().as_micros();
    println!("total sum is {} in {}us ", total, t);
    let input: String = contents.lines().collect();
    let s = sum_file(&input);
    let t = now.elapsed().as_micros();
    println!("total sum of two is {} in {}us ", s, t);
    Ok(())
}

fn sum_line(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<u64> = re
        .find_iter(input)
        .map(|m| re.captures(m.as_str()).unwrap())
        .map(|c| {
            let first = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let second = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
            first * second
        })
        .collect();
    matches.iter().sum()
}

/// this time need to ensure to only take strings inbetween do() and don't()
/// ensuring to check start and end
/// ok so the program starts with a do() - so i could just append that on the front
/// and add dont() onto the end but that has to be done on the input level -> just feed the whole
/// string in not line by line then i can do it on this level
fn sum_file(input: &str) -> u64 {
    let input_corrected = format!("do() {} don't()", input);
    let filter_re = Regex::new(r"do\(\)(.+?)don\'t\(\)").unwrap();
    let input_corrected: String = filter_re
        .captures_iter(&input_corrected)
        .map(|c| {
            let (_, [capt]) = c.extract();
            capt
        })
        .collect();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<u64> = re
        .find_iter(&input_corrected)
        .map(|m| re.captures(m.as_str()).unwrap())
        .map(|c| {
            let first = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let second = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
            first * second
        })
        .collect();
    matches.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_line() {
        assert_eq!(sum_line("mul(100,100),,, mul( 5,    1)mul(3,8)"), 10024);
    }

    #[test]
    fn test_sum_file() {
        assert_eq!(
            sum_file("mul(100,100)don't(),,mul(6,6), mul( 5,    1)do()mul(3,8)"),
            10024
        );
    }
}
