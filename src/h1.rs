use crate::axis::AxisData;
use crate::bin::Bin;

#[derive(Debug)]
pub struct H1 {
    name: Option<String>,
    axis: AxisData,
    bin_contents: Vec<f64>,
}

impl H1 {
    pub fn new(name: &str, axis: AxisData, bin_contents: Vec<f64>) -> Self {
        if axis.len() != bin_contents.len() {
            panic!("Axis and contents lengths must match.");
        }
        H1 {
            name: Some(String::from(name)),
            axis,
            bin_contents
        }
    }

    pub fn axis(&self) -> &AxisData {
        &self.axis
    }

    pub fn bin_contents(&self) -> &Vec<f64> {
        &self.bin_contents
    }

    pub fn get_bin(&self, n: usize) -> Bin {
        let bin_edges = self.axis.get_bin(n);
        Bin {
            lower: bin_edges.0,
            upper: bin_edges.1,
            value: self.bin_contents[n]
        }
    }
}
