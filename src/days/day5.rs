use std::{collections::HashMap, fs::read_to_string, io::Result, time::Instant};

// Ok so the file is formated with two sections seperated by a newline
// the first section contains X|Y pairs that are numbers indicating some pages
// page ordering rules - 47|53 means that if an update contains both then 47 must be before 53 (not
// immedialty)
// the second section contains comma seperated numbers incdicating the page numbers for each update
// need to identify which updates are already in the right order
//
// so i think store the rules and the check each one is satisfied. Can probably then
// first calculate for redundant rules
pub fn day_five(path: &str) -> Result<()> {
    let now = Instant::now();

    let contents = read_to_string(path)?;

    // TODO: Check that \n\n actually splits as intended
    let (first, second) = contents.split_once("\n\n").unwrap();

    let rules: Vec<Rule> = first.lines().map(Rule::from_str).collect();
    let updates: Vec<Update> = second.lines().map(Update::new).collect();

    // apply solver to all the lines and count the bools
    let answer = solver(updates, rules);

    println!(
        "the answer is {} and the time elapsed is {}",
        answer,
        now.elapsed().as_micros()
    );
    Ok(())
}

struct Update<'a> {
    line: &'a str,
    map: HashMap<u64, usize>,
    middle: usize,
}

impl<'a> Update<'a> {
    fn new(line: &'a str) -> Self {
        let line_iter = line.split(",").map(|val| val.parse::<u64>().unwrap());
        let mut len: usize = 0;

        let mut map = HashMap::new();
        for (i, page) in line_iter.enumerate() {
            map.insert(page, i);
            len = i;
        }
        Update {
            line,
            map,
            middle: len / 2,
        }
    }

    fn check(&self, rule: &Rule) -> bool {
        let Some(pos_y) = self.map.get(&rule.y) else {
            return true;
        };
        let Some(pos_x) = self.map.get(&rule.x) else {
            return true;
        };
        pos_x < pos_y
    }

    fn check_all(&self, rules: &[Rule]) -> bool {
        for rule in rules {
            let valid = self.check(rule);
            if !valid {
                return false;
            }
        }
        true
    }

    fn to_vec(&self) -> Vec<u64> {
        self.line
            .split(",")
            .map(|val| val.parse::<u64>().unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Rule {
    x: u64,
    y: u64,
}

impl Rule {
    fn from_str(line: &str) -> Self {
        // read a line of x|y and return the corresponding Rule

        let (x, y) = line.split_once("|").unwrap();
        Rule {
            x: x.parse::<u64>().unwrap(),
            y: y.parse::<u64>().unwrap(),
        }
    }
}

fn solver(updates: Vec<Update>, rules: Vec<Rule>) -> u64 {
    // iteratre through all lines and check each rule is satisfied and return if follows rules
    let mut sum: u64 = 0;
    for update in updates {
        let valid = update.check_all(&rules);
        println!(
            "update: {} with length is {valid} and middle {}",
            update.line, update.middle
        );
        if valid {
            let middle_val = update.to_vec()[update.middle];
            println!("with value {middle_val}");
            sum += middle_val;
            println!("sum is now {sum}");
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_new() {
        //should create a new update instance where the
        //input is a &str like 1,2,3,4,5 and it stores a middle of 2, and hashmap with values
        //of 1,2,3,4,5 and their posiitions 0,1,2,3,4
        let input = "1,2,3,4,5";
        let up = Update::new(input);
        let mut expected_map = HashMap::new();
        expected_map.insert(1, 0);
        expected_map.insert(2, 1);
        expected_map.insert(3, 2);
        expected_map.insert(4, 3);
        expected_map.insert(5, 4);
        assert_eq!(up.line, input);
        assert_eq!(up.middle, 2);
        assert_eq!(up.map, expected_map);
    }

    #[test]
    fn test_day_update_check() {
        // checks the update complies with a given rule i.e. 1,2,3,4,5 and 4|5 is true but 5|4 is
        // false
        //
        let update = Update::new("1,2,3,4,5");
        let rule = Rule::from_str("4|5");
        let rule_to_fail = Rule::from_str("5|4");
        assert!(update.check(&rule));
        assert!(!update.check(&rule_to_fail));
    }

    #[test]
    fn test_day_update_check_all() {
        let update = Update::new("1,2,3,4,5");
        let rule = Rule::from_str("4|5");
        let rule_to_fail = Rule::from_str("5|4");
        let rule2 = Rule::from_str("1|2");
        let rule3 = Rule::from_str("3|5");
        let rule4 = Rule::from_str("1|5");
        let rules = vec![rule, rule2];
        let failed_rules = vec![rule3, rule4, rule_to_fail];

        assert!(update.check_all(&rules));
        assert!(!update.check_all(&failed_rules));
    }

    #[test]
    fn test_day_to_vec() {
        //takes the &str and creates the vec
        //should create a new update instance where the
        //input is a &str like 1,2,3,4,5 and it stores a middle of 2, and hashmap with values
        //of 1,2,3,4,5 and their posiitions 0,1,2,3,4
        //
        //
        let input = "1,2,3,4,5";
        let up = Update::new(input);
        let expected = Vec::from([1, 2, 3, 4, 5]);
        assert_eq!(up.to_vec(), expected);
    }

    #[test]
    fn test_day_rule_from_str() {
        //takes &str and reutrn the x and y in a rule
        //
        let input = "5|4";
        let rule = Rule::from_str(input);
        assert_eq!(rule.x, 5);
        assert_eq!(rule.y, 4);
    }

    #[test]
    fn test_day_solver() {
        //takes a vec of updates and rules and returns the sum of the middle vals of those i
        // in correct order can
        let update1 = Update::new("1,2,3,4,5");
        let update2 = Update::new("10,20,30,40,50");
        let update3 = Update::new("100,200,201,202,203");
        let rule2 = Rule::from_str("1|2");
        let rule3 = Rule::from_str("3|5");
        let rule4 = Rule::from_str("1|5");
        let rule5 = Rule::from_str("203|202");

        let updates = vec![update1, update2, update3];
        let rules = vec![rule2, rule3, rule4, rule5];

        assert_eq!(solver(updates, rules), 33);
    }
}
