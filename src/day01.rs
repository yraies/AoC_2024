use crate::AdventOfCodeDay;
use std::collections::HashMap;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 1;
    type Parsed = (Vec<i64>, Vec<i64>);

    fn parse(input: String) -> Self::Parsed {
        let (mut left, mut right): (Vec<i64>, Vec<i64>) = input
            .lines()
            .map(|line| {
                line.split_once("   ")
                    .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
                    .unwrap()
            })
            .unzip();

        left.sort();
        right.sort();

        (left, right)
    }

    fn part_1((left, right): Self::Parsed) -> i64 {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn part_2((left, right): Self::Parsed) -> i64 {
        let mut right_counts = HashMap::<i64, i64>::new();
        right.iter().for_each(|&v| {
            right_counts
                .entry(v)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        left.iter()
            .map(|l| l * right_counts.get(l).unwrap_or(&0))
            .sum()
    }
}
