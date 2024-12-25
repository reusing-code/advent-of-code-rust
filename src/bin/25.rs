use advent_of_code::split_by_empt_line;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<i64> {
    let split = split_by_empt_line(input);
    let len = split[0][0].len();
    let height = split[0].len() - 2;
    let mut keys = vec![];
    let mut locks = vec![];

    for block in split {
        let mut result = vec![];
        result.resize(len, 0_usize);
        if block[0].chars().all(|c| c == '#') {
            for (h, line) in block.iter().enumerate() {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        result[i] = h;
                    }
                }
            }
            locks.push(result);
        } else {
            for (h, line) in block.iter().rev().enumerate() {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        result[i] = h;
                    }
                }
            }

            keys.push(result);
        }
    }
    let mut result = 0;
    for key in keys {
        'lock: for lock in &locks {
            for (i, kv) in key.iter().enumerate() {
                if kv + lock[i] > height {
                    continue 'lock;
                }
            }
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
