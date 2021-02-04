mod domain;

use aoc_lib::{Part1, Part2, Solution};
use domain::pocket::Pocket;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/17.txt"));

fn main() {
    Day17::default().solve_print(INPUT);
}

#[derive(Default)]
struct Day17;

impl Part1 for Day17 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        (0..6)
            .fold(Pocket::from(input), |mut pocket, _| {
                pocket.next();
                pocket
            })
            .size()
    }
}

impl Part2 for Day17 {
    type B = usize;

    fn solve(&self, _input: &str) -> Self::B {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{Day17, INPUT};
    use aoc_lib::{Part1, Part2};

    type Day = Day17;

    const EXAMPLE_INPUT: &str = "\
        .#.\n\
        ..#\n\
        ###";

    #[test]
    fn part1_example() {
        assert_eq!(Part1::solve(&Day::default(), EXAMPLE_INPUT), 112);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        // ...
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day::default(), INPUT), 230);
    }

    #[test]
    #[ignore]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day::default(), INPUT), Default::default());
    }
}
