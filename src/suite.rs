use crate::property::PyProperty;
use crate::status::{PyErrorStatus, PyFailure, PySkipped};
use crate::test_case::PyTestCase;
use crate::utils::{
    extra_to_dict, extract_extra, normalize_timestamp, report_with_timestamp, sanitize_optional,
    sanitize_xml_text, timestamp_to_python,
};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use quick_junit::{TestSuite as QuickTestSuite, XmlString};
use std::time::Duration;

#[pyclass(module = "juwunit._juwunit", name = "TestSuite")]
pub struct PyTestSuite {
    pub(crate) name: String,
    pub(crate) timestamp: Option<String>,
    pub(crate) time: Option<Duration>,
    pub(crate) test_cases: Vec<Py<PyTestCase>>,
    pub(crate) properties: Vec<Py<PyProperty>>,
    pub(crate) system_out: Option<String>,
    pub(crate) system_err: Option<String>,
    pub(crate) extra: Vec<(String, String)>,
}

#[pymethods]
impl PyTestSuite {
    #[new]
    #[pyo3(signature = (
        name,
        *,
        timestamp = None,
        time = None,
        test_cases = Vec::new(),
        properties = Vec::new(),
        system_out = None,
        system_err = None,
        extra = None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        name: String,
        timestamp: Option<&Bound<'_, PyAny>>,
        time: Option<Duration>,
        test_cases: Vec<Py<PyTestCase>>,
        properties: Vec<Py<PyProperty>>,
        system_out: Option<String>,
        system_err: Option<String>,
        extra: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        Ok(Self {
            name: sanitize_xml_text(name),
            timestamp: normalize_timestamp(py, timestamp)?,
            time,
            test_cases,
            properties,
            system_out: sanitize_optional(system_out),
            system_err: sanitize_optional(system_err),
            extra: extract_extra(extra)?,
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
    fn tests(&self) -> usize {
        self.test_cases.len()
    }

    #[getter]
    fn disabled(&self, py: Python<'_>) -> PyResult<usize> {
        Ok(suite_counts(py, self)?.0)
    }

    #[getter]
    fn failures(&self, py: Python<'_>) -> PyResult<usize> {
        Ok(suite_counts(py, self)?.1)
    }

    #[getter]
    fn errors(&self, py: Python<'_>) -> PyResult<usize> {
        Ok(suite_counts(py, self)?.2)
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
    fn test_cases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            self.test_cases
                .iter()
                .map(|test_case| test_case.clone_ref(py)),
        )
    }

    fn add_test_case(&mut self, test_case: Py<PyTestCase>) {
        self.test_cases.push(test_case);
    }

    fn add_test_cases(&mut self, test_cases: Vec<Py<PyTestCase>>) {
        self.test_cases.extend(test_cases);
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
}

impl PyTestSuite {
    pub(crate) fn to_quick(&self, py: Python<'_>) -> PyResult<QuickTestSuite> {
        let mut suite = QuickTestSuite::new(self.name.clone());
        if let Some(timestamp) = &self.timestamp {
            suite.timestamp = report_with_timestamp(timestamp)?.timestamp;
        }
        suite.time = self.time;
        for property in &self.properties {
            suite.add_property(property.bind(py).borrow().to_quick());
        }
        if let Some(system_out) = &self.system_out {
            suite.set_system_out(system_out.clone());
        }
        if let Some(system_err) = &self.system_err {
            suite.set_system_err(system_err.clone());
        }
        for (key, value) in &self.extra {
            suite.extra.insert(key.clone().into(), value.clone().into());
        }
        for test_case in &self.test_cases {
            suite.add_test_case(test_case.bind(py).borrow().to_quick(py)?);
        }
        Ok(suite)
    }

    pub(crate) fn from_quick(py: Python<'_>, suite: QuickTestSuite) -> PyResult<Self> {
        let test_cases = suite
            .test_cases
            .into_iter()
            .map(|test_case| Py::new(py, PyTestCase::from_quick(py, test_case)?))
            .collect::<PyResult<Vec<_>>>()?;
        let properties = suite
            .properties
            .into_iter()
            .map(|property| Py::new(py, PyProperty::from_quick(property)))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            name: suite.name.into_string(),
            timestamp: suite
                .timestamp
                .as_ref()
                .map(|timestamp| timestamp.to_rfc3339()),
            time: suite.time,
            test_cases,
            properties,
            system_out: suite.system_out.map(XmlString::into_string),
            system_err: suite.system_err.map(XmlString::into_string),
            extra: suite
                .extra
                .into_iter()
                .map(|(key, value)| (key.into_string(), value.into_string()))
                .collect(),
        })
    }
}

pub(crate) fn suite_counts(py: Python<'_>, suite: &PyTestSuite) -> PyResult<(usize, usize, usize)> {
    let mut disabled = 0;
    let mut failures = 0;
    let mut errors = 0;
    for test_case in &suite.test_cases {
        let test_case = test_case.bind(py).borrow();
        let status = test_case.status.bind(py);
        if status.is_instance_of::<PySkipped>() {
            disabled += 1;
        } else if status.is_instance_of::<PyFailure>() {
            failures += 1;
        } else if status.is_instance_of::<PyErrorStatus>() {
            errors += 1;
        }
    }
    Ok((disabled, failures, errors))
}
