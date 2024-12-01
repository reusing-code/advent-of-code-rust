advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = input.lines();
    let mut v1: Vec<i64> = vec![];
    let mut v2: Vec<i64> = vec![];

    for line in lines {
        let mut values = line.split_whitespace();
        v1.push(values.next().unwrap().parse::<i64>().unwrap());
        v2.push(values.next().unwrap().parse::<i64>().unwrap());
    }

    v1.sort();
    v2.sort();

    return (v1, v2);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (v1, v2) = parse(input);

    let mut result: i64 = 0;

    for (a, b) in v1.iter().zip(v2.iter()) {
        result += i64::abs(a - b);
    }
    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (v1, v2) = parse(input);
    let mut result: i64 = 0;

    let mut it = v2.iter();
    let mut current = it.next();

    let mut last_res: i64 = 0;
    let mut last_val: i64 = -1;

    for v in v1 {
        if v != last_val {
            last_res = 0;
            while current.is_some() && *current.unwrap() <= v {
                if *current.unwrap() == v {
                    last_res += v
                }
                current = it.next();
            }
        }
        result += last_res;
        last_val = v;
    }

    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
