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

use std::{fs::read, io::BufRead, time::Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
    operators: Vec<Operators>,
    valid: Validity,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operators {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Validity {
    True,
    False,
    Unchecked,
}

impl Operators {
    pub fn to_char(self) -> char {
        match self {
            Operators::Addition => '+',
            Operators::Multiplication => '*',
        }
    }
}

impl Equation {
    pub fn new(value: u64, numbers: Vec<u64>, operators: Vec<Operators>) -> Self {
        let valid = Validity::Unchecked;
        Equation {
            value,
            numbers,
            operators,
            valid,
        }
    }
    pub fn print_with_operators(&self) {
        for (num, op) in self.numbers.iter().zip(self.operators.clone()) {
            let op_char = op.to_char();
            print!("{num} {op_char} ");
        }

        let last_num = self.numbers.last().unwrap();
        let value = self.value;

        print!("{last_num} = {value}");
    }

    fn check_valid(&self) -> bool {
        let mut result = *self.numbers.first().unwrap();
        let mut num_iter = self.numbers.iter();
        num_iter.next();
        for (num, op) in num_iter.zip(self.operators.clone()) {
            match op {
                Operators::Addition => {
                    result += *num;
                }
                Operators::Multiplication => {
                    result *= *num;
                }
            }
        }
        result == self.value
    }
}

fn get_all_equations(value: u64, numbers: Vec<u64>, all_operators: &[Operators]) -> Vec<Equation> {
    let l = numbers.len();
    let mut operator_combos: Vec<Vec<Operators>> = Vec::new();
    for _ in 0..l - 1 {
        operator_combos = get_operator_combos(operator_combos, all_operators);
    }
    let equations = operator_combos
        .iter()
        .map(|combo| Equation::new(value, numbers.clone(), combo.clone()))
        .collect();
    equations
}

fn get_operator_combos(
    combo_of_operators: Vec<Vec<Operators>>,
    operators: &[Operators],
) -> Vec<Vec<Operators>> {
    // so take each combo of operator and append to each one of each of the operator options
    // so for 1,2,3 [1] ->  [1,1], [1,2], [1,3]
    if combo_of_operators.is_empty() {
        let mut result: Vec<Vec<Operators>> = Vec::new();
        for op in operators {
            result.push(vec![*op]);
        }
        return result;
    }

    let result = combo_of_operators.iter().flat_map(|combo| {
        let mut result: Vec<Vec<Operators>> = Vec::new();
        for op in operators {
            let mut c = combo.clone();
            c.push(*op);
            result.push(c.clone());
        }
        result
    });
    result.collect()
}

fn check_equation_set(equation_set: Vec<Equation>) -> bool {
    for eq in equation_set {
        if eq.check_valid() {
            return true;
        }
    }
    false
}

fn get_total_sum_of_valid_equations(equation_sets: Vec<Vec<Equation>>) -> u64 {
    equation_sets
        .iter()
        .map(|e| {
            if check_equation_set(e.to_vec()) {
                e.first().unwrap().value
            } else {
                0
            }
        })
        .sum()
}

pub fn day_seven(path: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let content = read(path)?;
    let all_ops = vec![Operators::Addition, Operators::Multiplication];
    let equation_sets: Vec<Vec<Equation>> = content
        .lines()
        .map(|l| {
            let binding = l.unwrap();
            let (value_str, eq_str) = binding.split_once(": ").unwrap();
            println!("value is {}", value_str);
            let value: u64 = value_str.parse::<u64>().unwrap();
            let eq: Vec<u64> = eq_str
                .trim_end()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();
            get_all_equations(value, eq, &all_ops)
        })
        .collect();
    let total_sum = get_total_sum_of_valid_equations(equation_sets);
    println!(
        "the total sum from the data is {} and it was calculated in {}",
        total_sum,
        now.elapsed().as_micros()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equation_check_valid() {
        let invalid_operators = vec![Operators::Addition, Operators::Multiplication];
        let valid_operators = vec![Operators::Multiplication, Operators::Addition];

        let valid_equation = Equation::new(100, vec![10, 10, 0], valid_operators);
        let invalid_equation = Equation::new(100, vec![10, 10, 0], invalid_operators);
        assert!(valid_equation.check_valid());
        assert!(!invalid_equation.check_valid());
    }

    #[test]
    fn test_get_all_equations() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equations = get_all_equations(100, vec![1, 2, 3], &all_ops);

        assert_eq!(
            vec![
                Equation::new(
                    100,
                    vec![1, 2, 3],
                    vec![Operators::Addition, Operators::Addition]
                ),
                Equation::new(
                    100,
                    vec![1, 2, 3],
                    vec![Operators::Addition, Operators::Multiplication]
                ),
                Equation::new(
                    100,
                    vec![1, 2, 3],
                    vec![Operators::Multiplication, Operators::Addition]
                ),
                Equation::new(
                    100,
                    vec![1, 2, 3],
                    vec![Operators::Multiplication, Operators::Multiplication]
                ),
            ],
            equations
        );
    }

    #[test]
    fn test_check_equation_set_zero() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equation_set = get_all_equations(100, vec![1, 2, 3], &all_ops);

        let check = check_equation_set(equation_set);
        assert!(!check)
    }

    #[test]
    fn test_check_equation_set_first() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equation_set = get_all_equations(3267, vec![81, 40, 27], &all_ops);
        let check = check_equation_set(equation_set);
        assert!(check)
    }

    #[test]
    fn test_total_sum_of_valid_equations_zero() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equations = get_all_equations(100, vec![1, 2, 3], &all_ops);
        let sum = get_total_sum_of_valid_equations(vec![equations]);
        assert_eq!(0, sum);
    }

    #[test]
    fn test_total_sum_of_valid_equations_base() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equations = get_all_equations(190, vec![10, 19], &all_ops);
        let sum = get_total_sum_of_valid_equations(vec![equations]);
        assert_eq!(190, sum);
    }

    #[test]
    fn test_total_sum_of_valid_equations_multi() {
        let all_ops = vec![Operators::Addition, Operators::Multiplication];
        let equations = get_all_equations(190, vec![10, 19], &all_ops);
        let equations_second = get_all_equations(3267, vec![81, 40, 27], &all_ops);
        let sum = get_total_sum_of_valid_equations(vec![equations, equations_second]);
        assert_eq!(190 + 3267, sum);
    }

    #[test]
    fn test_get_operator_combos_empty() {
        let empty: Vec<Vec<Operators>> = Vec::new();
        let option: &[Operators] = &[Operators::Addition, Operators::Multiplication];
        let actual = get_operator_combos(empty, option);
        let expected = vec![vec![Operators::Addition], vec![Operators::Multiplication]];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_operator_combos_base() {
        let base: Vec<Vec<Operators>> = vec![vec![Operators::Addition]];
        let option: &[Operators] = &[Operators::Addition, Operators::Multiplication];
        let actual = get_operator_combos(base, option);
        let expected = vec![
            vec![Operators::Addition, Operators::Addition],
            vec![Operators::Addition, Operators::Multiplication],
        ];

        assert_eq!(expected, actual);
    }
}
