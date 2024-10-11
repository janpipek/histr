use crate::axis::AxisData;

pub trait BinningAlgorithm {
    fn find_axis(&self, data: &[f64]) -> Result<AxisData, &str>;
}


pub struct StandardBins {
    pub n_bins: usize,
}

impl StandardBins {
    fn split_interval(&self, min: f64, max: f64) -> Result<AxisData, &str> {
        let n: i64 = self.n_bins.try_into().unwrap();
        if min == max {
            return Err("Lower and upper bounds are equal:");
        }
        let bin_width = (max - min) / (n as f64);
        Ok(AxisData::new((0..=n).map(|i| (i as f64) * bin_width + min).collect()))
    }
}

impl BinningAlgorithm for StandardBins {
    fn find_axis(&self, data: &[f64]) -> Result<AxisData, &str> {
        if data.len() < 2 {
            return Err("Not enough bins");
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
        self.split_interval(min, max)
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