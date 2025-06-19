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
    let mut updates: Vec<Update> = second
        .lines()
        .map(|line| Update::new(line.into()))
        .collect();

    let updates_slice: &mut [Update] = &mut updates;

    // apply solver to all the lines and count the bools
    let (answer, fixed_answer) = solver(updates_slice, rules);

    println!(
        "the answer is {} with {} and the time elapsed is {}",
        answer,
        fixed_answer,
        now.elapsed().as_micros()
    );
    Ok(())
}

struct Update {
    line: String,
    vec: Vec<u64>,
    map: HashMap<u64, usize>,
    middle: usize,
}

impl Update {
    fn new(line: String) -> Self {
        let line_iter = line.split(",").map(|val| val.parse::<u64>().unwrap());
        let mut len: usize = 0;

        let mut map = HashMap::new();
        let mut vec = Vec::new();
        for (i, page) in line_iter.enumerate() {
            map.insert(page, i);
            vec.push(page);
            len = i;
        }

        Update {
            line,
            vec,
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

    fn fix(&mut self, rule: &Rule) -> bool {
        // this should take the update and check against the rule
        // if the rule fails then change the underlying values
        // of self so that it is fixed
        // for now that means swapping the two positions
        // need to change:
        // vec -> easy with swap
        // line -> need to write a function that converts a vector to a &str csv
        // map -> should be easy to just overwrite the values of the two

        let map_x = &self.map.get(&rule.x).cloned();
        let map_y = &self.map.get(&rule.y).cloned();

        let Some(pos_y) = map_y else {
            return true;
        };
        let Some(pos_x) = map_x else {
            return true;
        };
        if pos_x < pos_y {
            return true;
        }
        self.vec.swap(*pos_x, *pos_y);
        self.line = Self::vec_to_csv(&self.vec);

        self.map.insert(rule.x, *pos_y);
        self.map.insert(rule.y, *pos_x);

        false
    }

    fn vec_to_csv(vec_vals: &[u64]) -> String {
        let vec_strings: Vec<String> = vec_vals.iter().map(|v| v.to_string()).collect();
        vec_strings.join(",")
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

    fn fix_all(&mut self, rules: &[Rule]) {
        for rule in rules {
            let valid = self.fix(rule);
            if !valid {
                return self.fix_all(rules);
            }
        }
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

fn solver(updates: &mut [Update], rules: Vec<Rule>) -> (u64, u64) {
    // iteratre through all lines and check each rule is satisfied and return if follows rules
    let mut sum: u64 = 0;
    let mut fixed_sum: u64 = 0;
    for update in updates {
        let valid = update.check_all(&rules);
        if valid {
            let middle_val = update.vec[update.middle];
            sum += middle_val;
        } else {
            update.fix_all(&rules);
            let middle_val = update.vec[update.middle];
            fixed_sum += middle_val;
        }
    }
    (sum, fixed_sum)
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
        let up = Update::new(input.into());
        let mut expected_map = HashMap::new();
        expected_map.insert(1, 0);
        expected_map.insert(2, 1);
        expected_map.insert(3, 2);
        expected_map.insert(4, 3);
        expected_map.insert(5, 4);

        let expected_vec = vec![1, 2, 3, 4, 5];

        assert_eq!(up.line, input);
        assert_eq!(up.middle, 2);
        assert_eq!(up.map, expected_map);
        assert_eq!(up.vec, expected_vec);
    }

    #[test]
    fn test_day_update_check() {
        // checks the update complies with a given rule i.e. 1,2,3,4,5 and 4|5 is true but 5|4 is
        // false
        //
        let update = Update::new("1,2,3,4,5".into());
        let rule = Rule::from_str("4|5");
        let rule_to_fail = Rule::from_str("5|4");
        assert!(update.check(&rule));
        assert!(!update.check(&rule_to_fail));
    }

    #[test]
    fn test_day_update_check_all() {
        let update = Update::new("1,2,3,4,5".into());
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
    fn test_day_update_fix() {
        let mut update = Update::new("1,2,3,4,5".into());
        let rule_to_fail = Rule::from_str("5|4");
        let expected_vec = vec![1, 2, 3, 5, 4];
        let expected_line = "1,2,3,5,4";
        let mut expected_map = HashMap::new();
        expected_map.insert(1, 0);
        expected_map.insert(2, 1);
        expected_map.insert(3, 2);
        expected_map.insert(5, 3);
        expected_map.insert(4, 4);
        let expected_middle = 2;

        update.fix(&rule_to_fail);
        assert!(update.vec == expected_vec);
        assert!(update.line == expected_line);
        assert!(update.map == expected_map);
        assert!(update.middle == expected_middle);
    }

    #[test]
    fn test_day_update_fix_all() {
        let mut update = Update::new("1,2,3,4,5".into());
        let rule_1 = Rule::from_str("5|4");
        let rule_2 = Rule::from_str("3|2");
        let rule_3 = Rule::from_str("5|1");
        let expected_vec = vec![5, 3, 2, 1, 4];
        let expected_line = "5,3,2,1,4";
        let mut expected_map = HashMap::new();
        expected_map.insert(5, 0);
        expected_map.insert(3, 1);
        expected_map.insert(2, 2);
        expected_map.insert(1, 3);
        expected_map.insert(4, 4);
        let expected_middle = 2;

        update.fix_all(&[rule_1, rule_2, rule_3]);
        assert!(update.vec == expected_vec);
        assert!(update.line == expected_line);
        assert!(update.map == expected_map);
        assert!(update.middle == expected_middle);
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
        let update1 = Update::new("1,2,3,4,5".into());
        let update2 = Update::new("10,20,30,40,50".into());
        let update3 = Update::new("100,200,201,202,203".into());

        let rule2 = Rule::from_str("1|2");
        let rule3 = Rule::from_str("3|5");
        let rule4 = Rule::from_str("1|5");
        let rule5 = Rule::from_str("203|202");

        let mut updates = vec![update1, update2, update3];
        let rules = vec![rule2, rule3, rule4, rule5];
        let updates_slice: &mut [Update] = &mut updates;

        assert_eq!(solver(updates_slice, rules), (33, 201));
    }
}
