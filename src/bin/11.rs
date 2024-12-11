use std::{collections::HashMap, iter::successors};

advent_of_code::solution!(11);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CacheKey {
    stone: i64,
    blinks_left: i64,
}

fn get_digits(n: i64) -> i64 {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count() as i64
}

fn blink_rec(stone: i64, blinks: i64, max_blinks: i64, cache: &mut HashMap<CacheKey, i64>) -> i64 {
    if blinks == max_blinks {
        return 1;
    }
    let key = CacheKey {
        stone,
        blinks_left: max_blinks - blinks,
    };
    let cache_result = cache.get(&key);
    if cache_result.is_some() {
        return *cache_result.unwrap();
    }
    let digits = get_digits(stone);
    let result;
    if stone == 0 {
        result = blink_rec(1, blinks + 1, max_blinks, cache);
    } else if digits % 2 == 0 {
        result = blink_rec(
            stone / (10_i64.pow((digits / 2) as u32)),
            blinks + 1,
            max_blinks,
            cache,
        ) + blink_rec(
            stone % (10_i64.pow((digits / 2) as u32)),
            blinks + 1,
            max_blinks,
            cache,
        );
    } else {
        result = blink_rec(stone * 2024, blinks + 1, max_blinks, cache);
    }
    cache.insert(key, result);
    result
}

fn blink(input: &str, blinks: i64) -> Option<i64> {
    let mut cache = HashMap::<CacheKey, i64>::new();
    let mut result = 0;
    for stone in input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
    {
        result += blink_rec(stone, 0, blinks, &mut cache)
    }
    Some(result)
}

pub fn part_one(input: &str) -> Option<i64> {
    blink(input, 25)
}

pub fn part_two(input: &str) -> Option<i64> {
    blink(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
