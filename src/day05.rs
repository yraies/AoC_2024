use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 5;
    type Parsed = Input;

    fn parse(input: String) -> Self::Parsed {
        let (orderings, updates) = input.split_once("\n\n").unwrap();

        let rules = Rules::from(
            orderings
                .lines()
                .map(|line| {
                    line.split_once("|")
                        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .collect::<Vec<_>>(),
        );

        let updates = updates
            .lines()
            .map(|line| {
                Update(
                    line.split(",")
                        .map(|split| split.parse::<usize>().unwrap())
                        .collect(),
                )
            })
            .collect();

        Input { rules, updates }
    }

    fn part_1(Input { rules, updates }: Self::Parsed) -> i64 {
        updates
            .iter()
            .filter(|&update| update.allowed_by(&rules))
            .map(|update| update.middle())
            .sum::<usize>() as i64
    }

    fn part_2(Input { rules, updates }: Self::Parsed) -> i64 {
        updates
            .iter()
            .filter_map(|update| {
                let rules = rules.get_applicable_for(update);
                if !update.allowed_by(&rules) {
                    Some(update.order_by(&rules))
                } else {
                    None
                }
            })
            .map(|update| update.middle())
            .sum::<usize>() as i64
    }
}

#[derive(Clone)]
pub struct Input {
    rules: Rules,
    updates: Vec<Update>,
}

#[derive(Clone)]
struct Rules(Vec<(usize, usize)>);

impl Rules {
    fn allow(&self, pair: (usize, usize)) -> bool {
        // if any rule disallows an inverted pair we return false
        !self.0
            .iter().any(|rule| (rule.0, rule.1) == (pair.1, pair.0))
    }

    fn get_applicable_for(&self, update: &Update) -> Rules {
        let applicable = self
            .0
            .iter()
            .filter(|(a, b)| update.contains(a) && update.contains(b))
            .cloned()
            .collect::<Vec<_>>();
        Rules::from(applicable)
    }
}

impl From<Vec<(usize, usize)>> for Rules {
    fn from(value: Vec<(usize, usize)>) -> Self {
        let mut value = value;
        value.sort();
        Rules(value)
    }
}

#[derive(Clone)]
struct Update(Vec<usize>);

impl Update {
    fn middle(&self) -> usize {
        self.0[self.0.len() / 2]
    }

    fn contains(&self, value: &usize) -> bool {
        self.0.contains(value)
    }

    fn allowed_by(&self, rules: &Rules) -> bool {
        let rules = rules.get_applicable_for(self);
        for left in 0..(self.0.len() - 1) {
            for right in (left + 1)..(self.0.len()) {
                if !rules.allow((self.0[left], self.0[right])) {
                    return false;
                }
            }
        }
        true
    }

    fn order_by(&self, rules: &Rules) -> Update {
        let mut entries = self.0.clone();
        let len = entries.len();

        for left in 0..(len - 1) {
            'foo: loop {
                for right in left..len {
                    if !rules.allow((entries[left], entries[right])) {
                        entries.swap(left, right);
                        continue 'foo;
                    }
                }
                break 'foo;
            }
        }

        Update(entries)
    }
}

mod test {
    

    #[test]
    fn test_rules() {
        let rules = Rules::from(vec![(1, 2), (2, 3)]);
        assert!(rules.allow((1, 2)));
        assert!(rules.allow((1, 3)));
        assert!(rules.allow((2, 3)));
        assert!(!rules.allow((2, 1)));
        assert!(rules.allow((3, 1)));
        assert!(!rules.allow((3, 2)));
    }

    #[test]
    fn foobar() {
        let rules = Rules::from(vec![(1, 2), (2, 3)]);
        let updates = [Update(vec![1, 2, 3]), Update(vec![2, 1, 3])];
        assert!(updates[0].allowed_by(&rules));
        assert!(!updates[1].allowed_by(&rules));
    }
}
