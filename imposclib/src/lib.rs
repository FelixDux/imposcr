// Using [PyO3](https://pyo3.rs/v0.14.1/), with [maturin](https://crates.io/crates/maturin) for distribution

use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyMappingProtocol};
use pyo3::types::{PyDict, IntoPyDict};

use std::convert::From;

use std::collections::HashMap;

#[pymodule]
fn imposclib(_py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our Python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values, and the Rust return value back into a Python object.
    // The `_py` argument represents that we're holding the GIL.
    m.add_class::<PropertyPair>()?;
    m.add_class::<ParameterProperties>()?;
    
    m.add_function(wrap_pyfunction!(symbol_properties, m)?)?;

    Ok(())
}

#[pyfunction]
fn symbol_properties() -> ParameterProperties {
    ParameterProperties::from(vec![("frequency", "ω"),("offset", "σ"), ("phi", "φ")])
}

#[pyclass]
#[derive(Clone, Default)]
pub struct PropertyPair {
    parameter: String,
    property: String,
}

#[pymethods]
impl PropertyPair {
    #[new]
    fn new() -> PyResult<Self>
    {
        Ok(PropertyPair
        {
            parameter: String::new(),
            property: String::new(),
        })
    }

}

impl From<(&str, &str)> for PropertyPair {
    fn from((parameter, property): (&str, &str)) -> PropertyPair {
        PropertyPair {
            parameter: String::from(parameter),
            property: String::from(property),
        }
    }
}

#[pyproto]
impl PyIterProtocol for PropertyPair {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyObject> {
        let props = &*slf;
        let gil = Python::acquire_gil();
        let py = gil.python();
        let vals = vec![(String::from("Parameter"), props.parameter.clone()), (String::from("Property"), props.property.clone())];
        let iter = IntoPy::into_py(
            Py::new(py, PyPropertyPairIter::new(vals))?,
            py,
        );

        Ok(iter)
    }
}

#[pyclass(name = "PropertyPairIter")]
pub struct PyPropertyPairIter {
    v: std::vec::IntoIter<(String, String)>,
}

impl PyPropertyPairIter {
    pub fn new(v: Vec<(String, String)>) -> Self {
        PyPropertyPairIter { v: v.into_iter() }
    }
}

#[pyproto]
impl PyIterProtocol for PyPropertyPairIter {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyPropertyPairIter>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<(String, String)>> {
        let slf = &mut *slf;
        Ok(slf.v.next())
    }
}

#[pyclass]
pub struct ParameterProperties {
    properties: HashMap<String, PropertyPair>,
}

#[pymethods]
impl ParameterProperties {
    #[new]
    fn new() -> PyResult<Self>
    {
        Ok(ParameterProperties
        {
            properties: HashMap::new(),
        })
    }
}

#[pyproto]
impl PyMappingProtocol for ParameterProperties {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.properties.len())
    }

    fn __getitem__(&self, key: String) -> PyResult<PropertyPair> {
        Ok(self.properties.get(&key).cloned().unwrap_or_default())
    }
}

#[pyproto]
impl PyIterProtocol for ParameterProperties {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyObject> {
        let props = &*slf;
        let gil = Python::acquire_gil();
        let py = gil.python();
        let iter = IntoPy::into_py(
            Py::new(py, PyParameterPropertiesIter::new(props.properties.iter().map(|(_, v)| v.to_owned()).collect()))?,
            py,
        );

        Ok(iter)
    }
}

#[pyclass(name = "ParameterPropertiesIter")]
pub struct PyParameterPropertiesIter {
    v: std::vec::IntoIter<PropertyPair>,
}

impl PyParameterPropertiesIter {
    pub fn new(v: Vec<PropertyPair>) -> Self {
        PyParameterPropertiesIter { v: v.into_iter() }
    }
}

#[pyproto]
impl PyIterProtocol for PyParameterPropertiesIter {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyParameterPropertiesIter>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PropertyPair>> {
        let slf = &mut *slf;
        Ok(slf.v.next())
    }
}

impl ParameterProperties {
    pub fn create() -> ParameterProperties
    {
        ParameterProperties
        {
            properties: HashMap::new(),
        }
    }

    fn add(&mut self, parameter: &str, property: &str) {
        self.properties.entry(String::from(parameter)).or_insert(PropertyPair::from((property, parameter)));
    }

    fn property(&self, parameter: &str) -> PropertyPair {
        self.properties.get(parameter).cloned().unwrap_or(PropertyPair::from((parameter, "")))
    }
}

impl From<Vec<(&str, &str)>> for ParameterProperties {
    fn from(records: Vec<(&str, &str)>) -> ParameterProperties {
        let mut properties = ParameterProperties::create();

        records.into_iter().for_each(|(parameter, property)| {properties.add(parameter, property);});

        properties
    }
}


#[cfg(test)]
mod tests {
    use crate::ParameterProperties;

    #[test]
    fn can_access_parameter_properties() {
        let properties = ParameterProperties::from(vec![("frequency", "ω")]);
        assert_eq!(properties.property("frequency"), String::from("ω"));
        assert_eq!(properties.property("period"), String::from(""));
    }
}
