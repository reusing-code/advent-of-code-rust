use std::collections::HashMap;

use advent_of_code::split_by_empt_line;

advent_of_code::solution!(19);

fn pattern_match_rec(
    design: &str,
    patterns: &Vec<String>,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if design.len() == 0 {
        return 1;
    }

    let v = cache.get(design);
    if v.is_some() {
        return *v.unwrap();
    }

    let mut result = 0;

    for pat in patterns {
        if design.starts_with(pat) {
            result += pattern_match_rec(&design[pat.len()..], patterns, cache);
        }
    }
    cache.insert(design.to_string(), result);
    return result;
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let split_emptyline = split_by_empt_line(input);
    let patterns = split_emptyline[0][0]
        .split(",")
        .map(|x| String::from(x.trim()))
        .collect::<Vec<_>>();

    (patterns, split_emptyline[1].clone())
}

pub fn part_one(input: &str) -> Option<i64> {
    let (patterns, designs) = parse_input(input);
    let mut result = 0;
    let mut cache = HashMap::<String, i64>::new();
    for design in designs {
        if pattern_match_rec(&design, &patterns, &mut cache) > 0 {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (patterns, designs) = parse_input(input);
    let mut result = 0;
    let mut cache = HashMap::<String, i64>::new();
    for design in designs {
        result += pattern_match_rec(&design, &patterns, &mut cache);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
