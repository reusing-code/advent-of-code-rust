use advent_of_code::Field;

advent_of_code::solution!(4);

fn check_stack(stack: &Vec<char>, needles: &Vec<&str>) -> u32 {
    //println!("{:?}", stack);
    let mut result: u32 = 0;

    for needle in needles {
        if stack.len() < needle.len() {
            continue;
        }
        let x = stack
            .into_iter()
            .rev()
            .zip(needle.chars())
            .map(|(a, b)| *a == b)
            .reduce(|acc, a| acc && a)
            .unwrap();
        if x {
            result += 1;
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = Field::<char>::new(input);
    let needles = vec!["XMAS", "SAMX"];

    let mut result: u32 = 0;

    // horizontal
    for i in 0..field.h {
        let mut stack: Vec<char> = vec![];
        for j in 0..field.w {
            stack.push(field.get(j, i)?);
            result += check_stack(&stack, &needles);
        }
    }

    // vertical
    for j in 0..field.w {
        let mut stack: Vec<char> = vec![];
        for i in 0..field.h {
            stack.push(field.get(j, i)?);
            result += check_stack(&stack, &needles);
        }
    }

    // diagonal
    for i in -(field.w as i32)..(field.h as i32 + field.w as i32) {
        // upwards
        let mut stack: Vec<char> = vec![];
        for j in 0..field.w {
            let y = i as i32 - j as i32;
            let x = j as i32;
            if y < 0 || x < 0 || y >= field.h as i32 || x >= field.w as i32 {
                continue;
            }
            stack.push(field.get_signed(x as i64, y as i64)?);
            result += check_stack(&stack, &needles);
        }
        stack.clear();

        //downwards
        for j in 0..field.w {
            let y = i as i32 + j as i32;
            let x = j as i32;
            if y < 0 || x < 0 || y >= field.h as i32 || x >= field.w as i32 {
                continue;
            }
            stack.push(field.get_signed(x as i64, y as i64)?);
            result += check_stack(&stack, &needles);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = Field::<char>::new(input);
    let mut result: u32 = 0;

    for y in 1..field.h - 1 {
        for x in 1..field.w - 1 {
            if field.get(x, y)? != 'A' {
                continue;
            }
            let tl = field.get(x - 1, y - 1)?;
            let tr = field.get(x + 1, y - 1)?;
            let bl = field.get(x - 1, y + 1)?;
            let br = field.get(x + 1, y + 1)?;
            if (tl == 'M' || tl == 'S')
                && (tr == 'M' || tr == 'S')
                && (bl == 'M' || bl == 'S')
                && (br == 'M' || br == 'S')
                && (tl != br)
                && (tr != bl)
            {
                result += 1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
