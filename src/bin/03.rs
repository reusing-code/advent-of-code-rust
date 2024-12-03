use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result: u32 = 0;
    for (_, [f1, f2]) in re.captures_iter(input).map(|c| c.extract()) {
        result += f1.parse::<u32>().unwrap() * f2.parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut doo = true;
    let mut i: usize = 0;
    loop {
        if doo {
            let dont = input[i..].find("don't()");
            let end = dont.or(Some(input[i..].len())).unwrap() + i;
            result += part_one(&input[i..end]).unwrap();
            i = end;
            doo = false;
        } else {
            let mat = input[i..].find("do()");
            let end = mat.or(Some(input[i..].len())).unwrap() + i;
            i = end;
            doo = true;
        }
        if i >= input.len() {
            break;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
