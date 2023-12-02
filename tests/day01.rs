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
    let mut matches = vec![];
    for i in 0..input.len() {
        let substr: String = input.chars().skip(i).take(5).collect();
        if substr.starts_with("1") || substr.starts_with("one") {
            matches.push(1);
        } else if substr.starts_with("2") || substr.starts_with("two") {
            matches.push(2);
        } else if substr.starts_with("3") || substr.starts_with("three") {
            matches.push(3);
        } else if substr.starts_with("4") || substr.starts_with("four") {
            matches.push(4);
        } else if substr.starts_with("5") || substr.starts_with("five") {
            matches.push(5);
        } else if substr.starts_with("6") || substr.starts_with("six") {
            matches.push(6);
        } else if substr.starts_with("7") || substr.starts_with("seven") {
            matches.push(7);
        } else if substr.starts_with("8") || substr.starts_with("eight") {
            matches.push(8);
        } else if substr.starts_with("9") || substr.starts_with("nine") {
            matches.push(9);
        }
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
}
