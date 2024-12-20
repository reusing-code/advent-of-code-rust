use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    i64,
};

use advent_of_code::{Coord2D, Field, DIRECTIONS};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<i64> {
    part_one_internal(input, 100)
}

fn part_one_internal(input: &str, time_cutoff: i64) -> Option<i64> {
    let mut field = Field::<char>::new(input);
    let mut start = Default::default();
    let mut end = Default::default();
    field.data.iter().enumerate().for_each(|(i, c)| {
        if *c == 'S' {
            start = field.coord_from_index(i as i64);
        } else if *c == 'E' {
            end = field.coord_from_index(i as i64)
        }
    });
    *field.get_coord_mut(&start).unwrap() = '.';
    *field.get_coord_mut(&end).unwrap() = '.';
    let shortest = shortest_path(&field, &start, &end).unwrap();
    let mut result = 0;
    for y in 1..field.h - 1 {
        for x in 1..field.w - 1 {
            if field.get(x, y).unwrap() == '#' {
                if field.get(x - 1, y).unwrap() == '.' && field.get(x + 1, y).unwrap() == '.' {
                    let mut new_field = field.clone();
                    new_field.set_coord(
                        &Coord2D {
                            x: x as i64,
                            y: y as i64,
                        },
                        &'-',
                    );
                    let dist = shortest_path(&new_field, &start, &end);
                    if dist.is_some() && dist.unwrap() + time_cutoff <= shortest {
                        result += 1;
                    }
                }
                if field.get(x, y - 1).unwrap() == '.' && field.get(x, y + 1).unwrap() == '.' {
                    let mut new_field = field.clone();
                    new_field.set_coord(
                        &Coord2D {
                            x: x as i64,
                            y: y as i64,
                        },
                        &'|',
                    );
                    let dist = shortest_path(&new_field, &start, &end);
                    if dist.is_some() && dist.unwrap() + time_cutoff <= shortest {
                        result += 1;
                    }
                }
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    part_two_internal(input, 100)
}

fn part_two_internal(input: &str, time_cutoff: i64) -> Option<i64> {
    let mut field = Field::<char>::new(input);
    let mut start = Default::default();
    let mut end = Default::default();
    field.data.iter().enumerate().for_each(|(i, c)| {
        if *c == 'S' {
            start = field.coord_from_index(i as i64);
        } else if *c == 'E' {
            end = field.coord_from_index(i as i64)
        }
    });
    *field.get_coord_mut(&start).unwrap() = '.';
    *field.get_coord_mut(&end).unwrap() = '.';

    let shortest = shortest_path(&field, &start, &end).unwrap();

    let distances_end = distance_field(&field, &end);
    let distances_start = distance_field(&field, &start);
    let mut result = 0;
    for x in 1..field.w - 1 {
        for y in 1..field.h - 1 {
            let max_end = shortest - distances_start.get(x, y).unwrap() - time_cutoff;
            if max_end >= 0 {
                for dx in -20_i64..21_i64 {
                    for dy in -20_i64..21_i64 {
                        let manh = dx.abs() + dy.abs();
                        if manh <= 20 {
                            let d_end = distances_end.get_signed(x as i64 + dx, y as i64 + dy);
                            if d_end.is_some() && d_end.unwrap() <= max_end - manh {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(result)
}

fn distance_field(field: &Field<char>, start: &Coord2D) -> Field<i64> {
    let distances_end = shortest_distances(&field, &start);
    let mut distance_field = Field::<i64> {
        w: field.w,
        h: field.h,
        data: vec![],
    };
    distance_field.data.resize(field.w * field.h, i64::MAX);
    for (c, d) in distances_end {
        distance_field.set_coord(&c, &d);
    }
    distance_field
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    position: Coord2D,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(field: &Field<char>, pos: &Coord2D) -> Vec<(Coord2D, i64)> {
    let mut result = vec![];
    for (i, dir) in DIRECTIONS.iter().enumerate() {
        let next = pos.add(dir);
        let val = field.get_coord(&next);
        if val.is_some()
            && (val.unwrap() == '.'
                || (val.unwrap() == '-' && i <= 1)
                || (val.unwrap() == '|' && i >= 2))
        {
            result.push((
                Coord2D {
                    x: next.x,
                    y: next.y,
                },
                1,
            ));
        }
    }
    result
}

fn shortest_path(field: &Field<char>, start: &Coord2D, goal: &Coord2D) -> Option<i64> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::<Coord2D, i64>::new();
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        position: start.clone(),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == *goal {
            return Some(cost);
        }

        if cost > *(dist.entry(position.clone()).or_insert(i64::MAX)) {
            continue;
        }

        for (n, cos) in get_neighbors(field, &position) {
            let next = State {
                cost: cost + cos,
                position: n,
            };

            if next.cost < *dist.entry(next.position.clone()).or_insert(i64::MAX) {
                heap.push(next.clone());
                dist.insert(next.position, next.cost);
            }
        }
    }
    None
}
fn shortest_distances(field: &Field<char>, start: &Coord2D) -> HashMap<Coord2D, i64> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::<Coord2D, i64>::new();
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        position: start.clone(),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > *(dist.entry(position.clone()).or_insert(i64::MAX)) {
            continue;
        }

        for (n, cos) in get_neighbors(field, &position) {
            let next = State {
                cost: cost + cos,
                position: n,
            };

            if next.cost < *dist.entry(next.position.clone()).or_insert(i64::MAX) {
                heap.push(next.clone());
                dist.insert(next.position, next.cost);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_internal(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_internal(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(285));
    }
}
