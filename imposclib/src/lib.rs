#![crate_name = "imposclib"]
// #![feature(result_contains_err)]
// Using [PyO3](https://pyo3.rs/v0.14.1/), with [maturin](https://crates.io/crates/maturin) for distribution

use log::debug;

use pyo3::prelude::*;
use pyo3::{PyIterProtocol, PyMappingProtocol, PySequenceProtocol};
use pyo3::types::{PyDict, IntoPyDict};

use std::convert::From;

use std::collections::HashMap;

mod io {
    pyo3::import_exception!(io, IndexError);
    pyo3::import_exception!(io, ValueError);
}

mod dynamics;

#[pymodule]
fn imposclib(_py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our Python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values, and the Rust return value back into a Python object.
    // The `_py` argument represents that we're holding the GIL.
    m.add_class::<PropertyPair>()?;
    m.add_class::<ParameterProperties>()?;
    m.add_class::<PyImpact>()?;
    m.add_class::<IterationInputs>()?;
    m.add_class::<IterationOutputs>()?;
    
    m.add_function(wrap_pyfunction!(app_info, m)?)?;
    m.add_function(wrap_pyfunction!(symbol_properties, m)?)?;
    m.add_function(wrap_pyfunction!(group_properties, m)?)?;
    m.add_function(wrap_pyfunction!(iterate, m)?)?;

    Ok(())
}

#[pyfunction]
fn app_info() -> ParameterProperties {
    ParameterProperties::from(vec![("title", "Impact Oscillator"),("version", env!("CARGO_PKG_VERSION")), ("description", env!("CARGO_PKG_DESCRIPTION"))])
}

#[pyfunction]
fn symbol_properties() -> ParameterProperties {
    ParameterProperties::from(vec![("frequency", "ω"),("offset", "σ"), ("phi", "φ")])
}

#[pyfunction]
fn group_properties() -> ParameterProperties {
    ParameterProperties::from(vec![("frequency", "System parameters"), ("offset", "System parameters"), ("r", "System parameters"), ("phi", "Initial impact"), ("v", "Initial impact"), ("maxPeriods", "Control parameters"), ("numIterations", "Control parameters"), ("numPoints", "Control parameters")])
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
        let pair = &*slf;
        let gil = Python::acquire_gil();
        let py = gil.python();
        let vals = vec![(String::from("Parameter"), pair.parameter.clone()), (String::from("Property"), pair.property.clone())];
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
        self.properties.entry(String::from(parameter)).or_insert(PropertyPair::from((parameter, property)));
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

use crate::dynamics::parameters::Parameters as Parameters;
use crate::dynamics::model_types::ParameterError as ParameterError;
use crate::dynamics::model_types::Phase as Phase;
use crate::dynamics::model_types::Velocity as Velocity;
use crate::dynamics::impact::SimpleImpact as SimpleImpact;
use crate::dynamics::impact_map::IterationResult as IterationResult;
use crate::dynamics::impact_map::ImpactMap as ImpactMap;

#[pyclass]
#[derive(Clone, Default, Debug)]
pub struct IterationInputs {
    frequency: f64,
    offset: f64,
    r: f64,
    max_periods: u32,
    phi: f64,
    v: f64,
    num_iterations: u32
}

#[pymethods]
impl IterationInputs {
    #[new]
    fn new(frequency: f64,
        offset: f64,
        r: f64,
        max_periods: u32,
        phi: f64,
        v: f64,
        num_iterations: u32) -> PyResult<Self>
    {
        Ok(IterationInputs
        {
            frequency: frequency,
            offset: offset,
            r: r,
            max_periods: max_periods,
            phi: phi,
            v: v,
            num_iterations: num_iterations
        })
    }

}

impl IterationInputs {
    fn get_parameters(&self) -> Result<Parameters, Vec<ParameterError>> {
        Parameters::new(self.frequency, self.offset, self.r, self.max_periods)
    }

    fn mapper(&self) -> Result<ImpactMap, Vec<ParameterError>> {
        match self.get_parameters() {
            Err(errors) => Err(errors),

            Ok(params) => Ok(ImpactMap::new(params))
        }
    }

    pub fn iterate(&self)-> Result<IterationResult, Vec<ParameterError>> {
        debug!("Calling iterate() on {:?}", self);
        let result = self.mapper()?.iterate_from_point(self.phi, self.v, self.num_iterations);

        Ok(result)
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct PyImpact {
	phase: Phase,
	velocity: Velocity
}

#[pymethods]
impl PyImpact {
    #[new]
    fn new() -> PyResult<Self>
    {
        Ok(PyImpact
        {
            phase: 0.0,
            velocity: 0.0,
        })
    }

}

impl From<SimpleImpact> for PyImpact {
    fn from(impact: SimpleImpact) -> PyImpact {
        PyImpact {
            phase: 0.0,
            velocity: 0.0,
        }
    }
}

#[pyproto]
impl PyIterProtocol for PyImpact {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PyObject> {
        let impact = &*slf;
        let gil = Python::acquire_gil();
        let py = gil.python();
        let vals = vec![(String::from("phase"), impact.phase), (String::from("velocity"), impact.velocity)];
        let iter = IntoPy::into_py(
            Py::new(py, PyPyImpactIter::new(vals))?,
            py,
        );

        Ok(iter)
    }
}

#[pyclass(name = "PyImpactIter")]
pub struct PyPyImpactIter {
    v: std::vec::IntoIter<(String, f64)>,
}

impl PyPyImpactIter {
    pub fn new(v: Vec<(String, f64)>) -> Self {
        PyPyImpactIter { v: v.into_iter() }
    }
}

#[pyproto]
impl PyIterProtocol for PyPyImpactIter {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyPyImpactIter>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<(String, f64)>> {
        let slf = &mut *slf;
        Ok(slf.v.next())
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct IterationOutputs {
	impacts: Vec<SimpleImpact>,

	long_excursions: bool
}

#[pymethods]
impl IterationOutputs {
    #[new]
    fn new() -> PyResult<Self>
    {
        Ok(IterationOutputs
        {
            impacts: vec![],
            long_excursions: false
        })
    }

}

impl From<&IterationResult> for IterationOutputs {
    fn from(result: &IterationResult) -> IterationOutputs {
        IterationOutputs {
            long_excursions: result.has_long_excursions(),
            impacts: result.trajectory().iter().map(|&impact| impact.get_simple_impact()).collect()
        }
    }
}

fn make_idx_usable (idx: isize, size: usize) -> usize {
    if idx < 0 {
        return make_idx_usable(idx + size as isize, size)
    }

    idx as usize
}

#[pyproto]
impl PySequenceProtocol for IterationOutputs {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.impacts.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<PyImpact> {

        let usable_idx = |idx: isize| -> usize {make_idx_usable(idx, self.impacts.len())};

        let idx_to_use = usable_idx(idx);

        if idx_to_use >= self.impacts.len() {
            use pyo3::exceptions::*;
            return Err(PyIndexError::new_err("Invalid index"));
        }

        Ok(PyImpact::from(self.impacts[idx_to_use]))
    }
}

#[pyclass(name = "IterationOutputsIter")]
pub struct PyIterationOutputsIter {
    v: std::vec::IntoIter<PyImpact>,
}

impl PyIterationOutputsIter {
    pub fn new(v: Vec<PyImpact>) -> Self {
        PyIterationOutputsIter { v: v.into_iter() }
    }
}

#[pyproto]
impl PyIterProtocol for PyIterationOutputsIter {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyIterationOutputsIter>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyImpact>> {
        let slf = &mut *slf;
        Ok(slf.v.next())
    }
}


#[pyfunction]
fn iterate(inputs: IterationInputs) -> IterationOutputs {
    let result = inputs.iterate().unwrap();

    IterationOutputs::from(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_access_parameter_properties() {
        let properties = ParameterProperties::from(vec![("frequency", "ω")]);
        assert_eq!(properties.property("frequency").property, String::from("ω"));
        assert_eq!(properties.property("period").property, String::from(""));
    }
}
