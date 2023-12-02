const INPUT: &str = include_str!("inputs/01.txt");

fn calibration_value(input: &str) -> u32 {
    let first = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    return first * 10 + last;
}

fn calibration_value_part2(input: &str) -> u32 {
    let digits = extract_digits(input);
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    first * 10 + last
}

fn extract_digits(input: &str) -> Vec<u32> {
    // you'd think a regex would be the right solution here, but it actually
    // isn't!! it doesn't work!! the number words can overlap!!!
    let chars: Vec<_> = input.chars().collect();
    let mut matches = vec![];
    for i in 0..input.len() {
        match &chars[i..] {
            ['1', ..] | ['o', 'n', 'e', ..] => matches.push(1),
            ['2', ..] | ['t', 'w', 'o', ..] => matches.push(2),
            ['3', ..] | ['t', 'h', 'r', 'e', 'e', ..] => matches.push(3),
            ['4', ..] | ['f', 'o', 'u', 'r', ..] => matches.push(4),
            ['5', ..] | ['f', 'i', 'v', 'e', ..] => matches.push(5),
            ['6', ..] | ['s', 'i', 'x', ..] => matches.push(6),
            ['7', ..] | ['s', 'e', 'v', 'e', 'n', ..] => matches.push(7),
            ['8', ..] | ['e', 'i', 'g', 'h', 't', ..] => matches.push(8),
            ['9', ..] | ['n', 'i', 'n', 'e', ..] => matches.push(9),
            _ => {}
        };
    }
    matches
}

#[test]
fn part1() {
    let ans: u32 = INPUT.lines().map(calibration_value).sum();
    println!("Day 1, part 1: {ans}");
    assert_eq!(54968, ans);
}

#[test]
fn part2() {
    let ans: u32 = INPUT.lines().map(calibration_value_part2).sum();
    println!("Day 1, part 2: {ans}");
    assert_eq!(54094, ans)
}
