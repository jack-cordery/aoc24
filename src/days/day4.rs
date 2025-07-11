use std::fs::read;
use std::io::{BufRead, Result};
use std::time::Instant;

// Ok so this function needs to take in a path from args and then
// read in the file and then apply a function to the contents of the file that
// returns the number of times XMAS appears
pub fn day_four(path: &str) -> Result<()> {
    let now = Instant::now();
    let contents = read(path)?;
    let char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let result = xmas_search(&char_matrix);
    let result_two = x_mas_search(&char_matrix);
    println!(
        "The number of XMAS codes found is {}! and the number of X-MAS codes found is {} It took {}us to calculate",
        result,
        result_two,
        now.elapsed().as_micros()
    );
    Ok(())
}

pub fn x_mas_search(puzzle: &[Vec<char>]) -> u32 {
    // ;alsdkfj;laskdfj;alsdkfj;alsdkjf;asldkfj;asldkfja;sldkfj
    // find all the As  and then search in the corners for MS or SM
    let mut result = 0;
    let max_rows = puzzle.len();
    let max_cols = puzzle[0].len();
    for (i, row) in puzzle.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == 'A' {
                // check the diaganols
                if i > 0 && j > 0 && i < max_rows - 1 && j < max_cols - 1 {
                    let mut a = puzzle[i - 1][j - 1].to_string();
                    a.push(puzzle[i + 1][j + 1]);
                    let mut b = puzzle[i - 1][j + 1].to_string();
                    b.push(puzzle[i + 1][j - 1]);
                    if (a == "MS" || a == "SM") && (b == "MS" || b == "SM") {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

//Generall thinking is that store each line as a vector of characters i.e. a matrix of chars
// find all xs and for that x then look for an m and then an a and s in all directions that is up
// left right and down,and diaganols as backwards is allowed
// so for an X at (i,j) when search at positions
// Down -> (i + 1, j),
// Up -> (i - 1, j)
// Right -> (i, j + 1),
// Left -> (i, j - 1)
// Diag UR -> (i -1, j + 1)
// Diag UL -> (i -1, j -1 )
// Diag DR -> (i +1, j + 1)
// Diag DL -> (i +1, j - 1)
// If there is a m then continue to look in that direction (for loop )
pub fn xmas_search(puzzle: &[Vec<char>]) -> u32 {
    let mut result = 0;
    let max_rows = puzzle.len();
    let max_cols = puzzle[0].len();
    for (i, row) in puzzle.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == 'X' {
                //look for M

                // search right
                if j + 3 < max_cols {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i][j + r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }

                // search left
                if j >= 3 {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i][j - r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }

                // search up
                if i >= 3 {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i - r][j]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
                // search down
                if i + 3 < max_rows {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i + r][j]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
                // search diag UR
                if (j + 3 < max_cols) && (i >= 3) {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i - r][j + r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
                // search diag UL
                if (j >= 3) && (i >= 3) {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i - r][j - r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
                // search diag DR
                if (j + 3 < max_cols) && (i + 3 < max_rows) {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i + r][j + r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
                // search diag DL
                if (j >= 3) && (i + 3 < max_rows) {
                    let a = 1..4;
                    let right: String = a.map(|r| puzzle[i + r][j - r]).collect();
                    if right == "MAS" {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

#[test]
fn test_xmas_search() {
    let input = "XMAS\nMASX\nASXM\nSXMA\nSAMX";
    let input_matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(xmas_search(&input_matrix), 3);
}

#[test]
fn test_x_mas_search() {
    let input = "XMAS\nMASX\nASXM\nSXMA\nSAMX";
    let input_matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(x_mas_search(&input_matrix), 0);
}

#[test]
fn test_xmas_search_again() {
    let input = "AMXS\nXMAS\nAMXS";
    let input_matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(x_mas_search(&input_matrix), 1);
}
