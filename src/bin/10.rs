use std::collections::HashSet;

use advent_of_code::{Coord2D, Field};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i64> {
    let field = Field::<i32>::new(input);
    let mut reachable_nines: Field<HashSet<Coord2D>> = Field {
        data: vec![],
        w: field.w,
        h: field.h,
    };
    reachable_nines
        .data
        .resize(field.w * field.h, HashSet::new());

    let mut current = HashSet::new();

    field
        .data
        .iter()
        .enumerate()
        .filter(|(_i, v)| **v == 9)
        .for_each(|(i, _v)| {
            let c = Coord2D {
                x: (i % field.w) as i64,
                y: (i / field.w) as i64,
            };
            reachable_nines.data[i].insert(c.clone());
            current.insert(c);
        });

    for height in (0..9).rev() {
        let mut new_current = HashSet::new();
        for c in current.iter() {
            for dir in DIRECTIONS {
                let new_c = c.add(dir);
                let val = field.get_coord(&new_c);
                if val.is_some() && val.unwrap() == height {
                    new_current.insert(new_c.clone());
                    for rn in reachable_nines.get_coord(&c).unwrap().iter() {
                        reachable_nines
                            .get_coord_mut(&new_c)
                            .unwrap()
                            .insert(rn.clone());
                    }
                }
            }
        }
        current = new_current;
    }
    let mut result = 0;
    for c in current.iter() {
        result += reachable_nines.get_coord(c).unwrap().len() as i64;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let field = Field::<i32>::new(input);
    let mut reachable_nines: Field<i64> = Field {
        data: vec![],
        w: field.w,
        h: field.h,
    };
    reachable_nines.data.resize(field.w * field.h, 0);

    let mut current = HashSet::new();
    let mut zeros = HashSet::new();

    field
        .data
        .iter()
        .enumerate()
        .filter(|(_i, v)| **v == 9 || **v == 0)
        .for_each(|(i, v)| {
            let c = Coord2D {
                x: (i % field.w) as i64,
                y: (i / field.w) as i64,
            };
            if *v == 9 {
                reachable_nines.data[i] = 1;
                current.insert(c);
            } else {
                zeros.insert(c);
            }
        });

    for height in (0..9).rev() {
        let mut new_current = HashSet::new();
        for c in current.iter() {
            for dir in DIRECTIONS {
                let new_c = c.add(dir);
                let val = field.get_coord(&new_c);
                if val.is_some() && val.unwrap() == height {
                    new_current.insert(new_c.clone());
                    let paths = reachable_nines.get_coord(&c).unwrap();
                    *reachable_nines.get_coord_mut(&new_c).unwrap() += paths;
                }
            }
        }
        current = new_current;
    }
    let mut result = 0;
    for c in zeros.iter() {
        result += reachable_nines.get_coord(c).unwrap();
    }
    Some(result)
}

const DIRECTIONS: &'static [Coord2D] = &[
    Coord2D { x: 1, y: 0 },
    Coord2D { x: -1, y: 0 },
    Coord2D { x: 0, y: 1 },
    Coord2D { x: 0, y: -1 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
