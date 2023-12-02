use crate::days::{Day, Debug, Example, IResult, Part};
use enum_ordinalize::Ordinalize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::character::complete::newline;
use nom::combinator::{map, map_opt, opt, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};

pub struct Day02;

impl Day for Day02 {
    fn number(&self) -> u32 {
        2
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        parse_games(&self.read_file(Part::Part1, example))
            .unwrap()
            .into_iter()
            .filter(|game| game.possible_with(BAG))
            .map(|game| game.id.0)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }
}

const BAG: Collection = Collection {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Game {
    id: GameId,
    rounds: Vec<Collection>,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct GameId(u32);

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Collection {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Hash, Ordinalize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct CubeInfo {
    count: u32,
    color: Color,
}

impl Game {
    pub fn possible_with(&self, collection: Collection) -> bool {
        self.rounds
            .iter()
            .all(|round| collection.is_superset_of(*round))
    }
}

impl Collection {
    pub fn is_superset_of(self, other: Collection) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

impl Collection {
    pub fn from_cube_infos(cubes: Vec<CubeInfo>) -> Option<Self> {
        if cubes.len() > Color::VARIANT_COUNT {
            return None;
        }

        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for cube in cubes {
            let variable = match cube.color {
                Color::Red => &mut red,
                Color::Green => &mut green,
                Color::Blue => &mut blue,
            };
            if variable.replace(cube.count).is_some() {
                return None;
            }
        }

        Some(Self {
            red: red.unwrap_or(0),
            green: green.unwrap_or(0),
            blue: blue.unwrap_or(0),
        })
    }
}

fn parse_games(input: &str) -> Result<Vec<Game>, String> {
    let (i, games) = terminated(games, opt(newline))(input).map_err(|e| e.to_string())?;
    if !i.is_empty() {
        return Err(format!("Input string not empty. Remaining string: \"{i}\""));
    }
    Ok(games)
}

fn games(i: &str) -> IResult<Vec<Game>> {
    separated_list1(newline, game)(i)
}

fn game(i: &str) -> IResult<Game> {
    map(
        separated_pair(id, tag(": "), collections),
        |(id, rounds)| Game { id, rounds },
    )(i)
}

fn id(i: &str) -> IResult<GameId> {
    map(preceded(tag("Game "), character::u32), GameId)(i)
}

fn collections(i: &str) -> IResult<Vec<Collection>> {
    separated_list1(tag("; "), round)(i)
}

fn round(i: &str) -> IResult<Collection> {
    map_opt(
        separated_list1(tag(", "), cube_info),
        Collection::from_cube_infos,
    )(i)
}

fn cube_info(i: &str) -> IResult<CubeInfo> {
    map(
        separated_pair(character::u32, tag(" "), color),
        |(count, color)| CubeInfo { count, color },
    )(i)
}

fn color(i: &str) -> IResult<Color> {
    alt((red, green, blue))(i)
}

fn red(i: &str) -> IResult<Color> {
    value(Color::Red, tag("red"))(i)
}

fn green(i: &str) -> IResult<Color> {
    value(Color::Green, tag("green"))(i)
}

fn blue(i: &str) -> IResult<Color> {
    value(Color::Blue, tag("blue"))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Color::Red, color("red").unwrap().1);
        assert_eq!(
            CubeInfo {
                color: Color::Blue,
                count: 3
            },
            cube_info("3 blue").unwrap().1
        );
        assert_eq!(
            Collection {
                red: 4,
                green: 0,
                blue: 3,
            },
            round("3 blue, 4 red").unwrap().1
        );

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (i, actual) = game(input).unwrap();
        assert!(i.is_empty());
        let rgb = |red, green, blue| Collection { red, green, blue };
        let expected = Game {
            id: GameId(1),
            rounds: vec![rgb(4, 0, 3), rgb(1, 2, 6), rgb(0, 2, 0)],
        };
        assert_eq!(expected, actual);

        assert!(parse_games(&Day02.read_file(Part::Part1, Example::Example)).is_ok());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!("8", Day02.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("2006", Day02.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day2.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day2.part2(Example::Real, Debug::NotDebug));
    }
}
