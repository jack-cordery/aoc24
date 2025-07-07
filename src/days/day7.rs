// ok so we want to build some kind of program
// that reads in lines of text as usual
// the structure is a <int>: []int{}
// so that will be easy just split by : and then store
// the space separated as an array or slice
//
// we then want to check if any combination of the operators
// + and * make the equation valid
// i.e. 100: 10 10 is valid as 10 * 10 = 100
//
// from a structures perspective i think
// we will have an equation with properies value, numbers, and valid
//
// we will have enum operators which include add and multiply
//
// and then we will just check if any combiations of operators applied to
// the numbers of the equations equals value
//
// finally we just sum all of the valid equation values
//

struct Equation {
    value: u16,
    numbers: Vec<u16>,
    valid: bool,
}

enum Operators {
    Addition,
    Multiplication,
}

impl Operators {
    pub fn to_char(&self) -> char {
        match self {
            Operators::Addition => '+',
            Operators::Multiplication => '*',
        }
    }
}

impl Equation {
    pub fn print_with_operators(&self, operator_combo: Vec<Operators>) {
        for (num, op) in self.numbers.iter().zip(operator_combo) {
            let op_char = op.to_char();
            print!("{num} {op_char} ");
        }

        let last_num = self.numbers.last().unwrap();
        let value = self.value;

        print!("{last_num} = {value}");
    }
}
