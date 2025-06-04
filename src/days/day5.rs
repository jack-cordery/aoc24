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
    map: HashMap<u8, usize>,
    middle: usize,
}

impl<'a> Update<'a> {
    fn new(line: &'a str) -> Self {
        let line_iter = line.split(",").map(|val| val.parse::<u8>().unwrap());
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
        let Some(pos_x) = self.map.get(&rule.x) else {
            return true;
        };
        let Some(pos_y) = self.map.get(&rule.y) else {
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

    fn to_vec(&self) -> Vec<u8> {
        self.line
            .split(",")
            .map(|val| val.parse::<u8>().unwrap())
            .collect()
    }
}

struct Rule {
    x: u8,
    y: u8,
}

impl Rule {
    fn from_str(line: &str) -> Self {
        // read a line of x|y and return the corresponding Rule

        let (x, y) = line.split_once("|").unwrap();
        Rule {
            x: x.parse::<u8>().unwrap(),
            y: y.parse::<u8>().unwrap(),
        }
    }
}

fn solver(updates: Vec<Update>, rules: Vec<Rule>) -> u64 {
    // iteratre through all lines and check each rule is satisfied and return if follows rules
    let mut sum: u64 = 0;
    for update in updates {
        let valid = update.check_all(&rules);
        if valid {
            sum += update.to_vec()[update.middle] as u64;
        }
    }
    sum
}
