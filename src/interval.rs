pub struct Interval {
    pub low: f64,
    pub high: f64,
}

impl Interval {
    pub fn new(low: f64, high: f64) -> Self {
        Self{low, high}
    }

    pub fn contains(&self, num: f64) -> bool {
        return self.low <= num && num <= self.high;
    }

    pub fn shrink_right(&mut self, h: f64) -> () {
        self.high = h;
    }
}
