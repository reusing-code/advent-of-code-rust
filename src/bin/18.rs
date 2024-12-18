use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

use advent_of_code::{Coord2D, Field, DIRECTIONS};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    part_one_internal(input, 71, 1024)
}

fn part_one_internal(input: &str, size: usize, num_bytes: usize) -> Option<i64> {
    let mut field = Field::<bool> {
        w: size,
        h: size,
        data: Vec::new(),
    };
    field.data.resize(size * size, false);

    input.lines().take(num_bytes).for_each(|s| {
        let mut split = s.split(",");
        *field
            .get_mut(
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
            .unwrap() = true;
    });

    field.print();

    Some(shortest_path(
        &field,
        &Coord2D { x: 0, y: 0 },
        &Coord2D {
            x: (size - 1) as i64,
            y: (size - 1) as i64,
        },
    ))?
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_internal(input, 71)
}

fn part_two_internal(input: &str, size: usize) -> Option<String> {
    let mut field = Field::<bool> {
        w: size,
        h: size,
        data: Vec::new(),
    };
    field.data.resize(size * size, false);

    for line in input.lines() {
        let mut split = line.split(",");
        let n = Coord2D {
            x: split.next().unwrap().parse::<i64>().unwrap(),
            y: split.next().unwrap().parse::<i64>().unwrap(),
        };
        *field.get_coord_mut(&n).unwrap() = true;

        let res = shortest_path(
            &field,
            &Coord2D { x: 0, y: 0 },
            &Coord2D {
                x: (size - 1) as i64,
                y: (size - 1) as i64,
            },
        );

        if res.is_none() {
            return Some(format!("{},{}", n.x, n.y));
        }
    }

    None
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

fn get_neighbors(field: &Field<bool>, pos: &Coord2D) -> Vec<(Coord2D, i64)> {
    let mut result = vec![];
    for dir in DIRECTIONS {
        let next = pos.add(dir);
        let val = field.get_coord(&next);
        if val.is_some() && !val.unwrap() {
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

fn shortest_path(field: &Field<bool>, start: &Coord2D, goal: &Coord2D) -> Option<i64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_internal(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_internal(&advent_of_code::template::read_file("examples", DAY), 7);
        assert_eq!(result, Some(String::from("6,1")));
    }
}
