use crate::AdventOfCodeDay;
use regex::Regex;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 3;
    type Parsed = String;

    fn parse(input: String) -> Self::Parsed {
        input
    }

    fn part_1(input: Self::Parsed) -> i64 {
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let input: Vec<(i64, i64)> = re
            .captures_iter(&input)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                )
            })
            .collect();

        input.iter().map(|(a, b)| a * b).sum()
    }

    fn part_2(input: Self::Parsed) -> i64 {
        let re = Regex::new(r"(m)ul\(([0-9]{1,3}),([0-9]{1,3})\)|(d)o\(\)|do(n)'t\(\)").unwrap();

        let mut enabled = true;
        let mut sum = 0;

        for capture in re.captures_iter(&input) {
            let (is_mul, is_do, is_dont) = (
                capture.get(1).is_some(),
                capture.get(4).is_some(),
                capture.get(5).is_some(),
            );
            if is_mul && enabled {
                let a = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let b = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
                sum += a * b;
            } else if is_do {
                enabled = true;
            } else if is_dont {
                enabled = false;
            }
        }
        sum
    }
}
