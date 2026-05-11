use crate::property::PyProperty;
use crate::status::{ensure_status, status_from_quick, status_to_quick};
use crate::utils::{
    extra_to_dict, extract_extra, normalize_timestamp, report_with_timestamp, sanitize_optional,
    sanitize_xml_text, timestamp_to_python,
};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use quick_junit::{TestCase as QuickTestCase, XmlString};
use std::time::Duration;

#[pyclass(module = "juwunit._juwunit", name = "TestCase")]
pub struct PyTestCase {
    pub(crate) name: String,
    pub(crate) classname: Option<String>,
    pub(crate) assertions: Option<usize>,
    pub(crate) timestamp: Option<String>,
    pub(crate) time: Option<Duration>,
    pub(crate) status: Py<PyAny>,
    pub(crate) system_out: Option<String>,
    pub(crate) system_err: Option<String>,
    pub(crate) extra: Vec<(String, String)>,
    pub(crate) properties: Vec<Py<PyProperty>>,
}

#[pymethods]
impl PyTestCase {
    #[new]
    #[pyo3(signature = (
        name,
        status,
        *,
        classname = None,
        assertions = None,
        timestamp = None,
        time = None,
        system_out = None,
        system_err = None,
        extra = None,
        properties = Vec::new()
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        name: String,
        status: Py<PyAny>,
        classname: Option<String>,
        assertions: Option<usize>,
        timestamp: Option<&Bound<'_, PyAny>>,
        time: Option<Duration>,
        system_out: Option<String>,
        system_err: Option<String>,
        extra: Option<&Bound<'_, PyDict>>,
        properties: Vec<Py<PyProperty>>,
    ) -> PyResult<Self> {
        ensure_status(py, &status)?;
        Ok(Self {
            name: sanitize_xml_text(name),
            classname: sanitize_optional(classname),
            assertions,
            timestamp: normalize_timestamp(py, timestamp)?,
            time,
            status,
            system_out: sanitize_optional(system_out),
            system_err: sanitize_optional(system_err),
            extra: extract_extra(extra)?,
            properties,
        })
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
    fn classname(&self) -> Option<&str> {
        self.classname.as_deref()
    }

    #[setter]
    fn set_classname(&mut self, value: Option<String>) {
        self.classname = sanitize_optional(value);
    }

    #[getter]
    fn assertions(&self) -> Option<usize> {
        self.assertions
    }

    #[setter]
    fn set_assertions(&mut self, value: Option<usize>) {
        self.assertions = value;
    }

    #[getter]
    fn timestamp<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        timestamp_to_python(py, self.timestamp.as_deref())
    }

    #[setter]
    fn set_timestamp(&mut self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        self.timestamp = normalize_timestamp(py, value)?;
        Ok(())
    }

    #[getter]
    fn time(&self) -> Option<Duration> {
        self.time
    }

    #[setter]
    fn set_time(&mut self, value: Option<Duration>) {
        self.time = value;
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> Py<PyAny> {
        self.status.clone_ref(py)
    }

    #[setter]
    fn set_status(&mut self, py: Python<'_>, value: Py<PyAny>) -> PyResult<()> {
        ensure_status(py, &value)?;
        self.status = value;
        Ok(())
    }

    #[getter]
    fn system_out(&self) -> Option<&str> {
        self.system_out.as_deref()
    }

    #[setter]
    fn set_system_out(&mut self, value: Option<String>) {
        self.system_out = sanitize_optional(value);
    }

    #[getter]
    fn system_err(&self) -> Option<&str> {
        self.system_err.as_deref()
    }

    #[setter]
    fn set_system_err(&mut self, value: Option<String>) {
        self.system_err = sanitize_optional(value);
    }

    #[getter]
    fn extra<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        extra_to_dict(py, &self.extra)
    }

    #[setter]
    fn set_extra(&mut self, value: &Bound<'_, PyDict>) -> PyResult<()> {
        self.extra = extract_extra(Some(value))?;
        Ok(())
    }

    #[getter]
    fn properties<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            self.properties
                .iter()
                .map(|property| property.clone_ref(py)),
        )
    }

    fn add_property(&mut self, property: Py<PyProperty>) {
        self.properties.push(property);
    }

    fn add_properties(&mut self, properties: Vec<Py<PyProperty>>) {
        self.properties.extend(properties);
    }
}

impl PyTestCase {
    pub(crate) fn to_quick(&self, py: Python<'_>) -> PyResult<QuickTestCase> {
        let mut test_case =
            QuickTestCase::new(self.name.clone(), status_to_quick(py, &self.status)?);
        if let Some(classname) = &self.classname {
            test_case.set_classname(classname.clone());
        }
        if let Some(assertions) = self.assertions {
            test_case.set_assertions(assertions);
        }
        if let Some(timestamp) = &self.timestamp {
            test_case.timestamp = report_with_timestamp(timestamp)?.timestamp;
        }
        test_case.time = self.time;
        if let Some(system_out) = &self.system_out {
            test_case.set_system_out(system_out.clone());
        }
        if let Some(system_err) = &self.system_err {
            test_case.set_system_err(system_err.clone());
        }
        for (key, value) in &self.extra {
            test_case
                .extra
                .insert(key.clone().into(), value.clone().into());
        }
        for property in &self.properties {
            test_case.add_property(property.bind(py).borrow().to_quick());
        }
        Ok(test_case)
    }

    pub(crate) fn from_quick(py: Python<'_>, test_case: QuickTestCase) -> PyResult<Self> {
        let status = status_from_quick(py, test_case.status)?;
        let properties = test_case
            .properties
            .into_iter()
            .map(|property| Py::new(py, PyProperty::from_quick(property)))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            name: test_case.name.into_string(),
            classname: test_case.classname.map(XmlString::into_string),
            assertions: test_case.assertions,
            timestamp: test_case
                .timestamp
                .as_ref()
                .map(|timestamp| timestamp.to_rfc3339()),
            time: test_case.time,
            status,
            system_out: test_case.system_out.map(XmlString::into_string),
            system_err: test_case.system_err.map(XmlString::into_string),
            extra: test_case
                .extra
                .into_iter()
                .map(|(key, value)| (key.into_string(), value.into_string()))
                .collect(),
            properties,
        })
    }
}
