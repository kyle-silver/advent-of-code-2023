const INPUT: &str = include_str!("inputs/01.txt");

fn calibration_value(input: &str) -> u32 {
    let first = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    return first * 10 + last;
}

fn calibration_value_part2(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    let first = first_digit(&chars)?;
    let last = last_digit(&chars)?;
    Some(first * 10 + last)
}

fn first_digit(input: &[char]) -> Option<u32> {
    for i in 0..input.len() {
        if let Some(digit) = digit(&input[i..]) {
            return Some(digit);
        }
    }
    None
}

fn last_digit(input: &[char]) -> Option<u32> {
    for i in (0..input.len()).rev() {
        if let Some(digit) = digit(&input[i..]) {
            return Some(digit);
        }
    }
    None
}

fn digit(substring: &[char]) -> Option<u32> {
    match substring {
        ['1', ..] | ['o', 'n', 'e', ..] => Some(1),
        ['2', ..] | ['t', 'w', 'o', ..] => Some(2),
        ['3', ..] | ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
        ['4', ..] | ['f', 'o', 'u', 'r', ..] => Some(4),
        ['5', ..] | ['f', 'i', 'v', 'e', ..] => Some(5),
        ['6', ..] | ['s', 'i', 'x', ..] => Some(6),
        ['7', ..] | ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
        ['8', ..] | ['e', 'i', 'g', 'h', 't', ..] => Some(8),
        ['9', ..] | ['n', 'i', 'n', 'e', ..] => Some(9),
        _ => None,
    }
}

#[test]
fn part1() {
    let ans: u32 = INPUT.lines().map(calibration_value).sum();
    println!("Day 1, part 1: {ans}");
    assert_eq!(54968, ans);
}

#[test]
fn part2() {
    let ans: u32 = INPUT.lines().filter_map(calibration_value_part2).sum();
    println!("Day 1, part 2: {ans}");
    assert_eq!(54094, ans)
}
