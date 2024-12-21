use std::collections::HashMap;

use advent_of_code::Coord2D;
use itertools::Itertools;

advent_of_code::solution!(21);

struct Cache {
    max_depth: usize,
    options: HashMap<(Coord2D, Coord2D), Vec<Vec<char>>>,
    rec: HashMap<(usize, Vec<char>), i64>,
}

fn calc(input: &str, max_depth: usize) -> Option<i64> {
    let mut cache = Cache {
        max_depth,
        options: HashMap::new(),
        rec: HashMap::new(),
    };
    Some(
        input
            .lines()
            .map(|s| {
                let seq = s.chars().collect::<Vec<_>>();
                let num = s[..3].parse::<i64>().unwrap();
                let complexity = calc_complexity(&seq, &mut cache);
                num * complexity
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<i64> {
    calc(input, 2)
}

pub fn part_two(input: &str) -> Option<i64> {
    calc(input, 25)
}

fn calc_complexity(seq: &Vec<char>, cache: &mut Cache) -> i64 {
    let mut res = 0;
    let mut last = Coord2D { x: 0, y: 0 };
    for c in seq {
        let next = get_coord(*c);
        let diff = next.sub(&last);
        let curr_res = calc_seq_len_single(&last, &diff, cache);
        res += curr_res;

        last = next;
    }
    //println!("seq: {:?} len: {}", seq, res);
    res
}

fn get_coord(c: char) -> Coord2D {
    match c {
        '0' => Coord2D { x: 1, y: 0 },
        '1' => Coord2D { x: 2, y: 1 },
        '2' => Coord2D { x: 1, y: 1 },
        '3' => Coord2D { x: 0, y: 1 },
        '4' => Coord2D { x: 2, y: 2 },
        '5' => Coord2D { x: 1, y: 2 },
        '6' => Coord2D { x: 0, y: 2 },
        '7' => Coord2D { x: 2, y: 3 },
        '8' => Coord2D { x: 1, y: 3 },
        '9' => Coord2D { x: 0, y: 3 },
        '<' => Coord2D { x: 2, y: 1 },
        '>' => Coord2D { x: 0, y: 1 },
        '^' => Coord2D { x: 1, y: 0 },
        'v' => Coord2D { x: 1, y: 1 },

        'A' => Coord2D { x: 0, y: 0 },
        _ => Coord2D { x: 0, y: 0 },
    }
}

fn calc_seq_len_single(current: &Coord2D, diff: &Coord2D, cache: &mut Cache) -> i64 {
    let options = get_options(current, diff, cache);

    let mut min = i64::MAX;
    for opt in options {
        let val = calc_small_rec(0, &opt, cache);

        if val < min {
            min = val;
        }
    }

    min
}

fn get_options(current: &Coord2D, diff: &Coord2D, cache: &mut Cache) -> Vec<Vec<char>> {
    let cache_key = &(current.clone(), diff.clone());
    let entry = cache.options.get(cache_key);
    if entry.is_some() {
        return entry.unwrap().clone();
    }
    let mut moves = vec![];

    moves.resize(
        diff.x.abs() as usize,
        if diff.x < 0 {
            Coord2D { x: -1, y: 0 }
        } else {
            Coord2D { x: 1, y: 0 }
        },
    );
    moves.resize(
        (diff.x.abs() + diff.y.abs()) as usize,
        if diff.y < 0 {
            Coord2D { x: 0, y: -1 }
        } else {
            Coord2D { x: 0, y: 1 }
        },
    );

    let result = moves
        .iter()
        .permutations(moves.len())
        .unique()
        .filter(|x| {
            let mut c = current.clone();
            for step in x {
                c = c.add(step);
                if c.x == 2 && c.y == 0 {
                    return false;
                }
            }
            return true;
        })
        .map(|x| {
            x.iter()
                .map(|f| match **f {
                    Coord2D { x: 1, y: 0 } => '<',
                    Coord2D { x: -1, y: 0 } => '>',
                    Coord2D { x: 0, y: 1 } => '^',
                    Coord2D { x: 0, y: -1 } => 'v',
                    _ => ' ',
                })
                .collect::<Vec<_>>()
        })
        .map(|mut x| {
            x.push('A');
            x
        })
        .collect::<Vec<_>>();
    cache.options.insert(cache_key.clone(), result.clone());
    result
}

fn calc_small_rec(depth: usize, seq: &Vec<char>, cache: &mut Cache) -> i64 {
    if depth == cache.max_depth {
        return seq.len() as i64;
    }
    let cache_key = &(depth, seq.clone());
    let entry = cache.rec.get(cache_key);
    if entry.is_some() {
        return *entry.unwrap();
    }
    let mut res = 0;
    let mut last = Coord2D { x: 0, y: 0 };
    for c in seq {
        let next = get_coord(*c);
        let mut diff = next.sub(&last);
        diff.y = -diff.y;
        res += calc_small_rec_single(depth, &last, &diff, cache);

        last = next;
    }
    cache.rec.insert(cache_key.clone(), res);
    //println!("d: {depth}, seq {:?}, {res}", seq);
    res
}

fn calc_small_rec_single(
    depth: usize,
    current: &Coord2D,
    diff: &Coord2D,
    cache: &mut Cache,
) -> i64 {
    let options = get_options(current, diff, cache);

    let mut min = i64::MAX;
    for opt in options {
        let val = calc_small_rec(depth + 1, &opt, cache);

        if val < min {
            min = val;
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(68 * 29));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
