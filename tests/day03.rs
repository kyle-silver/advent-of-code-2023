use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

const INPUT: &str = include_str!("inputs/03.txt");

#[derive(Debug)]
enum Entry {
    Digit(u32),
    Symbol(char),
}

#[derive(Debug)]
struct Schematic(HashMap<(i32, i32), Entry>);

impl Schematic {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut schematic = HashMap::new();
        for (row, line) in lines.enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                let value = if let Some(digit) = c.to_digit(10) {
                    Entry::Digit(digit)
                } else {
                    Entry::Symbol(c)
                };
                schematic.insert((row as i32, col as i32), value);
            }
        }
        Schematic(schematic)
    }

    fn new_part_numbers(
        &self,
        (row, col): (i32, i32),
        visited: &mut HashSet<(i32, i32)>,
    ) -> Vec<u32> {
        static DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        DIRECTIONS
            .iter()
            .map(|(d_row, d_col)| (row + d_row, col + d_col))
            .filter_map(|point| self.build_number(point, visited))
            .collect()
    }

    fn build_number(
        &self,
        (row, col): (i32, i32),
        visited: &mut HashSet<(i32, i32)>,
    ) -> Option<u32> {
        if visited.contains(&(row, col)) {
            return None;
        }
        let mut buffer = VecDeque::new();
        if let Some(Entry::Digit(d)) = self.0.get(&(row, col)) {
            buffer.push_front(*d);
            visited.insert((row, col));
        } else {
            return None;
        }
        let mut search_left = col;
        while let Some(Entry::Digit(d)) = self.0.get(&(row, search_left - 1)) {
            if visited.contains(&(row, search_left - 1)) {
                return None;
            }
            buffer.push_front(*d);
            visited.insert((row, search_left - 1));
            search_left -= 1;
        }
        let mut search_right = col;
        while let Some(Entry::Digit(d)) = self.0.get(&(row, search_right + 1)) {
            if visited.contains(&(row, search_right + 1)) {
                return None;
            }
            buffer.push_back(*d);
            visited.insert((row, search_right + 1));
            search_right += 1;
        }
        let number: u32 = buffer
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &d)| d * 10u32.pow(i as u32))
            .sum();
        Some(number)
    }
}

impl Display for Schematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..10 {
            for col in 0..10 {
                let to_print = match self.0.get(&(row, col)) {
                    Some(Entry::Digit(d)) => char::from_digit(*d, 10).unwrap(),
                    Some(Entry::Symbol(s)) => *s,
                    None => '.',
                };
                write!(f, "{to_print}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn part1() {
    let schematic = Schematic::parse(INPUT.lines());
    let mut visited = HashSet::new();
    let mut ans = 0;
    for ((row, col), entry) in schematic.0.iter() {
        match entry {
            Entry::Symbol(_) => {
                let part_numbers = schematic.new_part_numbers((*row, *col), &mut visited);
                for part_number in part_numbers {
                    ans += part_number;
                }
            }
            _ => {}
        }
    }
    println!("Day 3, part 1: {ans}");
    assert_eq!(549908, ans);
}

#[test]
fn part2() {
    let schematic = Schematic::parse(INPUT.lines());
    let mut ans = 0;
    for ((row, col), entry) in schematic.0.iter() {
        match entry {
            Entry::Symbol(_) => {
                let part_numbers = schematic.new_part_numbers((*row, *col), &mut HashSet::new());
                if part_numbers.len() != 2 {
                    continue;
                }
                let gear_ratio = part_numbers[0] * part_numbers[1];
                ans += gear_ratio;
            }
            _ => {}
        }
    }
    println!("Day 3, part 2: {ans}");
    assert_eq!(81166799, ans);
}
