use std::collections::{HashMap, HashSet};

use advent_of_code::Coord2D;

advent_of_code::solution!(8);

fn parse(input: &str) -> (i32, i32, HashMap<char, Vec<Coord2D>>) {
    let mut antennas: HashMap<char, Vec<Coord2D>> = HashMap::new();
    let mut h = 0;
    let mut w: i32 = 0;
    for (y, line) in input.lines().enumerate() {
        h += 1;
        w = line.chars().count() as i32;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| {
                        v.push(Coord2D {
                            x: x as i32,
                            y: y as i32,
                        })
                    })
                    .or_insert(vec![Coord2D {
                        x: x as i32,
                        y: y as i32,
                    }]);
            }
        }
    }

    (h, w, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (h, w, antennas) = parse(input);
    let mut antinodes: HashSet<Coord2D> = HashSet::new();
    for (_k, v) in antennas {
        if v.len() < 2 {
            continue;
        }
        for (i, ant1) in v.iter().enumerate() {
            for ant2 in v.iter().skip(i + 1) {
                {
                    let anti = calc_antinode(ant1, ant2);
                    if anti.x >= 0 && anti.x < w as i32 && anti.y >= 0 && anti.y < h {
                        antinodes.insert(anti);
                    }
                }
                let anti = calc_antinode(ant2, ant1);
                if anti.x >= 0 && anti.x < w as i32 && anti.y >= 0 && anti.y < h {
                    antinodes.insert(anti);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (h, w, antennas) = parse(input);
    let mut antinodes: HashSet<Coord2D> = HashSet::new();
    for (_k, v) in antennas {
        if v.len() < 2 {
            continue;
        }
        for (i, ant1) in v.iter().enumerate() {
            for ant2 in v.iter().skip(i + 1) {
                let antis = calc_antinode2(ant2, ant1, h, w);
                for anti in antis {
                    antinodes.insert(anti);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

fn calc_antinode(a: &Coord2D, b: &Coord2D) -> Coord2D {
    Coord2D {
        x: a.x - (b.x - a.x),
        y: a.y - (b.y - a.y),
    }
}
fn calc_antinode2(a: &Coord2D, b: &Coord2D, h: i32, w: i32) -> Vec<Coord2D> {
    let diff = Coord2D {
        x: b.x - a.x,
        y: b.y - a.y,
    };
    let mut current = a.clone();
    let mut result = vec![];

    while current.x >= 0 && current.x < w && current.y >= 0 && current.y < h {
        result.push(current.clone());
        current = current.add(&diff);
    }

    current = a.clone();
    let diff2 = Coord2D {
        x: a.x - b.x,
        y: a.y - b.y,
    };
    while current.x >= 0 && current.x < w && current.y >= 0 && current.y < h {
        result.push(current.clone());
        current = current.add(&diff2);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
