extern crate libc;
use std::convert::From;

use libc::c_char;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;

pub struct ParameterProperties {
    properties: HashMap<String, String>,
}

impl ParameterProperties {
    fn new() -> ParameterProperties
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
        let mut properties = ParameterProperties::new();

        records.into_iter().for_each(|(parameter, property)| {properties.add(parameter, property);});

        properties
    }
}

#[no_mangle]
pub extern "C" fn parameter_properties_new() -> *mut ParameterProperties {
    Box::into_raw(Box::new(ParameterProperties::new()))
}

#[no_mangle]
pub extern "C" fn parameter_properties_free(ptr: *mut ParameterProperties) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

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
