advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<i64> {
    let field = Field::<char>::new(input);
    let mut start = Default::default();
    let mut end = Default::default();
    field.data.iter().enumerate().for_each(|(i, c)| {
        if *c == 'S' {
            start = Node {
                c: field.coord_from_index(i as i64),
                dir: 0,
            };
        } else if *c == 'E' {
            end = field.coord_from_index(i as i64)
        }
    });

    let result = shortest_path(&field, &start, &end);

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut field = Field::<char>::new(input);
    let mut start = Default::default();
    let mut end = Default::default();
    field.data.iter().enumerate().for_each(|(i, c)| {
        if *c == 'S' {
            start = Node {
                c: field.coord_from_index(i as i64),
                dir: 0,
            };
        } else if *c == 'E' {
            end = field.coord_from_index(i as i64)
        }
    });

    let result = shortest_path_all(&mut field, &start, &end);

    Some(result)
}

use std::collections::{BinaryHeap, HashSet};
use std::i64;
use std::{cmp::Ordering, collections::HashMap};

use advent_of_code::{Coord2D, Field, DIRECTIONS};

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Default, Hash)]
struct Node {
    c: Coord2D,
    dir: i64,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    position: Node,
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

fn next_dir_clock(input: i64) -> i64 {
    match input {
        0 => 2,
        1 => 3,
        2 => 1,
        3 => 0,
        _ => 0,
    }
}
fn next_dir_anti_clock(input: i64) -> i64 {
    match input {
        0 => 3,
        1 => 2,
        2 => 0,
        3 => 1,
        _ => 0,
    }
}

fn get_neighbors(field: &Field<char>, node: &Node) -> Vec<(Node, i64)> {
    let mut result = vec![
        (
            Node {
                c: node.c.clone(),
                dir: next_dir_clock(node.dir),
            },
            1000,
        ),
        (
            Node {
                c: node.c.clone(),
                dir: next_dir_anti_clock(node.dir),
            },
            1000,
        ),
    ];

    let next = node.c.add(&DIRECTIONS[node.dir as usize]);
    if field.get_coord(&next).unwrap() != '#' {
        result.push((
            Node {
                c: next,
                dir: node.dir,
            },
            1,
        ));
    }

    result
}

fn shortest_path(field: &Field<char>, start: &Node, goal: &Coord2D) -> i64 {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::<Node, i64>::new();
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        position: start.clone(),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position.c == *goal {
            return cost;
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

    return i64::MAX;
}
fn shortest_path_all(field: &mut Field<char>, start: &Node, goal: &Coord2D) -> i64 {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::<Node, i64>::new();
    let mut from = HashMap::<Node, Vec<Node>>::new();
    dist.insert(start.clone(), 0);
    from.insert(start.clone(), vec![]);
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

            let old_cost = *dist.entry(next.position.clone()).or_insert(i64::MAX);
            if next.cost == old_cost {
                let e = from.entry(next.position).or_default();
                e.push(position.clone());
            } else if next.cost < old_cost {
                heap.push(next.clone());
                dist.insert(next.position.clone(), next.cost);
                from.insert(next.position, vec![position.clone()]);
            }
        }
    }

    let mut next_list = vec![];
    let mut goal_dir = 0;
    let mut goal_distance = i64::max_value();
    for i in 0..4 {
        let g = dist.get(&Node {
            c: goal.clone(),
            dir: i,
        });
        if g.is_some() {
            if *g.unwrap() < goal_distance {
                goal_distance = *g.unwrap();
                goal_dir = i;
            }
        }
    }
    let g = from.get(&Node {
        c: goal.clone(),
        dir: goal_dir,
    });
    next_list.extend(g.unwrap());

    let mut places = HashSet::new();
    places.insert(goal.clone());
    while !next_list.is_empty() {
        let elem = next_list.pop().unwrap();
        places.insert(elem.c.clone());
        next_list.extend(from.get(elem).unwrap());
    }

    for val in &places {
        *field.get_coord_mut(&val).unwrap() = 'O';
    }

    //field.print();
    return places.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
