use crate::utils::sanitize_xml_text;
use pyo3::prelude::*;
use quick_junit::Property as QuickProperty;

#[pyclass(module = "juwunit._juwunit", name = "Property")]
pub struct PyProperty {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[pymethods]
impl PyProperty {
    #[new]
    fn new(name: String, value: String) -> Self {
        Self {
            name: sanitize_xml_text(name),
            value: sanitize_xml_text(value),
        }
    }

    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[setter]
    fn set_name(&mut self, value: String) {
        self.name = sanitize_xml_text(value);
    }

    #[getter]
    fn value(&self) -> &str {
        &self.value
    }

    #[setter]
    fn set_value(&mut self, value: String) {
        self.value = sanitize_xml_text(value);
    }
}

impl PyProperty {
    pub(crate) fn to_quick(&self) -> QuickProperty {
        QuickProperty::new(self.name.clone(), self.value.clone())
    }

    pub(crate) fn from_quick(property: QuickProperty) -> Self {
        Self {
            name: property.name.into_string(),
            value: property.value.into_string(),
        }
    }
}
