use std::{fs::read, io::BufRead, iter::repeat, time::Instant};

// so we are given some kind of compressed data
// that represent larger data
// i.e. 12345 represents
// - 1 block with id 0 and two free spaces
// - 3 block with id 1 and 4 free spaces
// - 5 block to end
//
// we want to convert that to the repesnentation
// 12345 -> 00..111....22222
// we then want to apply a function to it that
// fills in the left most space with the right most value
// until they are all filled
//00..111....22222 -> 002211122
//and then calculate the checksum which is sum(value * position)
//
//
//
//
//
#[derive(Clone, PartialEq, Debug)]
enum Bits {
    Empty,
    Value(u16),
}

#[derive(Clone, PartialEq, Debug)]
struct Block {
    value: Bits,
    length: usize,
    start_pos: usize,
    moved: bool,
}

struct Data {
    compressed: Vec<u16>,
    raw: Vec<Bits>,
    raw_reduced: Vec<Bits>,
    empty_blocks: Vec<Block>,
    value_blocks: Vec<Block>,
    non_empty_count: usize,
}

impl Data {
    pub fn new(compressed: Vec<u16>) -> Self {
        Data {
            compressed,
            raw: vec![],
            raw_reduced: vec![],
            empty_blocks: vec![],
            value_blocks: vec![],
            non_empty_count: 0,
        }
    }
    pub fn expand(&mut self) {
        // so this should convert a compressed file to a raw and store it in the
        // struct
        let n = self.compressed.len() - 1;
        let mut result: Vec<Bits> = vec![];
        for (i, j) in (0..n + 1).step_by(2).enumerate() {
            let block_length = self.compressed[j];
            self.non_empty_count += block_length as usize;
            let free_length = if j == n { 0 } else { self.compressed[j + 1] };
            result.extend(std::iter::repeat_n(
                Bits::Value(i as u16),
                block_length as usize,
            ));
            result.extend(std::iter::repeat_n(Bits::Empty, free_length as usize));
        }
        self.raw = result;
    }

    pub fn d9p2(self) -> u64 {
        // so this time we want to take the right most, unattempted value block
        // and try to move it to the left most empty block. Attempt a move for each block
        // only once
        // so this should convert a compressed file to a raw and store it in the
        // struct
        let n = self.compressed.len() - 1;
        let mut empty_blocks: Vec<Block> = vec![];
        let mut value_blocks: Vec<Block> = vec![];
        let mut pos_counter = 0;
        for (i, j) in (0..n + 1).step_by(2).enumerate() {
            let new_block = Block {
                value: Bits::Value(i as u16),
                length: self.compressed[j] as usize,
                start_pos: pos_counter,
                moved: false,
            };
            value_blocks.push(new_block);
            pos_counter += self.compressed[j] as usize;
            if j != n {
                let empty_block = Block {
                    value: Bits::Empty,
                    length: self.compressed[j + 1] as usize,
                    start_pos: pos_counter,
                    moved: false,
                };
                empty_blocks.push(empty_block);
                pos_counter += self.compressed[j + 1] as usize;
            }
        }

        // ok so now i need to try and move each block from right to left to the
        // left most empty block it can. So lets start with the rest most
        // and go until i have tried to move them all
        //
        let mut right_block = value_blocks.last();
        for v in value_blocks.iter_mut().rev() {
            for e in empty_blocks.iter_mut() {
                if v.length <= e.length {
                    // move the block to that pos and change empty block to reflect
                    // that and exit the loop
                    //
                    v.start_pos = e.start_pos;
                    e.start_pos += v.length;
                    e.length -= v.length;

                    break;
                }
            }
        }
        return 20;
    }
    pub fn reduce_raw(&mut self) {
        // this will take the raw value and reduce it so that there are no
        // empty gaps between values
        // basic plan is to iterate through from left to right until non_empty_count
        // and swap the right most non_empty point with the left most empty
        let mut left = 0;
        let mut right = self.raw.len() - 1;
        let mut reduced = self.raw.clone();
        while left < self.non_empty_count {
            let mut v = &self.raw[left];
            while v != &Bits::Empty {
                left += 1;
                v = &reduced[left];
            }

            let mut w = &reduced[right];
            while w == &Bits::Empty {
                right -= 1;
                w = &reduced[right];
            }
            if left < self.non_empty_count {
                // swap left and right
                reduced.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        self.raw_reduced = reduced;
    }

    pub fn get_checksum(&self) -> u64 {
        // so now we just want to take reduced raw and
        // calculate sum(pos * val)
        let mut sum = 0;
        for (i, v) in self
            .raw_reduced
            .iter()
            .enumerate()
            .take(self.non_empty_count)
        {
            let val = match v {
                Bits::Empty => 0,
                Bits::Value(x) => *x as u64 * i as u64,
            };
            sum += val;
        }
        sum
    }
}

pub fn day_nine(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = std::fs::read_to_string(path).unwrap();
    let digits: Vec<u16> = content
        .chars()
        .filter_map(|s| s.to_digit(10))
        .map(|d| d as u16)
        .collect();

    let mut x = Data::new(digits);

    x.expand();
    x.reduce_raw();
    let checksum = x.get_checksum();

    println!(
        "the checksum is {} and was calculated in {} us",
        checksum,
        now.elapsed().as_micros()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data_expand() {
        let input = vec![1, 2, 3, 4, 5];
        let mut data = Data::new(input);
        data.expand();
        let expected = [
            Bits::Value(0),
            Bits::Empty,
            Bits::Empty,
            Bits::Value(1),
            Bits::Value(1),
            Bits::Value(1),
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
            Bits::Value(2),
            Bits::Value(2),
            Bits::Value(2),
            Bits::Value(2),
            Bits::Value(2),
        ];
        assert_eq!(data.raw, expected);
        assert_eq!(data.non_empty_count, 9);
    }

    #[test]
    fn test_data_reduce_raw() {
        let input = vec![1, 2, 3, 4, 5];
        let mut data = Data::new(input);
        data.expand();
        data.reduce_raw();
        let expected = [
            Bits::Value(0),
            Bits::Value(2),
            Bits::Value(2),
            Bits::Value(1),
            Bits::Value(1),
            Bits::Value(1),
            Bits::Value(2),
            Bits::Value(2),
            Bits::Value(2),
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
            Bits::Empty,
        ];
        assert_eq!(data.raw_reduced, expected);
    }

    #[test]
    fn test_data_get_checksum() {
        let input = vec![1, 2, 3, 4, 5];
        let mut data = Data::new(input);
        data.expand();
        data.reduce_raw();
        let actual = data.get_checksum();
        let expected = 60;
        assert_eq!(actual, expected);
    }
}
