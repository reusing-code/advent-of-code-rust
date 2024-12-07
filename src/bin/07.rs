advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;

    for line in input.lines() {
        let split1: Vec<&str> = line.split(':').collect();
        let target = split1[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = split1[1]
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if check_rec(numbers[0], 1, target, &numbers) {
            result += target;
        }
    }
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    println!("{}", concat(17, 8));
    let mut result = 0;

    for line in input.lines() {
        let split1: Vec<&str> = line.split(':').collect();
        let target = split1[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = split1[1]
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if check_rec2(numbers[0], 1, target, &numbers) {
            result += target;
        }
    }
    Some(result as u64)
}

fn check_rec(val: i64, idx: usize, target: i64, numbers: &Vec<i64>) -> bool {
    if idx == numbers.len() {
        return val == target;
    }
    if val > target {
        return false;
    }
    check_rec(val + numbers[idx], idx + 1, target, numbers)
        || check_rec(val * numbers[idx], idx + 1, target, numbers)
}

fn check_rec2(val: i64, idx: usize, target: i64, numbers: &Vec<i64>) -> bool {
    if idx == numbers.len() {
        return val == target;
    }
    if val > target {
        return false;
    }
    check_rec2(val + numbers[idx], idx + 1, target, numbers)
        || check_rec2(val * numbers[idx], idx + 1, target, numbers)
        || check_rec2(concat(val, numbers[idx]), idx + 1, target, numbers)
}

fn concat(a: i64, b: i64) -> i64 {
    let mut b2 = b;
    let mut a2 = a;
    while b2 != 0 {
        b2 = b2 / 10;
        a2 = a2 * 10;
    }
    return a2 + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
