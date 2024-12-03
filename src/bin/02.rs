advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for report in input.lines() {
        let mut vals: Vec<u32> = vec![];
        for num in report.split_whitespace() {
            vals.push(num.parse::<u32>().unwrap());
        }
        let asc: bool = vals[0] < vals[1];

        let mut safe = true;
        for i in 1..vals.len() {
            if (vals[i - 1] > vals[i]) == asc {
                safe = false;
                break;
            }
            if vals[i - 1] == vals[i] {
                safe = false;
                break;
            }
            if vals[i - 1].abs_diff(vals[i]) > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            result += 1;
        }
    }
    Some(result)
}

fn check(mut vals: Vec<u32>, allow_error: bool) -> bool {
    let asc: bool = vals[0] < vals[1];

    let mut safe = true;
    let mut one_err = !allow_error;
    for i in 1..vals.len() {
        let mut fail = false;
        if (vals[i - 1] > vals[i]) == asc {
            fail = true;
        }
        if vals[i - 1] == vals[i] {
            fail = true;
        }
        if vals[i - 1].abs_diff(vals[i]) > 3 {
            fail = true;
        }
        if one_err && fail {
            safe = false;
            break;
        }
        if fail {
            vals[i] = vals[i - 1];
            one_err = true;
        }
    }
    safe
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for report in input.lines() {
        let mut vals: Vec<u32> = vec![];
        for num in report.split_whitespace() {
            vals.push(num.parse::<u32>().unwrap());
        }
        let safe = check(vals.clone(), true);
        let mut vals2 = vals.clone();
        vals2.remove(0);
        let safe2 = check(vals2, false);
        vals.remove(1);
        let safe3 = check(vals, false);
        if safe || safe2 || safe3 {
            result += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
