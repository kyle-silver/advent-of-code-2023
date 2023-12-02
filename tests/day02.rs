const INPUT: &str = include_str!("inputs/02.txt");

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn parse(input: &str) -> Self {
        let mut round = Round {
            red: 0,
            blue: 0,
            green: 0,
        };
        input
            .split(",")
            .map(|token| token.trim())
            .map(|count| {
                let (count, color) = count.split_once(" ").unwrap();
                (count.to_string(), color.to_string())
            })
            .map(|(count, color)| (color, count.parse().unwrap()))
            .for_each(|(color, count)| match color.as_str() {
                "red" => round.red = count,
                "green" => round.green = count,
                "blue" => round.blue = count,
                _ => panic!(),
            });
        round
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(input: &str) -> Option<Self> {
        let (id, rounds) = input.split_once(":")?;
        let (_, id) = id.split_once(" ")?;
        let id = id.parse().ok()?;
        let rounds = rounds.split(";").map(Round::parse).collect();
        Some(Self { id, rounds })
    }

    fn possible(&self) -> bool {
        self.rounds.iter().all(Round::possible)
    }

    fn min_round(&self) -> Round {
        Round {
            red: self.rounds.iter().map(|r| r.red).max().unwrap_or(0),
            green: self.rounds.iter().map(|r| r.green).max().unwrap_or(0),
            blue: self.rounds.iter().map(|r| r.blue).max().unwrap_or(0),
        }
    }

    fn power(&self) -> u32 {
        self.min_round().power()
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .lines()
        .filter_map(Game::parse)
        .filter(Game::possible)
        .map(|game| game.id)
        .sum();
    println!("Day 2, part 1: {ans}");
    assert_eq!(2156, ans);
}

#[test]
fn part2() {
    let ans: u32 = INPUT
        .lines()
        .filter_map(Game::parse)
        .map(|g| g.power())
        .sum();
    println!("Day 2, part 2: {ans}");
    assert_eq!(66909, ans);
}
