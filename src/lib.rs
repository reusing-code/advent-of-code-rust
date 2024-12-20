pub mod template;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Coord2D {
    pub x: i64,
    pub y: i64,
}

impl Coord2D {
    pub fn add(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn manhatten(&self, other: &Coord2D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub const DIRECTIONS: &'static [Coord2D] = &[
    Coord2D { x: 1, y: 0 },
    Coord2D { x: -1, y: 0 },
    Coord2D { x: 0, y: 1 },
    Coord2D { x: 0, y: -1 },
];

pub fn opposite_direction(c: &Coord2D) -> Coord2D {
    Coord2D { x: -c.x, y: -c.y }
}

#[derive(Clone, Debug, Default)]
pub struct Field<T> {
    pub data: Vec<T>,
    pub w: usize,
    pub h: usize,
}

impl<T> Field<T>
where
    T: Clone,
{
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.w || y >= self.h {
            return None;
        }
        return Some(self.data[y * self.w + x].clone());
    }

    pub fn get_signed(&self, x: i64, y: i64) -> Option<T> {
        if x < 0 || y < 0 || x >= self.w as i64 || y >= self.h as i64 {
            return None;
        }
        return Some(self.data[y as usize * self.w + x as usize].clone());
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.w || y >= self.h {
            return None;
        }
        return Some(&mut self.data[y * self.w + x]);
    }

    pub fn get_coord(&self, c: &Coord2D) -> Option<T> {
        return self.get_signed(c.x, c.y);
    }
    pub fn get_coord_mut(&mut self, c: &Coord2D) -> Option<&mut T> {
        if c.x < 0 || c.y < 0 || c.x >= self.w as i64 || c.y >= self.h as i64 {
            return None;
        }
        return Some(&mut self.data[c.y as usize * self.w + c.x as usize]);
    }

    pub fn set_coord(&mut self, c: &Coord2D, v: &T) {
        if c.x < 0 || c.y < 0 || c.x >= self.w as i64 || c.y >= self.h as i64 {
            panic!("Index out of bounds")
        }
        self.data[c.y as usize * self.w + c.x as usize] = v.clone();
    }

    pub fn coord_from_index(&self, i: i64) -> Coord2D {
        Coord2D {
            x: i % self.w as i64,
            y: i / self.w as i64,
        }
    }
}

impl Field<char> {
    pub fn new(input: &str) -> Field<char> {
        let mut res = Field {
            data: vec![],
            w: 0,
            h: 0,
        };
        for line in input.lines() {
            res.h += 1;
            res.w = line.len();
            for c in line.chars() {
                res.data.push(c);
            }
        }
        res
    }

    pub fn parse_vec(input: &Vec<String>) -> Field<char> {
        let mut res = Field {
            data: vec![],
            w: 0,
            h: 0,
        };
        for line in input {
            res.h += 1;
            res.w = line.len();
            for c in line.chars() {
                res.data.push(c);
            }
        }
        res
    }

    pub fn print(&self) {
        for y in 0..self.h {
            println!(
                "{}",
                self.data[y * self.w..(y + 1) * self.w]
                    .iter()
                    .collect::<String>()
            );
        }
    }
}

impl Field<bool> {
    pub fn print(&self) {
        for y in 0..self.h {
            println!(
                "{}",
                self.data[y * self.w..(y + 1) * self.w]
                    .iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

impl Field<i32> {
    pub fn new(input: &str) -> Field<i32> {
        let mut res = Field {
            data: vec![],
            w: 0,
            h: 0,
        };
        for line in input.lines() {
            res.h += 1;
            res.w = line.len();
            for c in line.chars() {
                res.data.push(c.to_string().parse::<i32>().unwrap());
            }
        }
        res
    }
}

pub fn split_by_empt_line(input: &str) -> Vec<Vec<String>> {
    let mut result = vec![];
    let mut current = vec![];
    for line in input.lines() {
        if line.is_empty() {
            result.push(current.clone());
            current.clear();
        } else {
            current.push(String::from(line));
        }
    }
    result.push(current);

    result
}
