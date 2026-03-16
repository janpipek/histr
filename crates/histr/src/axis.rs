use std::borrow::Cow;
use std::fmt::Debug;
use std::iter::zip;

#[derive(Debug, PartialEq, Clone)]
pub struct GeneralAxis {
    bin_edges: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FixedWidthAxis { min_edge: f64,
    max_edge: f64,
    bin_width: f64,
    n_bins: usize,
}

pub trait Axis: Debug + Send + Sync {
    fn bin_edges(&self) -> Cow<Vec<f64>>;

    fn min_edge(&self) -> f64 {
        self.bin_edges()[0]
    }

    fn max_edge(&self) -> f64 {
        *self.bin_edges().last().unwrap()
    }

    /// Total number of bins.
    fn len(&self) -> usize {
        self.bin_edges().len() - 1
    }

    fn get_bin(&self, n: usize) -> Option<(f64, f64)> {
        if n >= self.len() {
            None
        } else {
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

    fn apply_weighted(&self, data: &[f64], weights: &[f64]) -> Result<Vec<f64>, &'static str> {
        if data.len() != weights.len() {
            return Err("Data and weights must have the same length.");
        }
        let mut result = vec![0.0; self.len()];

        // zip data and weights
        let data_weights = zip(data, weights);
        for (value, weight) in data_weights {
            if let Some(bin) = self.find_bin(*value) {
                result[bin] += weight;
            }
        }
        Ok(result)
    }

    fn find_bin(&self, value: f64) -> Option<usize> {
        if value < self.min_edge() {
            return None;
        }
        for (i, edge) in self.bin_edges().iter().skip(1).enumerate() {
            if *edge > value {
                return Some(i);
            }
        }
        if value == self.max_edge() {
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

impl FixedWidthAxis {
    pub fn new(min_edge: f64, max_edge: f64, bin_width: f64, n_bins: usize) -> Self {
        Self {
            min_edge,
            max_edge,
            bin_width,
            n_bins
        }
    }

    pub fn create_from_min_and_bins(min_edge: f64, bin_width: f64, n_bins: usize) -> Self {
        Self {
            min_edge,
            max_edge: min_edge + bin_width * n_bins as f64,
            bin_width,
            n_bins
        }
    }

    pub fn create_from_range(min_edge: f64, max_edge: f64, bin_width: f64) -> Self {
        // TODO: Make sure 5.00000001 bins do not become 6
        Self {
            min_edge,
            max_edge,
            bin_width,
            n_bins: ((max_edge - min_edge) / bin_width).ceil() as usize,
        }
    }
}

impl Axis for FixedWidthAxis {
    fn bin_edges(&self) -> Cow<Vec<f64>> {
        let mut edges = vec![self.min_edge];
        for i in 1..(self.n_bins as usize) {
            edges.push(self.min_edge + i as f64 * self.bin_width);
        }
        edges.push(self.max_edge);
        Cow::Owned(edges)
    }

    fn min_edge(&self) -> f64 {
        self.min_edge
    }

    fn max_edge(&self) -> f64 {
        self.max_edge
    }
    
    fn find_bin(&self, value: f64) -> Option<usize> {
        if value < self.min_edge {
            return None;
        }
        if value == self.max_edge {
            return Some((self.n_bins - 1) as usize);
        }
        let bin = ((value - self.min_edge) / self.bin_width).floor() as usize;
        if bin >= self.n_bins {
            return None;
        }
        Some(bin)
    }

    fn clone_box(&self) -> Box<dyn Axis> {
        return Box::new(FixedWidthAxis {
            min_edge: self.min_edge,
            n_bins: self.n_bins,
            max_edge: self.max_edge,
            bin_width: self.bin_width,
        });
    }
}

impl From<&[f64]> for Box<GeneralAxis> {
    fn from(value: &[f64]) -> Self {
        return Box::new(GeneralAxis::new(value.to_vec()));
    }
}

impl Axis for GeneralAxis {
    fn bin_edges(&self) -> Cow<Vec<f64>> {
        Cow::Borrowed(&self.bin_edges)
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
