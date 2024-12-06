use std::collections::HashMap;

use advent_of_code::split_by_empt_line;

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let split = split_by_empt_line(input);
    let mut order_map = HashMap::<u32, Vec<u32>>::new();
    for val in &split[0] {
        let mut split2 = val.split('|');
        let val1 = split2.next().unwrap().parse::<u32>().unwrap();
        let val2 = split2.next().unwrap().parse::<u32>().unwrap();
        let entry = order_map.get_mut(&val1);
        match entry {
            Some(e) => e.push(val2),
            None => {
                order_map.insert(val1, vec![val2]);
            }
        };
    }

    let mut updates: Vec<Vec<u32>> = vec![];
    for val in &split[1] {
        updates.push(val.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
    }
    (order_map, updates)
}

fn check_invalid(update: &Vec<u32>, order_map: &HashMap<u32, Vec<u32>>) -> Option<(u32, u32)> {
    for (i, page) in update.iter().enumerate() {
        let e = order_map.get(page);
        if e.is_some() {
            for p2 in e.unwrap() {
                let pos = update[..i].iter().position(|&x| x == *p2);
                if pos.is_some() {
                    return Some((i as u32, pos.unwrap() as u32));
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (order_map, updates) = parse(input);
    let mut result = 0;
    for update in updates {
        if check_invalid(&update, &order_map).is_none() {
            result += update[update.len() / 2]
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order_map, updates) = parse(input);
    let mut result = 0;
    for update in updates {
        if check_invalid(&update, &order_map).is_some() {
            let mut up = update.clone();
            loop {
                let inval = check_invalid(&up, &order_map);
                if inval.is_none() {
                    break;
                }
                let (a, b) = inval.unwrap();
                up.swap(a as usize, b as usize);
            }
            result += up[up.len() / 2]
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
