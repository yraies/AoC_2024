use time::OffsetDateTime;

mod day01;

fn main() {
    let days = [day01::Day1::print];

    let today = OffsetDateTime::now_utc().day() as usize;
    if today + 1 <= days.len() {
        println!("Outputting Day {}", today);
        days[today]();
    } else {
        println!("Outputting All Days");
        for day in days {
            day();
        }
    }
}

trait AdventOfCodeDay {
    const DAY: usize;
    type Parsed: Clone;

    fn parse(input: String) -> Self::Parsed;
    fn part_1(parsed: Self::Parsed) -> i64;
    fn part_2(parsed: Self::Parsed) -> i64;

    fn load() -> String {
        std::fs::read_to_string(format!("inputs/{:0>2}.txt", Self::DAY.to_string())).unwrap()
    }
    fn print() {
        let input = Self::load();
        let parsed = Self::parse(input);
        let res1 = Self::part_1(parsed.clone());
        println!("Result Day {:02} Part 1: {}", Self::DAY, res1);
        let res2 = Self::part_2(parsed);
        println!("Result Day {:02} Part 2: {}", Self::DAY, res2);
    }
}