use std::iter::repeat;

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
    Value(u8),
}

struct Data {
    compressed: Vec<u8>,
    raw: Vec<Bits>,
    raw_reduced: Vec<Bits>,
}

impl Data {
    pub fn new(compressed: Vec<u8>) -> Self {
        Data {
            compressed,
            raw: vec![],
            raw_reduced: vec![],
        }
    }
    pub fn expand(&mut self) {
        // so this should convert a compressed file to a raw and store it in the
        // struct
        let n = self.compressed.len() - 1;
        let mut result: Vec<Bits> = vec![];
        for (i, j) in (0..n).step_by(2).enumerate() {
            let block_length = self.compressed[j];
            let free_length = if j == n { 0 } else { self.compressed[j + 1] };
            result.extend(std::iter::repeat_n(
                Bits::Value(i as u8),
                block_length as usize,
            ));
            result.extend(std::iter::repeat_n(Bits::Empty, free_length as usize));
        }
        self.raw = result;
    }
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
    }
}
