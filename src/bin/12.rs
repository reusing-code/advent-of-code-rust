use advent_of_code::{Coord2D, Field, DIRECTIONS};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let field = Field::<char>::new(input);
    let mut visited = Field::<bool> {
        w: field.w,
        h: field.h,
        data: Vec::new(),
    };

    visited.data.resize(field.data.len(), false);
    let mut result = 0;
    for (i, c) in field.data.iter().enumerate() {
        let coord = field.coord_from_index(i as i64);
        if visited.data[i] {
            continue;
        }
        visited.data[i] = false;

        let mut size = 0;
        let mut fence = 0;
        let mut next_list: Vec<Coord2D> = vec![coord.clone()];
        while !next_list.is_empty() {
            let curr = next_list.pop().unwrap();
            if visited.get_coord(&curr).unwrap() {
                continue;
            }
            size += 1;
            *visited.get_coord_mut(&curr).unwrap() = true;
            for dir in DIRECTIONS {
                let next = curr.add(&dir);
                match field.get_coord(&next) {
                    None => {
                        fence += 1;
                    }
                    Some(v) => {
                        if v == *c {
                            next_list.push(next);
                        } else {
                            fence += 1;
                        }
                    }
                }
            }
        }
        result += size * fence;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let field = Field::<char>::new(input);
    let mut visited = Field::<bool> {
        w: field.w,
        h: field.h,
        data: Vec::new(),
    };

    visited.data.resize(field.data.len(), false);
    let mut result = 0;
    for (i, c) in field.data.iter().enumerate() {
        let coord = field.coord_from_index(i as i64);
        if visited.data[i] {
            continue;
        }
        visited.data[i] = false;

        let mut size = 0;
        let mut fences = vec![];
        let mut next_list: Vec<Coord2D> = vec![coord.clone()];
        while !next_list.is_empty() {
            let curr = next_list.pop().unwrap();
            if visited.get_coord(&curr).unwrap() {
                continue;
            }
            size += 1;
            *visited.get_coord_mut(&curr).unwrap() = true;
            for (j, dir) in DIRECTIONS.iter().enumerate() {
                let next = curr.add(&dir);
                match field.get_coord(&next) {
                    None => {
                        fences.push((next, j));
                    }
                    Some(v) => {
                        if v == *c {
                            next_list.push(next);
                        } else {
                            fences.push((next, j));
                        }
                    }
                }
            }
        }
        fences.sort();
        let sides = sides_from_fences(&mut fences);
        result += size * sides;
    }
    Some(result)
}

fn sides_from_fences(fences: &mut Vec<(Coord2D, usize)>) -> i64 {
    let mut sides = 0;

    while !fences.is_empty() {
        let mut curr = vec![fences.pop().unwrap()];
        let mut popped = true;
        let hor = curr[0].1;
        sides += 1;
        while popped {
            popped = false;
            let mut matc = vec![];
            for (i, f) in fences.iter().enumerate().filter(|(_i, (_x, d))| *d == hor) {
                let dir_vec = if hor == 0 || hor == 1 {
                    &DIRECTIONS[2..]
                } else {
                    &DIRECTIONS[..2]
                };
                for dir in dir_vec {
                    let next = f.0.add(dir);
                    for p in &curr {
                        if p.0 == next {
                            matc.push(i);
                        }
                    }
                }
                matc.sort();
            }
            matc.dedup();
            for m in matc.iter().rev() {
                popped = true;
                curr.push(fences[*m].clone());
                fences.swap_remove(*m);
            }
        }
    }
    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
}
