use crate::day4::parse;
use std::fs::read_to_string;

pub fn run() {
    println!("aoc 2022 day 4 part 2");

    let contents = read_to_string("src/day4/data.in").expect("no input file found");
    println!("{}", implementation(&contents));
}

pub fn implementation(inp: &str) -> usize {
    parse(inp)
        .filter(|((a, b), (c, d))| a <= d && c <= b)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day4::part2::implementation;
    use std::fs::read_to_string;

    #[test]
    pub fn test_day_4_part_2() {
        let contents = read_to_string("src/day4/data.in").expect("no input file found");
        assert_eq!(implementation(&contents), 881)
    }

    #[test]
    pub fn test_day_4_part_2_test_input() {
        let testdata = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(implementation(testdata), 4)
    }
}
