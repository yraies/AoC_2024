use std::cmp::Ordering;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 2;
    type Parsed = Vec<Report>;

    fn parse(input: String) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let levels = line
                    .split(" ")
                    .map(|level| level.parse::<i64>().unwrap())
                    .collect();
                Report(levels)
            })
            .collect()
    }

    fn part_1(reports: Self::Parsed) -> i64 {
        reports.iter().filter(|report| report.is_safe()).count() as i64
    }

    fn part_2(reports: Self::Parsed) -> i64 {
        reports
            .iter()
            .filter(|report| report.is_safe_with_problem_dampener())
            .count() as i64
    }
}

#[derive(Clone, Debug)]
pub struct Report(Vec<i64>);

impl Report {
    fn is_safe_with_problem_dampener(&self) -> bool {
        self.is_safe()
            || (0..self.0.len())
                .into_iter()
                .any(|idx| self.with_idx_removed(idx).is_safe())
    }

    fn with_idx_removed(&self, idx: usize) -> Report {
        let mut other = self.clone();
        other.0.remove(idx);
        other
    }

    fn is_safe(&self) -> bool {
        let levels = &self.0;
        if levels.len() <= 1 {
            return true;
        }
        let direction = levels[0].cmp(&levels[1]);
        if matches!(direction, Ordering::Equal) {
            return false;
        }
        let res = levels[1..].iter().fold(Ok(levels[0]), |acc, next| {
            acc.and_then(|last| {
                let diff = (last - next).abs();
                if last.cmp(next).eq(&direction) && diff > 0 && diff <= 3 {
                    Ok(*next)
                } else {
                    Err(())
                }
            })
        });
        res.is_ok()
    }
}
