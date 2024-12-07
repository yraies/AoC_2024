use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    const DAY: usize = 6;
    type Parsed = Map;

    fn parse(input: String) -> Self::Parsed {
        let width = input.lines().peekable().next().unwrap().len();
        let height = input.lines().count();
        let grid = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstructed,
                    '^' => Tile::Guard(Dir::Up),
                    'v' => Tile::Guard(Dir::Down),
                    '<' => Tile::Guard(Dir::Left),
                    '>' => Tile::Guard(Dir::Right),
                    _ => unimplemented!(),
                })
            })
            .collect::<Vec<Tile>>();

        Map::new(grid, width, height)
    }

    fn part_1(mut map: Self::Parsed) -> i64 {
        while map.guard_idx.is_some() {
            map.step();
        }

        map.grid
            .iter()
            .filter(|t| matches!(t, Tile::Marked))
            .count() as i64
    }

    fn part_2(map: Self::Parsed) -> i64 {
        let lines = map.clone().get_all_lines();

        let mut obstacles = lines
            .iter()
            .fold(
                (vec![], vec![]),
                |(mut prev_lines, mut obstacles), next_line| {
                    for prev_line in &prev_lines {
                        if let Some(obstacle) = try_get_obstacle(next_line, &prev_line, &map) {
                            obstacles.push(obstacle);
                        }
                    }

                    prev_lines.push(extend_line(&next_line, &map));
                    (prev_lines, obstacles)
                },
            )
            .1;

        println!("Obstacles: {:?}", obstacles);
        obstacles.sort();
        obstacles.dedup();
        obstacles.len() as i64
    }
}

fn extend_line(line: &Line, map: &Map) -> Line {
    let mut from = line.from;
    let rev_dir = line.dir.rotate().rotate();
    loop {
        let new_from = rev_dir.step(from);
        if map.out_of_bounds(new_from) || map.grid[map.index_from(new_from)] == Tile::Obstructed {
            break;
        }
        from = new_from;
    }
    Line::new(line.dir, from, line.to)
}

fn try_get_obstacle(next_line: &Line, prev_line: &Line, map: &Map) -> Option<(i64, i64)> {
    if next_line.dir.rotate() != prev_line.dir {
        return None;
    }

    if let Some(intersection) = next_line.intersects_with(&prev_line) {
        //println!(
        //    "Found intersection at {:?} for lines\n{:?}\n{:?}",
        //    intersection, prev_line, next_line
        //);
        let new_obstacle_coord = next_line.dir.step(intersection);
        if map.grid[map.index_from(new_obstacle_coord)] != Tile::Obstructed {
            return Some(new_obstacle_coord);
        }
    } else {
        //println!(
        //    "Found no intersection for lines\n{:?}\n{:?}",
        //    prev_line, next_line
        //);
    }

    None
}

#[derive(Clone)]
pub struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
    guard_idx: Option<usize>,
}

impl Map {
    fn new(grid: Vec<Tile>, width: usize, height: usize) -> Map {
        let guard_idx = grid
            .iter()
            .enumerate()
            .find_map(|(index, t)| matches!(t, Tile::Guard(_)).then_some(index));

        Map {
            grid,
            width,
            height,
            guard_idx,
        }
    }

    fn step(&mut self) -> StepEvent {
        if self.guard_idx.is_none() {
            println!("simulated too much");
            return StepEvent::Exit;
        }
        let guard_idx = self.guard_idx.unwrap();
        let guard_coord = self.coordinate_from(guard_idx);
        let guard_dir = self.unsafe_guard_dir();
        let new_coord = guard_dir.step(guard_coord);

        if self.out_of_bounds(new_coord) {
            self.grid[guard_idx] = Tile::Marked;
            self.guard_idx = None;
            return StepEvent::Exit;
        } else {
            let new_idx = self.index_from(new_coord);

            if self.is_obstructed(new_idx) {
                let rotated = guard_dir.rotate();
                self.grid[guard_idx] = Tile::Guard(rotated);
                return StepEvent::Rotate(rotated);
            } else {
                self.grid[guard_idx] = Tile::Marked;
                self.grid[new_idx] = Tile::Guard(guard_dir);
                self.guard_idx = Some(new_idx);
                return StepEvent::Forward;
            }
        }
    }

    fn get_all_lines(mut self) -> Vec<Line> {
        let mut last_dir = self.unsafe_guard_dir();
        let mut last_coord = self.coordinate_from(self.guard_idx.unwrap());

        let mut lines = vec![];
        let mut event = StepEvent::Forward;
        loop {
            if let StepEvent::Rotate(to) = event {
                let coord = self.coordinate_from(self.guard_idx.unwrap());
                if coord.ne(&last_coord) {
                    lines.push(Line::new(last_dir, last_coord, coord));
                }
                last_dir = to;
                last_coord = coord;
            } else if let StepEvent::Exit = event {
                let coord = match last_dir {
                    Dir::Up => (last_coord.0, 0),
                    Dir::Down => (last_coord.0, self.height as i64 - 1),
                    Dir::Left => (0, last_coord.1),
                    Dir::Right => (self.width as i64 - 1, last_coord.1),
                };
                if coord.ne(&last_coord) {
                    lines.push(Line::new(last_dir, last_coord, coord));
                }
                break;
            }
            event = self.step();
        }

        lines
    }

    fn unsafe_guard_dir(&self) -> Dir {
        match self.grid[self.guard_idx.unwrap()].clone() {
            Tile::Guard(g) => g,
            _ => unreachable!(),
        }
    }

    fn is_obstructed(&mut self, new_idx: usize) -> bool {
        match self.grid[new_idx] {
            Tile::Obstructed => true,
            Tile::Empty => false,
            Tile::Marked => false,
            Tile::Guard(_) => true,
        }
    }

    fn out_of_bounds(&self, new_coord: (i64, i64)) -> bool {
        new_coord.0 < 0
            || new_coord.0 >= (self.width as i64)
            || new_coord.1 < 0
            || new_coord.1 >= (self.height as i64)
    }

    fn coordinate_from(&self, index: usize) -> (i64, i64) {
        (
            index.rem_euclid(self.width) as i64,
            index.div_euclid(self.width) as i64,
        )
    }

    fn index_from(&self, coord: (i64, i64)) -> usize {
        coord.1 as usize * self.width + coord.0 as usize
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstructed,
    Marked,
    Guard(Dir),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rotate(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn step(&self, coord: (i64, i64)) -> (i64, i64) {
        match self {
            Dir::Up => (coord.0, coord.1 - 1),
            Dir::Down => (coord.0, coord.1 + 1),
            Dir::Left => (coord.0 - 1, coord.1),
            Dir::Right => (coord.0 + 1, coord.1),
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Dir::Up => false,
            Dir::Down => false,
            Dir::Left => true,
            Dir::Right => true,
        }
    }
}

#[derive(Clone, Debug)]
enum StepEvent {
    Forward,
    Rotate(Dir),
    Exit,
}

#[derive(Clone, Debug)]
struct Line {
    dir: Dir,
    from: (i64, i64),
    to: (i64, i64),
}

impl Line {
    fn new(dir: Dir, from: (i64, i64), to: (i64, i64)) -> Line {
        assert!(
            (from.1 == to.1 && (matches!(dir, Dir::Left) || matches!(dir, Dir::Right)))
                || (from.0 == to.0 && (matches!(dir, Dir::Up) || matches!(dir, Dir::Down)))
        );
        Line { dir, from, to }
    }

    fn intersects_with(&self, other: &Line) -> Option<(i64, i64)> {
        // Ensure that the lines do not start or end at the same point
        // if self.from == other.from
        //     || self.to == other.to
        //     || self.from == other.to
        //     || self.to == other.from
        // {
        //     return None;
        // }

        // parallel lines don't intersect in our terms
        if self.dir.is_horizontal() == other.dir.is_horizontal() {
            return None;
        }

        if self.dir.is_horizontal() && !other.dir.is_horizontal() {
            let self_y = self.from.1;
            let other_x = other.from.0;

            // Check if the x of the vertical line is within the range of the horizontal line
            if (self.from.0.min(self.to.0)..=self.from.0.max(self.to.0)).contains(&other_x)
                && (other.from.1.min(other.to.1)..=other.from.1.max(other.to.1)).contains(&self_y)
            {
                return Some((other_x, self_y));
            } else {
                return None;
            }
        } else {
            other.intersects_with(self)
        }
    }
}

#[allow(unused_imports)]
mod test {
    use super::Line;
    use crate::day06::*;

    #[test]
    fn test_lines() {
        let line1 = Line::new(Dir::Right, (0, 1), (9, 1));
        let line2 = Line::new(Dir::Down, (9, 0), (9, 9));

        assert!(line1.intersects_with(&line2).is_some());
    }
}
