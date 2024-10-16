mod axis;
mod bin;
mod binnings;
mod h1;

use std::error::Error;

use crate::axis::{Axis, GeneralAxis};
use crate::binnings::{BinningAlgorithm, StandardBins};
use crate::h1::H1;

pub fn h1(data: &[f64]) -> Result<H1<'static>, Box<dyn Error>> {
    let binning_algorithm = StandardBins { n_bins: 10 };
    let axis = binning_algorithm.find_axis(data)?;
    let values = axis.as_ref().apply(data);
    Ok(H1::new(axis, values))
}

pub fn h1_with_bins(data: &[f64], bins: &[f64]) -> H1<'static> {
    let axis: Box<GeneralAxis> = Box::from(bins);
    let values = axis.as_ref().apply(data);
    H1::new(axis, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod h1 {
        use super::*;
        use crate::bin::Bin;

        #[test]
        fn is_created() {
            let h = h1(&[0.0, 1.0]).unwrap();

            // First bin should be
            let bin = h.get_bin(0);
            let Bin {
                value,
                lower,
                upper,
            } = bin;

            assert_eq!(value, 1.0);
            assert_eq!(lower, 0.0);
            assert_eq!(upper, 0.1);
        }
    }
}
