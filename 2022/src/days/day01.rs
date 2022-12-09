use crate::days::{Day, Debug, Example, Part};

pub struct Day01;

impl Day for Day01 {
    fn number(&self) -> u32 {
        1
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day01 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let text = self.read_file(example);
        let elves = parse_elves(&text);
        let most_calories = elves.iter().map(|items| items.iter().sum()).max().unwrap();
        most_calories
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        let text = self.read_file(example);
        let elves = parse_elves(&text);
        let mut calories = elves
            .iter()
            .map(|items| items.iter().sum::<u32>())
            .collect::<Vec<_>>();
        calories.sort_unstable_by(|a, b| b.cmp(a));
        let top_3_sum = calories.iter().take(3).sum();
        top_3_sum
    }
}

fn parse_elves(text: &str) -> Vec<Vec<u32>> {
    text.split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|items: &_| !items.is_empty())
        .collect()
}
