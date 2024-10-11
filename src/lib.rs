use std::error::Error;
use crate::h1::H1;

mod bin;
mod axis;
mod h1;
mod binnings;

use crate::binnings::{StandardBins, BinningAlgorithm};


pub fn h1(data: &[f64]) -> Result<H1, Box<dyn Error>> {
    let binning_algorithm = StandardBins { n_bins: 10 };
    let axis = binning_algorithm.find_axis(data)?;
    let values = axis.apply(data);
    Ok(H1::new(
        "",
        axis,
        values,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod h1 {
        use crate::bin::Bin;
        use super::*;

        #[test]
        fn is_created() {
            let h = h1(&[0.0, 1.0]).unwrap();

            // First bin should be
            let bin = h.get_bin(0);
            let Bin { value, lower, upper} = bin;

            assert_eq!(value, 1.0);
            assert_eq!(lower, 0.0);
            assert_eq!(upper, 0.1);
        }
    }
}
