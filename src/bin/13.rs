use advent_of_code::{split_by_empt_line, Coord2D};
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    button_a: Coord2D,
    button_b: Coord2D,
    target: Coord2D,
    price_a: i64,
    price_b: i64,
}

fn parse_machine(input: &Vec<String>, price_a: i64, price_b: i64) -> Machine {
    let mut result = Machine {
        button_a: Coord2D { x: 0, y: 0 },
        button_b: Coord2D { x: 0, y: 0 },
        target: Coord2D { x: 0, y: 0 },
        price_a,
        price_b,
    };
    let button_regex = Regex::new(r"Button .+: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let target_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let caps = button_regex.captures(&input[0]).unwrap();
    result.button_a.x = caps[1].parse::<i64>().unwrap();
    result.button_a.y = caps[2].parse::<i64>().unwrap();
    let caps = button_regex.captures(&input[1]).unwrap();
    result.button_b.x = caps[1].parse::<i64>().unwrap();
    result.button_b.y = caps[2].parse::<i64>().unwrap();
    let caps = target_regex.captures(&input[2]).unwrap();
    result.target.x = caps[1].parse::<i64>().unwrap();
    result.target.y = caps[2].parse::<i64>().unwrap();

    result
}

pub fn part_one(input: &str) -> Option<i64> {
    let price_a = 3;
    let price_b = 1;
    let machines: Vec<_> = split_by_empt_line(input)
        .iter()
        .map(|x| parse_machine(x, price_a, price_b))
        .collect();
    let mut result = 0;
    for m in machines {
        let b1 = (m.target.x * m.button_a.y - m.target.y * m.button_a.x)
            / (m.button_b.x * m.button_a.y - m.button_b.y * m.button_a.x);
        let a1 = (m.target.x - b1 * m.button_b.x) / m.button_a.x;
        if a1 <= 100 && a1 >= 0 && b1 <= 100 && b1 >= 0 {
            if (a1 * m.button_a.x + b1 * m.button_b.x == m.target.x)
                && (a1 * m.button_a.y + b1 * m.button_b.y == m.target.y)
            {
                //result += a1 * m.price_a + b1 * m.price_b;
            }
        }
        for a in 0..101 {
            let b = (m.target.x - a * m.button_a.x) / m.button_b.x;
            if a * m.button_a.y + b * m.button_b.y == m.target.y {
                if b > 0 && b <= 100 {
                    result += a * m.price_a + b * m.price_b;
                    break;
                }
            }
        }
    }
    Some(result as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let price_a = 3;
    let price_b = 1;
    let mut machines: Vec<_> = split_by_empt_line(input)
        .iter()
        .map(|x| parse_machine(x, price_a, price_b))
        .collect();
    let mut result = 0;
    for m in &mut machines {
        m.target.x += 10000000000000;
        m.target.y += 10000000000000;
        let b1 = (m.target.x * m.button_a.y - m.target.y * m.button_a.x)
            / (m.button_b.x * m.button_a.y - m.button_b.y * m.button_a.x);
        let a1 = (m.target.x - b1 * m.button_b.x) / m.button_a.x;
        if a1 >= 0 && b1 >= 0 {
            if (a1 * m.button_a.x + b1 * m.button_b.x == m.target.x)
                && (a1 * m.button_a.y + b1 * m.button_b.y == m.target.y)
            {
                result += a1 * m.price_a + b1 * m.price_b;
            }
        }
    }
    Some(result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
