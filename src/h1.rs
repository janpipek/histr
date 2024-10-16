use crate::axis::Axis;
use crate::bin::Bin;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct H1<'a> {
    // We will probably want some meta-data here
    axis: Box<dyn Axis + 'a>,
    bin_contents: Vec<f64>,
}

impl<'a> H1<'a> {
    pub fn new(axis: Box<dyn Axis>, bin_contents: Vec<f64>) -> Self {
        if axis.len() != bin_contents.len() {
            panic!("Axis and contents lengths must match.");
        }
        H1 { axis, bin_contents }
    }

    pub fn axis(&self) -> &dyn Axis {
        self.axis.as_ref()
    }

    pub fn bin_contents(&self) -> &Vec<f64> {
        &self.bin_contents
    }

    pub fn len(&self) -> usize {
        self.axis.len()
    }

    pub fn total(&self) -> f64 {
        self.bin_contents.iter().sum()
    }

    pub fn get_bin(&self, n: usize) -> Option<Bin> {
        if let Some(bin_edges) = self.axis().get_bin(n) {
            Some(Bin {
                lower: bin_edges.0,
                upper: bin_edges.1,
                value: self.bin_contents[n],
            })
        }
        else { None }
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
        self.axis
            .apply(values)
            .iter()
            .enumerate()
            .for_each(|(bin, value)| {
                self.bin_contents[bin] += value;
            });
    }

    pub fn fill_weighted_many(&mut self, values: &[f64], weights: &[f64]) {
        self.axis
            .apply_weighted(values, weights)
            .iter()
            .enumerate()
            .for_each(|(bin, value)| {
                self.bin_contents[bin] += value;
            });
    }
}

impl<'a> Add<&H1<'_>> for &H1<'a> {
    type Output = Result<H1<'static>, &'static str>;

    fn add(self, other: &H1) -> Result<H1<'static>, &'static str> {
        if !self.axis.equal_bins(other.axis()) {
            return Err("Cannot add histograms with different axes.");
        }
        Ok(H1 {
            axis: self.axis.clone_box(), // or not clone?
            bin_contents: self
                .bin_contents
                .iter()
                .zip(other.bin_contents.iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }
}

impl<'a> Mul<f64> for &H1<'a> {
    type Output = Result<H1<'static>, &'static str>;

    fn mul(self, other: f64) -> Result<H1<'static>, &'static str> {
        Ok(H1 {
            axis: self.axis.clone_box(), // or not clone?
            bin_contents: self.bin_contents.iter().map(|&a| other * a).collect(),
        })
    }
}

// TODO: Add support for other numeric types

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis::GeneralAxis;
    use crate::bin::Bin;
    use std::error::Error;

    fn get_h1() -> H1<'static> {
        // fixture
        H1 {
            axis: Box::new(GeneralAxis::new(vec![0., 1., 2., 3.])),
            bin_contents: vec![1.0, 2.0, 3.0],
        }
    }

    #[test]
    fn test_len() {
        assert_eq!(get_h1().len(), 3)
    }

    #[test]
    fn test_total() {
        let h1 = get_h1();
        assert_eq!(h1.total(), 6.0);
    }

    #[test]
    fn test_mul() -> Result<(), Box<dyn Error>> {
        let h1 = get_h1();
        let h1_times_3 = (&h1 * 3.0)?;

        assert!(h1_times_3.axis().equal_bins(h1.axis()));
        assert_eq!(h1_times_3.bin_contents(), &vec![3.0, 6.0, 9.0]);
        Ok(())
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn Error>> {
        let h1 = get_h1();
        let h1_times_2 = (&h1 + &h1)?;

        assert!(h1_times_2.axis().equal_bins(h1.axis()));
        assert_eq!(h1_times_2.bin_contents(), &vec![2.0, 4.0, 6.0]);
        Ok(())
    }

    #[test]
    fn test_fill() {
        let mut h1 = get_h1();

        h1.fill(1.5);  // 2nd bin
        assert_eq!(h1.bin_contents(), &vec![1.0, 3.0, 3.0]);

        h1.fill(10.5); // outside bounds
        assert_eq!(h1.bin_contents(), &vec![1.0, 3.0, 3.0]);
    }

    #[test]
    fn test_fill_many() {
        let mut h1 = get_h1();

        // Some values out of bounds
        h1.fill_many(&[-5., 1.5, 2.3, 7.5]);
        assert_eq!(h1.bin_contents(), &vec![1.0, 3.0, 4.0]);
    }

    #[test]
    fn test_get_bin() {
        let h1 = get_h1();
        let bin = h1.get_bin(1).unwrap();
        assert_eq!(bin, Bin { value: 2.0, lower: 1.0, upper: 2.0});

        assert_eq!(h1.get_bin(h1.len()), None);
    }
}
