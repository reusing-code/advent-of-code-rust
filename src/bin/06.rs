use std::collections::HashSet;

use advent_of_code::{Coord2D, Field};

advent_of_code::solution!(6);

fn rotate(dir: &mut Coord2D) {
    if dir.y == -1 {
        dir.y = 0;
        dir.x = 1;
    } else if dir.x == 1 {
        dir.x = 0;
        dir.y = 1;
    } else if dir.y == 1 {
        dir.y = 0;
        dir.x = -1;
    } else if dir.x == -1 {
        dir.x = 0;
        dir.y = -1;
    }
}

fn parse_field(input: &str) -> (Field<char>, Coord2D) {
    let mut field = Field::<char>::new(input);
    let mut current = Coord2D { x: 0, y: 0 };
    for y in 0..field.h {
        for x in 0..field.w {
            if field.get(x, y).unwrap() == '^' {
                current.x = x as i64;
                current.y = y as i64;
                field.set_coord(&current, &'U');
            }
        }
    }
    (field, current)
}

fn dir_char(dir: &Coord2D) -> char {
    if dir.y == -1 {
        return 'U';
    } else if dir.x == 1 {
        return 'R';
    } else if dir.y == 1 {
        return 'D';
    } else if dir.x == -1 {
        return 'L';
    } else {
        panic!("unreachable code")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut field, mut current) = parse_field(input);
    let mut dir = Coord2D { x: 0, y: -1 };

    loop {
        let next = current.add(&dir);
        let next_val = field.get_coord(&next);
        match next_val {
            None => break,
            Some('#') => rotate(&mut dir),
            Some(_) => {
                current = next;
                field.set_coord(&current, &'U');
            }
        }
    }

    Some(field.data.iter().filter(|x| **x == 'U').count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_field, start) = parse_field(input);
    let mut path = HashSet::new();
    {
        let mut current = start.clone();
        let mut dir = Coord2D { x: 0, y: -1 };
        let mut field = start_field.clone();
        loop {
            let next = current.add(&dir);
            let next_val = field.get_coord(&next);
            match next_val {
                None => break,
                Some('#') => rotate(&mut dir),
                Some(_) => {
                    current = next;
                    path.insert(current.clone());
                    field.set_coord(&current, &dir_char(&dir));
                }
            }
        }
    }

    let mut result = 0;

    for step in path {
        let mut new_field = start_field.clone();
        let mut current = start.clone();
        let mut dir = Coord2D { x: 0, y: -1 };
        new_field.set_coord(&step, &'#');

        loop {
            let next = current.add(&dir);
            let next_val = new_field.get_coord(&next);
            match next_val {
                None => break,
                Some('#') => rotate(&mut dir),
                Some(x) => {
                    if x == dir_char(&dir) && step != start {
                        result += 1;
                        break;
                    }
                    current = next;
                    new_field.set_coord(&current, &dir_char(&dir));
                }
            }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
