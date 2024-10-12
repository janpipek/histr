use std::iter::zip;

#[derive(Debug, PartialEq, Clone)]
pub struct AxisData {
    bin_edges: Vec<f64>,
}

impl AxisData {
    /// Total number of bins.
    pub fn len(&self) -> usize {
        self.bin_edges.len() - 1
    }

    pub fn min(&self) -> f64 {
        self.bin_edges[0]
    }

    pub fn max(&self) -> f64 {
        *self.bin_edges.last().unwrap()
    }

    pub fn new(bin_edges: Vec<f64>) -> Self {
        AxisData {
            bin_edges,
        }
    }

    pub fn bin_edges(&self) -> &Vec<f64> {
        &self.bin_edges
    }

    pub fn get_bin(&self, n: usize) -> (f64, f64) {
        (self.bin_edges[n], self.bin_edges[n+1])
    }

    pub fn apply(&self, data: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.len()];
        for value in data.iter() {
            if let Some(bin) = self.find_bin(*value) {
                result[bin] += 1.0;
            }
        }
        result
    }

    pub fn apply_weighted(&self, data: &[f64], weights: &[f64]) -> Vec<f64> {
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

    pub fn find_bin(&self, value: f64) -> Option<usize> {
        if value < self.min() {
            return None;
        }
        for (i, edge) in self.bin_edges.iter().skip(1).enumerate() {
            if *edge > value {
                return Some(i);
            }
        }
        if value == self.max() {
            return Some(self.len() - 1);
        }
        None
    }
}

// TODO: Actually have this as trait so that we can find the contents fast

#[cfg(test)]
mod tests {
    mod find_bin {
        use crate::axis::AxisData;

        #[test]
        fn test_below() {
            let ax = AxisData { bin_edges: vec![1.0, 2.0, 3.0]};

            assert_eq!(ax.find_bin(1.0), Some(0));
            assert_eq!(ax.find_bin(2.2), Some(1));
            assert_eq!(ax.find_bin(3.0), Some(1));

            assert_eq!(ax.find_bin(0.2), None);
            assert_eq!(ax.find_bin(3.2), None);
        }
    }
}
