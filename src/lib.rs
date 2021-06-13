use pyo3::prelude::*;

#[pyclass]
struct Pet {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    sound: String
}

#[pymethods]
impl Pet {
    #[new]
    pub fn new(name: String, sound: String) -> Self {
        Pet { name, sound }
    }
}

#[pymodule]
fn pet(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pet>()?;

    Ok(())
}

