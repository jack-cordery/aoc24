use aoc24::days::day1::day_one;
use aoc24::days::day2::day_two;
use aoc24::days::day3::day_three;
use aoc24::days::day4::day_four;
use aoc24::days::day5::day_five;
use aoc24::days::day6::day_six;
use aoc24::days::day7::day_seven;
use aoc24::days::day8::day_eight;
use aoc24::days::day9::day_nine;
use aoc24::days::day10::day_ten;
use aoc24::days::day11::day_eleven;
use aoc24::days::day12::day_twelve;
use aoc24::days::day13::day_thirteen;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let (Some(_), Some(day), Some(path)) = (args.next(), args.next(), args.next()) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Expected day arg",
        ));
    };
    match day.as_str() {
        "day_one" => day_one(path)?,
        "day_two" => day_two(path.as_str())?,
        "day_three" => day_three(path.as_str())?,
        "day_four" => day_four(path.as_str())?,
        "day_five" => day_five(path.as_str())?,
        "day_six" => day_six(path.as_str())?,
        "day_seven" => day_seven(path.as_str())?,
        "day_eight" => day_eight(path.as_str())?,
        "day_nine" => day_nine(path.as_str())?,
        "day_ten" => day_ten(path.as_str())?,
        "day_eleven" => day_eleven(path.as_str())?,
        "day_twelve" => day_twelve(path.as_str())?,
        "day_thirteen" => day_thirteen(path.as_str())?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected day_x",
            ));
        }
    }
    Ok(())
}
