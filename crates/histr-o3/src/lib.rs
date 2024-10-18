use pyo3::PyObject;
use pyo3::prelude::*;
use histr::*;

#[pyclass(name = "H1")]
pub struct PyH1 {
    inner: H1<'static>,
}

#[pymethods]
impl PyH1 {
    fn bin_contents(&self, py: Python) -> PyObject {
        self.inner.bin_contents().clone().into_py(py)
    }
}

#[pyfunction(name = "h1")]
fn py_h1(py: Python<'_>, data: PyObject) -> PyResult<PyH1> {
    let values: Vec<f64> = data.extract(py)?;
    let h1 = h1!(&values).unwrap();
    Ok(PyH1 { inner: h1 })
}

#[pymodule(name = "histr")]
fn py_histr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyH1>()?;
    m.add_function(wrap_pyfunction!(py_h1, m)?)?;
    Ok(())
}