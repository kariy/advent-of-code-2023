use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;

    let game_rg = Regex::new(r"Game (\d*)").unwrap();
    let green_rg = Regex::new(r"(\d+) (?:green)").unwrap();
    let blue_rg = Regex::new(r"(\d+) (?:blue)").unwrap();
    let red_rg = Regex::new(r"(\d+) (?:red)").unwrap();

    let mut possible_games: Vec<u32> = vec![];

    for line in input.lines() {
        let (game, sets) = line.split_once(':').unwrap();
        let num = game_rg
            .captures(game)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let mut possible = true;

        for set in sets.split(';') {
            let green_num = green_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            let blue_num = blue_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            let red_num = red_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            if green_num > MAX_GREEN_CUBES || blue_num > MAX_BLUE_CUBES || red_num > MAX_RED_CUBES {
                possible = false;
                break;
            }
        }

        if possible {
            possible_games.push(num);
        }
    }

    possible_games.into_iter().reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u32> {
    let green_rg = Regex::new(r"(\d+) (?:green)").unwrap();
    let blue_rg = Regex::new(r"(\d+) (?:blue)").unwrap();
    let red_rg = Regex::new(r"(\d+) (?:red)").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let sets = line.split_once(':').unwrap().1;

        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for set in sets.split(';') {
            let green_num = green_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            if green_num > min_green {
                min_green = green_num;
            }

            let blue_num = blue_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            if blue_num > min_blue {
                min_blue = blue_num;
            }

            let red_num = red_rg
                .captures(set)
                .map(|m| {
                    let digit = m.get(1).unwrap().as_str();
                    digit.parse::<u32>().unwrap()
                })
                .unwrap_or_default();

            if red_num > min_red {
                min_red = red_num;
            }
        }

        sum += min_blue * min_red * min_green;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
