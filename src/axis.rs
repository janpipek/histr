#[derive(Debug, PartialEq)]
pub struct AxisData {
    bin_edges: Vec<f64>,
}

impl AxisData {
    /// Total number of bins.
    pub fn len(&self) -> usize {
        self.bin_edges.len() - 1
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
}


// TODO: Actually have this as trait so that we can find the contents fast