use std::collections::HashSet;

use advent_of_code::{split_by_empt_line, Coord2D, Field, DIRECTIONS};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<i64> {
    let split_empyline = split_by_empt_line(input);
    let mut field = Field::<char>::parse_vec(&split_empyline[0]);
    print_field(&field);
    let mut current =
        field.coord_from_index(field.data.iter().position(|x| *x == '@').unwrap() as i64);

    for move_line in &split_empyline[1] {
        for mov in move_line.chars() {
            do_move(&mut field, &mut current, mov);
        }
    }

    let mut result = 0;
    for (i, c) in field.data.iter().enumerate() {
        if *c == 'O' {
            let coord = field.coord_from_index(i as i64);
            result += coord.x + 100 * coord.y;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let split_empyline = split_by_empt_line(input);
    let new_input: Vec<String> = split_empyline[0].iter().map(|x| double_up(x)).collect();
    let mut field = Field::<char>::parse_vec(&new_input);
    print_field(&field);
    let mut current =
        field.coord_from_index(field.data.iter().position(|x| *x == '@').unwrap() as i64);

    for move_line in &split_empyline[1] {
        for mov in move_line.chars() {
            do_move2(&mut field, &mut current, mov);
        }
    }

    let mut result = 0;
    for (i, c) in field.data.iter().enumerate() {
        if *c == '[' {
            let coord = field.coord_from_index(i as i64);
            result += coord.x + 100 * coord.y;
        }
    }
    Some(result)
}

fn char_to_direction(c: char) -> &'static Coord2D {
    match c {
        '>' => &DIRECTIONS[0],
        '<' => &DIRECTIONS[1],
        'v' => &DIRECTIONS[2],
        '^' => &DIRECTIONS[3],
        _ => &DIRECTIONS[0],
    }
}

fn do_move2(field: &mut Field<char>, current: &mut Coord2D, c: char) {
    let dir = char_to_direction(c);
    let mut move_stack = vec![];
    let horz = c == '<' || c == '>';
    if horz {
        let mut next = current.add(dir);
        let mut next_next = next.add(dir);
        while (field.get_coord(&next).unwrap() == '['
            && field.get_coord(&next_next).unwrap() == ']')
            || (field.get_coord(&next).unwrap() == ']'
                && field.get_coord(&next_next).unwrap() == '[')
        {
            move_stack.push(next.clone());
            move_stack.push(next_next.clone());
            next = next_next.add(dir);
            next_next = next.add(dir);
        }
        if field.get_coord(&next).unwrap() == '#' {
            return;
        }
    } else {
        let mut next = vec![current.add(dir)];
        loop {
            let mut next_next = vec![];
            let mut new_moves = HashSet::new();
            if next.iter().any(|x| field.get_coord(x).unwrap() == '#') {
                return;
            }
            if next.iter().all(|x| field.get_coord(x).unwrap() == '.') {
                break;
            }
            let _ = next
                .iter()
                .map(|x| {
                    let val = field.get_coord(x).unwrap();
                    if val == '[' {
                        new_moves.insert(x.clone());
                        new_moves.insert(x.add(&DIRECTIONS[0]));

                        next_next.push(x.add(dir));
                        next_next.push(x.add(&DIRECTIONS[0]).add(dir));
                    } else if val == ']' {
                        new_moves.insert(x.clone());
                        new_moves.insert(x.add(&DIRECTIONS[1]));

                        next_next.push(x.add(dir));
                        next_next.push(x.add(&DIRECTIONS[1]).add(dir));
                    }
                })
                .collect::<Vec<_>>();
            next_next.sort();
            next_next.dedup();
            next = next_next;
            move_stack.extend(new_moves);
        }
    }
    let _ = move_stack
        .iter()
        .rev()
        .map(|x| {
            *field.get_coord_mut(&x.add(dir)).unwrap() = field.get_coord(x).unwrap();
            *field.get_coord_mut(x).unwrap() = '.';
        })
        .collect::<Vec<_>>();
    *field.get_coord_mut(current).unwrap() = '.';
    *current = current.add(dir);
    *field.get_coord_mut(current).unwrap() = '@';
}

fn do_move(field: &mut Field<char>, current: &mut Coord2D, c: char) {
    let dir = char_to_direction(c);
    let mut move_stack = vec![];
    let mut next = current.add(dir);
    while field.get_coord(&next).unwrap() == 'O' {
        move_stack.push(next.clone());
        next = next.add(dir);
    }
    if field.get_coord(&next).unwrap() == '#' {
        return;
    }

    let _ = move_stack
        .iter()
        .rev()
        .map(|x| {
            *field.get_coord_mut(&x.add(dir)).unwrap() = 'O';
        })
        .collect::<Vec<_>>();
    *field.get_coord_mut(current).unwrap() = '.';
    *current = current.add(dir);
    *field.get_coord_mut(current).unwrap() = '@';
}

fn print_field(field: &Field<char>) {
    for y in 0..field.h {
        println!("");
        for x in 0..field.w {
            print!("{}", field.get(x, y).unwrap());
        }
    }
    println!("");
}

fn double_up(input: &str) -> String {
    let mut result = String::from("");
    for c in input.chars() {
        result.push_str(match c {
            '.' => "..",
            '#' => "##",
            'O' => "[]",
            '@' => "@.",
            _ => "",
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(618));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
