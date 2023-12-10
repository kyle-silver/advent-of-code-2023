use std::collections::VecDeque;

const INPUT: &str = include_str!("inputs/04.txt");

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Option<Self> {
        let (_, numbers) = line.split_once(": ")?;
        let (winning_numbers, card_numbers) = numbers.split_once(" | ")?;
        let winning_numbers = winning_numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let card_numbers = card_numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Some(Self {
            winning_numbers,
            card_numbers,
        })
    }

    fn matches(&self) -> impl Iterator<Item = &u32> {
        self.card_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
    }

    fn points(&self) -> u32 {
        let matches = self.matches().count();
        if matches == 0 {
            return 0;
        }
        return 2u32.pow(matches as u32 - 1);
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .lines()
        .filter_map(Card::parse)
        .map(|card| card.points())
        .sum();
    println!("Day 4, part 1: {ans}");
    assert_eq!(21485, ans);
}

#[test]
fn part2() {
    // parse the cards like normal
    let cards: Vec<_> = INPUT.lines().filter_map(Card::parse).collect();

    // pre-compute all of the matches; if we had to do this every time we added
    // a new card, it would take orders of magnitude longer to find the solution
    let matches: Vec<Vec<usize>> = cards
        .iter()
        .map(|card| card.matches().map(|id| *id as usize).collect())
        .collect();

    // keep track of how many copies of each card we've won
    let mut counts = vec![0; cards.len()];
    let mut buffer: VecDeque<_> = (0..cards.len()).collect();
    while let Some(id) = buffer.pop_front() {
        counts[id] += 1;
        for (match_number, _) in matches[id].iter().enumerate() {
            let next_card_id = id + match_number + 1;
            buffer.push_back(next_card_id);
        }
    }
    let ans: u32 = counts.iter().sum();
    println!("Day 4, part 2: {ans}");
    assert_eq!(11024379, ans);
}
