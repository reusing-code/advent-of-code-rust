pub mod template;

#[derive(Clone)]
pub struct Field<T> {
    data: Vec<T>,
    pub w: usize,
    pub h: usize,
}

impl<T> Field<T>
where
    T: Clone,
{
    pub fn get(&self, x: usize, y: usize) -> T {
        if x >= self.w || y >= self.h {
            panic!("Index out of bounds")
        }
        return self.data[y * self.w + x].clone();
    }

    pub fn get_signed(&self, x: i32, y: i32) -> T {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            panic!("Index out of bounds")
        }
        return self.data[y as usize * self.w + x as usize].clone();
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
