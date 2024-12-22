use std::collections::HashMap;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|s| {
                let mut secret = s.parse::<i64>().unwrap();
                for _i in 0..2000 {
                    secret = (secret ^ (secret * 64)) % 16777216;
                    secret = (secret ^ (secret / 32)) % 16777216;
                    secret = (secret ^ (secret * 2048)) % 16777216;
                }
                secret
            })
            .sum(),
    )
}

type MaxMap = HashMap<(i8, i8, i8, i8), i64>;

pub fn part_two(input: &str) -> Option<i64> {
    let mut max_map = MaxMap::new();

    for initial in input.lines().map(|s| s.parse::<i64>().unwrap()) {
        let mut map = MaxMap::new();
        let mut last = [0_i8; 4];
        let mut last_price = 0_i8;
        let mut secret = initial;
        for i in 0..2000 {
            secret = (secret ^ (secret * 64)) % 16777216;
            secret = (secret ^ (secret / 32)) % 16777216;
            secret = (secret ^ (secret * 2048)) % 16777216;

            let price = (secret % 10) as i8;

            last[i % 4] = price - last_price;
            if i >= 5 {
                map.entry((
                    last[(i - 3) % 4],
                    last[(i - 2) % 4],
                    last[(i - 1) % 4],
                    last[(i - 0) % 4],
                ))
                .or_insert(price as i64);
            }

            last_price = price;
        }

        for (k, v) in map {
            max_map.entry(k).and_modify(|x| *x = *x + v).or_insert(v);
        }
    }
    Some(*max_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
