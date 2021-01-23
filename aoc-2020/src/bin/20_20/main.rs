mod domain;

use aoc_lib::{solve_print, Part1, Part2};
use domain::{
    image::{Image, SEA_MONSTER_X_COUNT},
    orient::Orientations,
    tile::{Tiles, X},
};

const INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/20.txt"));

fn main() {
    solve_print(Box::new(Day20::default()), INPUT);
}

#[derive(Default)]
struct Day20;

impl Part1 for Day20 {
    type A = usize;

    fn solve(&self, input: &str) -> Self::A {
        Tiles::from(input)
            .corners()
            .map(|(_, tile)| tile.id)
            .product()
    }
}

impl Part2 for Day20 {
    type B = usize;

    fn solve(&self, input: &str) -> Self::B {
        let image = Image::from(Tiles::from(input));

        let (image, sea_monsters) = Orientations::from(image)
            .map(|image| {
                let sm = image.sea_monsters();
                (image, sm)
            })
            .skip_while(|(_, count)| *count == 0)
            .next()
            .expect("Could not find any sea monster in any orientation :(");

        image
            .0
            .iter()
            .map(|row| row.iter().filter(|cell| **cell == X).count())
            .sum::<usize>()
            - (sea_monsters * SEA_MONSTER_X_COUNT)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day20, INPUT};
    use aoc_lib::{Part1, Part2};

    #[test]
    fn part1_example() {
        assert_eq!(
            Part1::solve(&Day20::default(), EXAMPLE_INPUT),
            20899048083289
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(Part2::solve(&Day20::default(), EXAMPLE_INPUT), 273);
    }

    #[test]
    fn part1_answer() {
        assert_eq!(Part1::solve(&Day20::default(), INPUT), 19955159604613);
    }

    #[test]
    fn part2_answer() {
        assert_eq!(Part2::solve(&Day20::default(), INPUT), 1639);
    }

    const EXAMPLE_INPUT: &'static str = "\
        Tile 2311:\n\
        ..##.#..#.\n\
        ##..#.....\n\
        #...##..#.\n\
        ####.#...#\n\
        ##.##.###.\n\
        ##...#.###\n\
        .#.#.#..##\n\
        ..#....#..\n\
        ###...#.#.\n\
        ..###..###\n\
        \n\
        Tile 1951:\n\
        #.##...##.\n\
        #.####...#\n\
        .....#..##\n\
        #...######\n\
        .##.#....#\n\
        .###.#####\n\
        ###.##.##.\n\
        .###....#.\n\
        ..#.#..#.#\n\
        #...##.#..\n\
        \n\
        Tile 1171:\n\
        ####...##.\n\
        #..##.#..#\n\
        ##.#..#.#.\n\
        .###.####.\n\
        ..###.####\n\
        .##....##.\n\
        .#...####.\n\
        #.##.####.\n\
        ####..#...\n\
        .....##...\n\
        \n\
        Tile 1427:\n\
        ###.##.#..\n\
        .#..#.##..\n\
        .#.##.#..#\n\
        #.#.#.##.#\n\
        ....#...##\n\
        ...##..##.\n\
        ...#.#####\n\
        .#.####.#.\n\
        ..#..###.#\n\
        ..##.#..#.\n\
        \n\
        Tile 1489:\n\
        ##.#.#....\n\
        ..##...#..\n\
        .##..##...\n\
        ..#...#...\n\
        #####...#.\n\
        #..#.#.#.#\n\
        ...#.#.#..\n\
        ##.#...##.\n\
        ..##.##.##\n\
        ###.##.#..\n\
        \n\
        Tile 2473:\n\
        #....####.\n\
        #..#.##...\n\
        #.##..#...\n\
        ######.#.#\n\
        .#...#.#.#\n\
        .#########\n\
        .###.#..#.\n\
        ########.#\n\
        ##...##.#.\n\
        ..###.#.#.\n\
        \n\
        Tile 2971:\n\
        ..#.#....#\n\
        #...###...\n\
        #.#.###...\n\
        ##.##..#..\n\
        .#####..##\n\
        .#..####.#\n\
        #..#.#..#.\n\
        ..####.###\n\
        ..#.#.###.\n\
        ...#.#.#.#\n\
        \n\
        Tile 2729:\n\
        ...#.#.#.#\n\
        ####.#....\n\
        ..#.#.....\n\
        ....#..#.#\n\
        .##..##.#.\n\
        .#.####...\n\
        ####.#.#..\n\
        ##.####...\n\
        ##..#.##..\n\
        #.##...##.\n\
        \n\
        Tile 3079:\n\
        #.#.#####.\n\
        .#..######\n\
        ..#.......\n\
        ######....\n\
        ####.#..#.\n\
        .#...#.##.\n\
        #.#####.##\n\
        ..#.###...\n\
        ..#.......\n\
        ..#.###...\n";
}
