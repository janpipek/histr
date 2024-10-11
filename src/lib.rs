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


}
