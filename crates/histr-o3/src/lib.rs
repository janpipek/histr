use pyo3::exceptions::PyException;
use pyo3::PyObject;
use pyo3::prelude::*;
use histr::*;

#[pyclass(name = "H1")]
pub struct PyH1 {
    inner: H1<'static>,
}

#[pymethods]
impl PyH1 {
    #[getter]
    fn bin_contents(&self, py: Python) -> PyObject {
        self.inner.bin_contents().clone().into_py(py)
    }

    #[getter]
    fn bin_edges(&self, py: Python) -> PyObject {
        self.inner.axis().bin_edges().clone().into_py(py)
    }

    #[getter]
    fn total(&self) -> f64 {
        self.inner.total()
    }

    fn fill(&mut self, value: f64) {
        self.inner.fill(value);
    }

    fn __repr__(&self) -> String {
        format!("H1({} bins)", self.inner.len())
    }
}

#[pyfunction(name = "h1")]
#[pyo3(signature = (data, *, bin_width=None, bin_edges=None))]
fn py_h1(py: Python<'_>, data: PyObject, bin_width: Option<f64>, bin_edges: Option<PyObject>) -> PyResult<PyH1> {
    let values: Vec<f64> = data.extract(py)?;
    let h1 = match bin_width {
        Some(bin_width) => h1!(&values, bin_width: bin_width),
        None => match bin_edges {
            Some(bin_edges) => {
                let bin_edges: Vec<f64> = bin_edges.extract(py)?;
                Ok(h1!(&values, bin_edges: &bin_edges))
            }
            None => h1!(&values),
        }
    };
    match h1 {
        Ok(h1) => Ok(PyH1 { inner: h1 }),
        Err(_e) => Err(PyErr::new::<PyException, _>("Error creating H1")),
    }
}

#[pymodule(name="_histr")]
fn _histr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyH1>()?;
    m.add_function(wrap_pyfunction!(py_h1, m)?)?;
    Ok(())
}