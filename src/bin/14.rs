use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use advent_of_code::Coord2D;
use tempfile::{Builder, TempDir};

advent_of_code::solution!(14);
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Robot {
    start: Coord2D,
    v: Coord2D,
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        robots.push(Robot {
            start: Coord2D {
                x: caps[1].parse::<i64>().unwrap(),
                y: caps[2].parse::<i64>().unwrap(),
            },
            v: Coord2D {
                x: caps[3].parse::<i64>().unwrap(),
                y: caps[4].parse::<i64>().unwrap(),
            },
        });
    }
    robots
}

pub fn part_one(input: &str) -> Option<i64> {
    part_one_internal(input, Coord2D { x: 101, y: 103 })
}

fn part_one_internal(input: &str, room_size: Coord2D) -> Option<i64> {
    let robots = parse_robots(input);
    let mut quadrant_count = vec![0, 0, 0, 0];
    for r in robots {
        let dest = Coord2D {
            x: (r.start.x + 100 * r.v.x).rem_euclid(room_size.x),
            y: (r.start.y + 100 * r.v.y).rem_euclid(room_size.y),
        };
        if dest.x == room_size.x / 2 || dest.y == room_size.y / 2 {
            continue;
        }
        let mut quad = if dest.x < room_size.x / 2 { 0 } else { 2 };
        quad += if dest.y < room_size.y / 2 { 0 } else { 1 };
        quadrant_count[quad] += 1;
    }
    Some(quadrant_count.iter().product())
}

pub fn part_two(input: &str) -> Option<i64> {
    part_two_internal(input, Coord2D { x: 101, y: 103 })
}

fn part_two_internal(input: &str, room_size: Coord2D) -> Option<i64> {
    let robots = parse_robots(input);
    let mut pos: Vec<_> = robots.iter().map(|r| r.start.clone()).collect();
    for (i, r) in robots.iter().enumerate() {
        pos[i].x = r.start.x;
        pos[i].y = r.start.y;
    }
    let tmp_dir = Builder::new().keep(true).prefix("aoc-").tempdir().unwrap();
    let mut hashes = HashSet::new();
    for step in 1..10000001 {
        for (i, r) in robots.iter().enumerate() {
            pos[i].x = (pos[i].x + r.v.x).rem_euclid(room_size.x);
            pos[i].y = (pos[i].y + r.v.y).rem_euclid(room_size.y);
        }
        /*let mut valid = true;
        for (i, p) in pos.iter().enumerate() {
            if !pos
                .iter()
                .enumerate()
                .any(|(i2, p2)| i != i2 && (p2.x - p.x).abs() <= 1 && (p2.y - p.y).abs() <= 1)
            {
                valid = false;
                break;
            }
        }*/
        let mut hasher = DefaultHasher::new();
        pos.hash(&mut hasher);
        let h = hasher.finish();
        if hashes.contains(&h) {
            break;
        }
        hashes.insert(h);
        if (step - 39) % 101 == 0 || (step - 99) % 103 == 0 {
            draw_image(&pos, step, &tmp_dir, &room_size);
        }
    }

    None
}

fn draw_image(pos: &Vec<Coord2D>, step: i64, tmp_dir: &TempDir, room_size: &Coord2D) {
    let mut imgbuf = image::ImageBuffer::new(room_size.x as u32, room_size.y as u32);
    for p in pos {
        *imgbuf.get_pixel_mut(p.x as u32, p.y as u32) = image::Rgb([0 as u8, 255 as u8, 0 as u8]);
    }
    let binding = tmp_dir.path().join(format!("{:05}.png", step));
    let path = binding.to_str().unwrap();
    imgbuf.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_internal(
            &advent_of_code::template::read_file("examples", DAY),
            Coord2D { x: 11, y: 7 },
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_internal(
            &advent_of_code::template::read_file("examples", DAY),
            Coord2D { x: 11, y: 7 },
        );
        assert_eq!(result, None);
    }
}
