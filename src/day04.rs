use std::fmt::Write;

use itertools::Itertools;
use regex::Regex;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 4;
    type Parsed = Grid2D;

    fn parse(input: String) -> Self::Parsed {
        let vecvec = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Grid2D::new(vecvec)
    }

    fn part_1(grid: Self::Parsed) -> i64 {
        let horizontal = HorizontalIterator::from(grid.clone());
        let vertical = VerticalIterator::from(grid.clone());
        let diagonal = DiagonalIterator::from(grid);

        let iters = horizontal
            .into_iter()
            .chain(vertical)
            .chain(diagonal);

        let re1 = Regex::new("XMAS").unwrap();
        let re2 = Regex::new("SAMX").unwrap();

        let res = iters
            .map(|s| re1.find_iter(&s).count() + re2.find_iter(&s).count())
            .sum::<usize>();
        res as i64
    }

    fn part_2(grid: Self::Parsed) -> i64 {
        let re1 = Regex::new("MAS").unwrap();
        let re2 = Regex::new("SAM").unwrap();

        BlockIterator::from(grid)
            .filter(|block| {
                let diag = DiagonalIterator::from(block.to_owned());
                diag.into_iter()
                    .map(|d| re1.find_iter(&d).count() + re2.find_iter(&d).count())
                    .sum::<usize>()
                    == 2
            })
            .count() as i64
    }
}

#[derive(Clone, Debug)]
pub struct Grid2D {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid2D {
    fn new(vecvec: Vec<Vec<char>>) -> Grid2D {
        let height = vecvec.len();
        let width = vecvec[0].len();
        Grid2D {
            grid: vecvec,
            width,
            height,
        }
    }
}

impl std::fmt::Display for Grid2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().for_each(|line| {
            line.iter().for_each(|c| f.write_char(*c).unwrap());
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

struct HorizontalIterator {
    grid: Grid2D,
    row: usize,
}

impl HorizontalIterator {
    fn from(grid: Grid2D) -> HorizontalIterator {
        HorizontalIterator { grid, row: 0 }
    }
}

impl Iterator for HorizontalIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.height {
            let res = self.grid.grid[self.row].iter().collect();
            self.row += 1;
            Some(res)
        } else {
            None
        }
    }
}

struct VerticalIterator {
    grid: Grid2D,
    column: usize,
}

impl VerticalIterator {
    fn from(grid: Grid2D) -> VerticalIterator {
        VerticalIterator { grid, column: 0 }
    }
}

impl Iterator for VerticalIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column < self.grid.width {
            let res = self.grid.grid.iter().map(|row| row[self.column]).collect();
            self.column += 1;
            Some(res)
        } else {
            None
        }
    }
}

struct DiagonalIterator {
    grid: Grid2D,
    offset: usize,
    forward: bool,
}

impl DiagonalIterator {
    fn from(grid: Grid2D) -> DiagonalIterator {
        DiagonalIterator {
            grid,
            offset: 0,
            forward: false,
        }
    }
}

impl Iterator for DiagonalIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == (self.grid.width + self.grid.height - 1) {
            if self.forward {
                return None;
            } else {
                self.offset = 0;
                self.forward = true;
            }
        }

        let get_row = |row: usize| -> Option<char> {
            let DiagonalIterator {
                offset,
                grid,
                forward,
            } = &self;

            if *forward && *offset >= row {
                let index = offset - row;
                grid.grid[row].get(index).copied()
            } else if !*forward && *offset >= self.grid.height - row - 1 {
                let index = offset - (self.grid.height - row - 1);
                grid.grid[row].get(index).copied()
            } else {
                None
            }
        };

        let res = (0..self.grid.height)
            .filter_map(get_row)
            .collect();
        self.offset += 1;
        Some(res)
    }
}

struct BlockIterator {
    grid: Grid2D,
    offset_x: usize,
    offset_y: usize,
}

impl BlockIterator {
    fn from(grid: Grid2D) -> BlockIterator {
        BlockIterator {
            grid,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

impl Iterator for BlockIterator {
    type Item = Grid2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset_y + 3 > self.grid.width {
            return None;
        }

        let res = Grid2D::new(vec![
            vec![
                self.grid.grid[self.offset_x][self.offset_y],
                self.grid.grid[self.offset_x + 1][self.offset_y],
                self.grid.grid[self.offset_x + 2][self.offset_y],
            ],
            vec![
                self.grid.grid[self.offset_x][self.offset_y + 1],
                self.grid.grid[self.offset_x + 1][self.offset_y + 1],
                self.grid.grid[self.offset_x + 2][self.offset_y + 1],
            ],
            vec![
                self.grid.grid[self.offset_x][self.offset_y + 2],
                self.grid.grid[self.offset_x + 1][self.offset_y + 2],
                self.grid.grid[self.offset_x + 2][self.offset_y + 2],
            ],
        ]);
        self.offset_x += 1;
        if self.offset_x + 3 > self.grid.height {
            self.offset_x = 0;
            self.offset_y += 1;
        }

        Some(res)
    }
}
