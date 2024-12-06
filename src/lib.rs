pub mod template;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord2D {
    pub x: i32,
    pub y: i32,
}

impl Coord2D {
    pub fn add(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
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

    pub fn get_signed(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return None;
        }
        return Some(self.data[y as usize * self.w + x as usize].clone());
    }

    pub fn get_coord(&self, c: &Coord2D) -> Option<T> {
        return self.get_signed(c.x, c.y);
    }

    pub fn set_coord(&mut self, c: &Coord2D, v: &T) {
        if c.x < 0 || c.y < 0 || c.x >= self.w as i32 || c.y >= self.h as i32 {
            panic!("Index out of bounds")
        }
        self.data[c.y as usize * self.w + c.x as usize] = v.clone();
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
