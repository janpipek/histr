use pyo3::prelude::*;
use histr::*;

#[pyclass(name = "H1")]
pub struct PyH1 {
    inner: H1<'static>,
}

#[pyfunction(name = "h1")]
fn py_h1(py: Python<'_>, data: PyObject) -> PyResult<PyH1> {
    let values = data.extract::<Vec<f64>>(py)?;
    let h1 = h1!(&values).unwrap();
    Ok(PyH1 { inner: h1 })
}

#[pymodule(name = "histr")]
fn py_histr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyH1>()?;
    m.add_function(wrap_pyfunction!(py_h1, m)?)?;
    Ok(())
}