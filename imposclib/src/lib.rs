// extern crate libc;
use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyMappingProtocol};

use std::convert::From;

// use libc::c_char;
use std::collections::HashMap;
// use std::ffi::CStr;
// use std::ffi::CString;

#[pymodule]
fn imposclib(py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our Python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values, and the Rust return value back into a Python object.
    // The `_py` argument represents that we're holding the GIL.
    m.add_class::<ParameterProperties>()?;
    
    m.add_function(wrap_pyfunction!(symbol_properties, m)?)?;

    Ok(())
}

#[pyfunction]
fn symbol_properties() -> ParameterProperties {
    ParameterProperties::from(vec![("frequency", "ω")])
}

#[pyclass]
pub struct ParameterProperties {
    properties: HashMap<String, String>,
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

    fn __getitem__(&self, key: String) -> PyResult<String> {
        Ok(self.properties.get(&key).cloned().unwrap_or_default())
    }

    // fn __iter__(&self, py: Python) -> PyResult<PyObject> {
    //     let iter = IntoPy::into_py(
    //         Py::new(py, PyParameterPropertiesIter::new(self.properties.iter().map(|(k, _)| k.to_owned()).collect()))?,
    //         py,
    //     );

    //     Ok(iter)
    // }

    // fn __contains__(&self, word: String) -> PyResult<bool> {
    //     Ok(self.properties.contains_key(&word))
    // }
}

// #[pyproto]
// impl PyIterProtocol for ParameterProperties {
//     fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyObject> {
//         let mapping = &*slf;
//         let gil = Python::acquire_gil();
//         let py = gil.python();
//         let iter = IntoPy::into_py(
//             Py::new(py, PyParameterPropertiesIter::new(mapping.properties.iter().map(|(k, _)| k.to_owned()).collect()))?,
//             py,
//         );

//         Ok(iter)
//     }
// }

// #[pyclass(name = ParameterPropertiesIter)]
// pub struct PyParameterPropertiesIter {
//     v: vec::IntoIter<String>,
// }

// impl PyParameterPropertiesIter {
//     pub fn new(v: Vec<String>) -> Self {
//         PyParameterPropertiesIter { v: v.into_iter() }
//     }
// }

// #[pyproto]
// impl PyIterProtocol for PyParameterPropertiesIter {
//     fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyParameterPropertiesIter>> {
//         Ok(slf.into())
//     }

//     fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<String>> {
//         let slf = &mut *slf;
//         Ok(slf.v.next())
//     }
// }

impl ParameterProperties {
    pub fn create() -> ParameterProperties
    {
        ParameterProperties
        {
            properties: HashMap::new(),
        }
    }

    fn add(&mut self, parameter: &str, property: &str) {
        self.properties.entry(String::from(parameter)).or_insert(String::from(property));
    }

    fn property(&self, parameter: &str) -> String {
        self.properties.get(parameter).cloned().unwrap_or(String::from(""))
    }
}

impl From<Vec<(&str, &str)>> for ParameterProperties {
    fn from(records: Vec<(&str, &str)>) -> ParameterProperties {
        let mut properties = ParameterProperties::create();

        records.into_iter().for_each(|(parameter, property)| {properties.add(parameter, property);});

        properties
    }
}

// #[no_mangle]
// pub extern "C" fn parameter_properties_new() -> *mut ParameterProperties {
//     Box::into_raw(Box::new(ParameterProperties::new()))
// }

// #[no_mangle]
// pub extern "C" fn parameter_properties_free(ptr: *mut ParameterProperties) {
//     if ptr.is_null() {
//         return;
//     }
//     unsafe {
//         Box::from_raw(ptr);
//     }
// }

// #[no_mangle]
// pub extern "C" fn parameter_properties_populate(ptr: *mut ParameterProperties) {
//     let database = unsafe {
//         assert!(!ptr.is_null());
//         &mut *ptr
//     };
//     database.populate();
// }

// #[no_mangle]
// pub extern "C" fn parameter_properties_population_of(
//     ptr: *const ParameterProperties,
//     zip: *const c_char,
// ) -> u32 {
//     let database = unsafe {
//         assert!(!ptr.is_null());
//         &*ptr
//     };
//     let zip = unsafe {
//         assert!(!zip.is_null());
//         CStr::from_ptr(zip)
//     };
//     let zip_str = zip.to_str().unwrap();
//     database.population_of(zip_str)
// }

#[cfg(test)]
mod tests {
    use crate::ParameterProperties;

    #[test]
    fn can_access_parameter_properties() {
        let mut properties = ParameterProperties::from(vec![("frequency", "ω")]);
        assert_eq!(properties.property("frequency"), String::from("ω"));
        assert_eq!(properties.property("period"), String::from(""));
    }
}
