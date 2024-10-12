use crate::axis::AxisData;
use crate::bin::Bin;
use std::ops::Add;

#[derive(Debug)]
pub struct H1 {
    // We will probably want some meta-data here

    axis: AxisData,
    bin_contents: Vec<f64>,
}

impl H1 {
    pub fn new(axis: AxisData, bin_contents: Vec<f64>) -> Self {
        if axis.len() != bin_contents.len() {
            panic!("Axis and contents lengths must match.");
        }
        H1 {
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

    pub fn fill(&mut self, value: f64) {
        if let Some(bin) = self.axis.find_bin(value) {
            self.bin_contents[bin] += 1.0;
        }
    }

    pub fn fill_weighted(&mut self, value: f64, weight: f64) {
        if let Some(bin) = self.axis.find_bin(value) {
            self.bin_contents[bin] += weight;
        }
    }

    pub fn fill_many(&mut self, values: &[f64]) {
        self.axis.apply(values).iter().enumerate().for_each(|(bin, value)| {
            self.bin_contents[bin] += value;
        });
    }

    pub fn fill_weighted_many(&mut self, values: &[f64], weights: &[f64]) {
        self.axis.apply_weighted(values, weights).iter().enumerate().for_each(|(bin, value)| {
            self.bin_contents[bin] += value;
        });
    }
}

impl Add for H1 {
    type Output = Result<Self, &'static str>;

    fn add(self, other: Self) -> Result<Self, &'static str> {
        if self.axis != other.axis {
            return Err("Cannot add histograms with different axes.");
        }
        Ok(
        Self {
            axis: self.axis.clone(), // or not clone?
            bin_contents: self.bin_contents.iter().zip(other.bin_contents.iter()).map(|(a, b)| a + b).collect()
        })
    }
}