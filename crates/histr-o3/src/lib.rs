use pyo3::exceptions::PyException;
use pyo3::PyObject;
use pyo3::prelude::*;
use histr::*;
use histr::binnings::*;

#[pyclass(name = "H1")]
pub struct PyH1 {
    inner: H1<'static>,
}

#[pymethods]
impl PyH1 {
    fn bin_contents(&self, py: Python) -> PyObject {
        self.inner.bin_contents().clone().into_py(py)
    }

    fn bin_edges(&self, py: Python) -> PyObject {
        self.inner.axis().bin_edges().clone().into_py(py)
    }

    fn total(&self) -> f64 {
        self.inner.total()
    }
}

#[pyfunction(name = "h1")]
#[pyo3(signature = (data, *, bin_width))]
fn py_h1(py: Python<'_>, data: PyObject, bin_width: Option<f64>) -> PyResult<PyH1> {
    let values: Vec<f64> = data.extract(py)?;
    let h1 = match bin_width {
        Some(bin_width) => h1!(&values, bin_width: bin_width),
        None => h1!(&values),
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