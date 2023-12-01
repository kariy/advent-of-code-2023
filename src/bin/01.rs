advent_of_code::solution!(1);

use std::str::FromStr;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut values: Vec<u32> = vec![];

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut second: Option<char> = None;

        for char in line.chars() {
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = Some(char);
                }

                second = Some(char);
            }
        }

        let digit: u32 = u32::from_str(&format!("{}{}", first.unwrap(), second.unwrap())).unwrap();
        values.push(digit);
    }

    let sum = values.into_iter().reduce(|a, b| a + b);

    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    fn parse_digit_str(word: &str) -> u32 {
        if let Ok(digit) = word.parse() {
            digit
        } else {
            match word {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => {
                    panic!("invalid")
                }
            }
        }
    }

    let regex = Regex::new(r"[0-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        .unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let matches: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();

        let first = parse_digit_str(matches.first().unwrap());
        let last = parse_digit_str(matches.last().unwrap());
        let value = u32::from_str(&format!("{first}{last}")).unwrap();

        sum += value;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(
            r"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
        );
        assert_eq!(result, Some(281));
    }
}
