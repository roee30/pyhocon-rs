/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
use pyo3::prelude::*;
use pyo3::conversion::IntoPyObjectExt;
use pyo3::types::{PyList, PyDict};
use hocon::{Hocon,HoconLoader, Error};

fn map_err<T>(value: Result<T, Error>) -> PyResult<T> {
    value.map_err(|x| pyo3::exceptions::PyValueError::new_err(x.to_string()))
}
#[pyfunction]
fn parse_file(py: Python<'_>, path: &str) -> PyResult<PyObject> {
    let rust_hocon: Hocon = map_err(parse_file_(path))?;
    Ok(to_py_object(py,&rust_hocon))
}
#[pyfunction]
fn parse_string(py: Python<'_>, input: &str) -> PyResult<PyObject> {
    let loader: HoconLoader = map_err(HoconLoader::new().load_str(input))?;
    Ok(to_py_object(py, &map_err(loader.hocon())?))
}
fn parse_file_(path: &str) -> Result<Hocon, Error>{
    let loader: HoconLoader = HoconLoader::new().load_file(path)?;
    loader.hocon()
}

fn to_py_object(py: Python<'_>, h: &Hocon) -> PyObject {
    match h {
        Hocon::Real(x) => x.into_py_any(py).unwrap(),
        Hocon::Integer(x) => x.into_py_any(py).unwrap(),
        Hocon::String(s) => s.into_py_any(py).unwrap(),
        Hocon::Boolean(b) => b.into_py_any(py).unwrap(),
        Hocon::Array(arr) => {
            let v: Vec<PyObject> = arr.iter().map(|h| to_py_object(py, h)).collect();
            PyList::new(py, v).unwrap().unbind().into_py_any(py).unwrap()
        }
        Hocon::Hash(map) => {
            let d = PyDict::new(py);
            for (k, v) in map.iter() {
                let _ = d.set_item(k, to_py_object(py, v));
            }
            d.into_py_any(py).unwrap()
        }
        Hocon::Null => py.None(),
        Hocon::BadValue(e) => {
            // Represent as a ValueError instance (object), to keep "to_python" pure.
            pyo3::exceptions::PyValueError::new_err(e.to_string()).into_py_any(py).unwrap()
        }
    }
}
#[pymodule]

fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    Ok(())
}
