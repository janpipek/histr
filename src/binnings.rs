use crate::axis::AxisData;

pub trait BinningAlgorithm {
    fn find_axis(&self, data: &[f64]) -> Result<AxisData, &str>;
}


/// StandardBins is a simple binning algorithm that splits the data into n_bins
///
/// This mimics the behavior of numpy.histogram with a number of bins.
pub struct StandardBins {
    pub n_bins: usize,
}


impl StandardBins {
    fn split_interval(&self, min: f64, max: f64) -> AxisData {
        let n: i64 = self.n_bins.try_into().unwrap();

        // Update the min/max if values are incomplete
        let (min, max) = if min == max {
            (min - 0.5, max + 0.5)
            // Numpy return: (min - 0.5, max + 0.5)
        } else {
            (min, max)
        };
        let bin_width = (max - min) / (n as f64);

        // Compute all bins but for the last to avoid rounding errors
        let mut raw_data: Vec<f64> = (0..n).map(|i| (i as f64) * bin_width + min).collect();
        raw_data.push(max);

        AxisData::new(raw_data)
    }
}

impl BinningAlgorithm for StandardBins {
    fn find_axis(&self, data: &[f64]) -> Result<AxisData, &str> {
        if data.is_empty() {
            return Ok(self.split_interval(0.0, 1.0))
        }

        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for value in data.iter() {
            if value.is_nan() {
                return Err("NaNs in the data");
            }
            if *value < min { min = *value; }
            if *value > max { max = *value; }
        }
        Ok(self.split_interval(min, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod standard_bins {
        use crate::axis::AxisData;
        use crate::binnings::{BinningAlgorithm, StandardBins};

        #[test]
        fn valid_data() {
            let data = vec![0.0, 0.1, 1.0];
            let algo = StandardBins { n_bins: 4 };
            let axis = algo.find_axis(&data);

            let expected = AxisData::new(
                vec![0.0, 0.25, 0.5, 0.75, 1.0]
            );

            assert_eq!(Ok(expected), axis)
        }
    }
}