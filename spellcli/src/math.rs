//! Components used for math

/// Keeps track of vales that are to be averaged
/// Coppyed from one of my other projects [My Own Planet](https://github.com/looking-g/My-Own-Planet)
#[derive(Clone)]
pub struct Average{
    sum: f32,
    count: u32,
}

impl Average{
    /// Makes a new counter
    pub fn new() -> Self{
        Self{
            sum: 0.0,
            count: 0,
        }
    }

    /// Adds a value to the count
    pub fn add(&mut self, val: f32){
        self.sum += val;
        self.count += 1_u32;
    }

    /// Calculates the average
    pub fn solve(&self) -> Option<f32>{
        if self.count == 0{
            None
        } else {
            Some(self.sum / self.count as f32)
        }
    }

}

