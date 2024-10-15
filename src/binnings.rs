use crate::axis::{Axis, GeneralAxis};

pub trait BinningAlgorithm {
    fn find_axis(&self, data: &[f64]) -> Result<Box<impl Axis>, &str>;
}


/// StandardBins is a simple binning algorithm that splits the data into n_bins
///
/// This mimics the behavior of numpy.histogram with a number of bins.
pub struct StandardBins {
    pub n_bins: u64,
}

fn find_bounds(data: &[f64]) -> Result<(f64, f64), &'static str> {
    // The default, same as numpy.histogram
    if data.is_empty() {
        return Ok((0.0, 1.0));
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for value in data.iter() {
        if value.is_nan() {
            return Err("NaNs in the data");
        }
        if value.is_infinite() {
            return Err("Infinite values in the data");
        }
        if *value < min { min = *value; }
        if *value > max { max = *value; }
    }
    Ok((min, max))
}


impl StandardBins {
    fn split_interval(&self, min: f64, max: f64) -> Result<Vec<f64>, &'static str> {
        if min.is_infinite() {
            return Err("Infinite values in the data");
        }
        if max.is_infinite() {
            return Err("Infinite values in the data");
        }

        // Update the min/max if values are incomplete
        let (min, max) = if min == max {
            (min - 0.5, max + 0.5)
            // Numpy return: (min - 0.5, max + 0.5)
        } else {
            (min, max)
        };
        let bin_width = (max - min) / self.n_bins as f64;

        // Compute all bins but for the last to avoid rounding errors
        let mut raw_data: Vec<f64> = (0..self.n_bins).map(|i| (i as f64) * bin_width + min).collect();
        raw_data.push(max);

        Ok(raw_data)
    }
}

impl BinningAlgorithm for StandardBins {
    fn find_axis<'a>(&self, data: &'a [f64]) -> Result<Box<GeneralAxis>, &'static str> {
        let (min, max) = find_bounds(data)?;
        let raw_data = self.split_interval(min, max)?;
        Ok(Box::new(GeneralAxis::new(raw_data)))
    }
}

pub struct FixedWidthBins {
    pub bin_width: f64,
}

impl BinningAlgorithm for FixedWidthBins {
    fn find_axis(&self, data: &[f64]) -> Result<Box<GeneralAxis>, &'static str> {
        Ok(Box::new(GeneralAxis::new(find_fixed_width_bins(&data, self.bin_width)?)))
    }
}

fn find_fixed_width_bins(data: &[f64], bin_width: f64) -> Result<Vec<f64>, &'static str>  {
    let (min, max) = find_bounds(data)?;

    let min_index = (min / bin_width).floor();
    let min_edge = min_index * bin_width;
    let n_bins: i64 = ((max - min_edge) / bin_width).floor() as i64;

    let raw_data: Vec<f64> = (0..=n_bins).map(|i| (i as f64) * bin_width + min_edge).collect();
    Ok(raw_data)
}

pub struct PrettyBins {
    pub approx_bins: usize,
}

impl BinningAlgorithm for PrettyBins {
    fn find_axis(&self, data: &[f64]) -> Result<Box<GeneralAxis>, &'static str> {
        let (min, max) = find_bounds(data)?;
        let raw_width = (max - min) / self.approx_bins as f64;
        let bin_width = find_pretty_width(raw_width);
        Ok(Box::new(GeneralAxis::new(find_fixed_width_bins(&data, bin_width)?)))
    }
}

fn find_pretty_width(raw_width: f64) -> f64 {
    let subscales = [0.5, 1., 2., 2.5, 5., 10.];
    let power = raw_width.log10().floor() as i32;
    let normalized_subscale: f64 = raw_width / (10.0f64).powi(power);

    let mut min_subscale = 0.0;
    let mut min_log_distance = f64::INFINITY;
    for subscale in subscales.iter() {
        let log_distance = (normalized_subscale / subscale).log10().abs();
        if log_distance < min_log_distance {
            min_log_distance = log_distance;
            min_subscale = *subscale
        }
    }
    min_subscale * (10.0f64).powi(power)
}

#[cfg(test)]
mod tests {
    mod standard_bins {
        use std::error::Error;
        use crate::axis::{GeneralAxis};
        use crate::binnings::{BinningAlgorithm, StandardBins};

        #[test]
        fn valid_data() -> Result<(), Box<dyn Error>> {
            let data = vec![0.0, 0.1, 1.0];
            let algo = StandardBins { n_bins: 4 };
            let axis = algo.find_axis(&data)?;

            let expected = Box::new(GeneralAxis::new(
                vec![0.0, 0.25, 0.5, 0.75, 1.0]
            ));

            assert_eq!(expected, axis);
            Ok(())
        }
    }

    mod pretty_bins {
        use std::error::Error;
        use crate::axis::{GeneralAxis};
        use crate::binnings::{BinningAlgorithm, PrettyBins, find_pretty_width};

        #[test]
        fn valid_data() -> Result<(), Box<dyn Error>> {
            let data = vec![0.23, 0.47, 0.63, 0.83];
            let algo = PrettyBins { approx_bins: 4 };
            let axis = algo.find_axis(&data)?;

            let expected = Box::new(GeneralAxis::new(
                vec![0.0, 0.25, 0.5, 0.75, 1.0]
            ));

            assert_eq!(expected, axis);
            Ok(())
        }

        #[test]
        fn test_find_pretty_width() {
            assert_eq!(find_pretty_width(0.1), 0.1);
            assert_eq!(find_pretty_width(6.4), 5.0);
            assert_eq!(find_pretty_width(23.0), 25.0);
            assert_eq!(find_pretty_width(125658.0), 1e5);
        }
    }
}