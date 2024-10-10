// TODO: parameterize types
#[derive(Debug)]
pub struct Bin {
    pub value: f64,
    pub lower: f64,
    pub upper: f64,
}

impl Bin {
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}