use std::fmt::Debug;
use std::iter::zip;

#[derive(Debug, PartialEq, Clone)]
pub struct GeneralAxis {
    bin_edges: Vec<f64>,
}

pub trait Axis: Debug {
    fn bin_edges(&self) -> &Vec<f64>;

    fn min(&self) -> f64 {
        self.bin_edges()[0]
    }

    fn max(&self) -> f64 {
        *self.bin_edges().last().unwrap()
    }

    /// Total number of bins.
    fn len(&self) -> usize {
        self.bin_edges().len() - 1
    }

    fn get_bin(&self, n: usize) -> Option<(f64, f64)>  {
        if n >= self.len() {
            None
        }  else {
            Some((self.bin_edges()[n], self.bin_edges()[n + 1]))
        }
    }

    fn apply(&self, data: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.len()];
        for value in data.iter() {
            if let Some(bin) = self.find_bin(*value) {
                result[bin] += 1.0;
            }
        }
        result
    }

    fn apply_weighted(&self, data: &[f64], weights: &[f64]) -> Vec<f64> {
        if data.len() != weights.len() {
            panic!("Data and weights must have the same length.");
        }
        let mut result = vec![0.0; self.len()];

        // zip data and weights
        let data_weights = zip(data, weights);
        for (weight, value) in data_weights {
            if let Some(bin) = self.find_bin(*value) {
                result[bin] += weight;
            }
        }
        result
    }

    fn find_bin(&self, value: f64) -> Option<usize> {
        if value < self.min() {
            return None;
        }
        for (i, edge) in self.bin_edges().iter().skip(1).enumerate() {
            if *edge > value {
                return Some(i);
            }
        }
        if value == self.max() {
            return Some(self.len() - 1);
        }
        None
    }

    fn equal_bins(&self, other: &dyn Axis) -> bool {
        self.bin_edges() == other.bin_edges()
    }

    fn clone_box(&self) -> Box<dyn Axis>;
}

impl GeneralAxis {
    pub fn new(bin_edges: Vec<f64>) -> Self {
        GeneralAxis { bin_edges }
    }
}

impl From<&[f64]> for Box<GeneralAxis> {
    fn from(value: &[f64]) -> Self {
        return Box::new(GeneralAxis::new(value.to_vec()));
    }
}

impl Axis for GeneralAxis {
    fn bin_edges(&self) -> &Vec<f64> {
        &self.bin_edges
    }

    fn clone_box(&self) -> Box<dyn Axis> {
        return Box::new(GeneralAxis::new(self.bin_edges.clone()));
    }
}

// TODO: Actually have this as trait so that we can find the contents fast

#[cfg(test)]
mod tests {
    mod find_bin {
        use crate::axis::{Axis, GeneralAxis};

        #[test]
        fn test_below() {
            let ax = GeneralAxis {
                bin_edges: vec![1.0, 2.0, 3.0],
            };

            assert_eq!(ax.find_bin(1.0), Some(0));
            assert_eq!(ax.find_bin(2.2), Some(1));
            assert_eq!(ax.find_bin(3.0), Some(1));

            assert_eq!(ax.find_bin(0.2), None);
            assert_eq!(ax.find_bin(3.2), None);
        }
    }
}
