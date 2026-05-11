use crate::errors::{DeserializationError, SerializationError};
use crate::suite::{PyTestSuite, suite_counts};
use crate::utils::{
    extract_xml, normalize_timestamp, normalize_uuid, report_with_timestamp, sanitize_xml_text,
    timestamp_to_python, uuid_to_python,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyAnyMethods, PyTuple, PyType};
use quick_junit::{Report as QuickReport, ReportUuid};
use std::time::Duration;

#[pyclass(module = "juwunit._juwunit", name = "Report")]
pub struct PyReport {
    pub(crate) name: String,
    pub(crate) uuid: Option<String>,
    pub(crate) timestamp: Option<String>,
    pub(crate) time: Option<Duration>,
    pub(crate) test_suites: Vec<Py<PyTestSuite>>,
}

#[pymethods]
impl PyReport {
    #[new]
    #[pyo3(signature = (name, *, uuid = None, timestamp = None, time = None))]
    fn new(
        py: Python<'_>,
        name: String,
        uuid: Option<&Bound<'_, PyAny>>,
        timestamp: Option<&Bound<'_, PyAny>>,
        time: Option<Duration>,
    ) -> PyResult<Self> {
        Ok(Self {
            name: sanitize_xml_text(name),
            uuid: normalize_uuid(py, uuid)?,
            timestamp: normalize_timestamp(py, timestamp)?,
            time,
            test_suites: Vec::new(),
        })
    }

    #[classmethod]
    fn from_xml(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        xml: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let xml = extract_xml(xml)?;
        let report = QuickReport::deserialize_from_str(&xml)
            .map_err(|error| DeserializationError::new_err(error.to_string()))?;
        Self::from_quick(py, report)
    }

    #[classmethod]
    fn read_xml(
        cls: &Bound<'_, PyType>,
        py: Python<'_>,
        reader: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let xml = reader.call_method0("read")?;
        Self::from_xml(cls, py, &xml)
    }

    fn to_xml(&self, py: Python<'_>) -> PyResult<String> {
        self.to_quick(py)?
            .to_string()
            .map_err(|error| SerializationError::new_err(error.to_string()))
    }

    fn write_xml(&self, py: Python<'_>, writer: &Bound<'_, PyAny>) -> PyResult<()> {
        writer.call_method1("write", (self.to_xml(py)?,))?;
        Ok(())
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
    fn uuid<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        uuid_to_python(py, self.uuid.as_deref())
    }

    #[setter]
    fn set_uuid(&mut self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        self.uuid = normalize_uuid(py, value)?;
        Ok(())
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
    fn tests(&self, py: Python<'_>) -> PyResult<usize> {
        let mut total = 0;
        for suite in &self.test_suites {
            total += suite.bind(py).borrow().test_cases.len();
        }
        Ok(total)
    }

    #[getter]
    fn failures(&self, py: Python<'_>) -> PyResult<usize> {
        let mut total = 0;
        for suite in &self.test_suites {
            total += suite_counts(py, &suite.bind(py).borrow())?.1;
        }
        Ok(total)
    }

    #[getter]
    fn errors(&self, py: Python<'_>) -> PyResult<usize> {
        let mut total = 0;
        for suite in &self.test_suites {
            total += suite_counts(py, &suite.bind(py).borrow())?.2;
        }
        Ok(total)
    }

    #[getter]
    fn test_suites<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            self.test_suites
                .iter()
                .map(|test_suite| test_suite.clone_ref(py)),
        )
    }

    fn add_test_suite(&mut self, test_suite: Py<PyTestSuite>) {
        self.test_suites.push(test_suite);
    }

    fn add_test_suites(&mut self, test_suites: Vec<Py<PyTestSuite>>) {
        self.test_suites.extend(test_suites);
    }
}

impl PyReport {
    fn to_quick(&self, py: Python<'_>) -> PyResult<QuickReport> {
        let mut report = QuickReport::new(self.name.clone());
        if let Some(uuid) = &self.uuid {
            let uuid = uuid
                .parse::<ReportUuid>()
                .map_err(|error| PyValueError::new_err(error.to_string()))?;
            report.uuid = Some(uuid);
        }
        if let Some(timestamp) = &self.timestamp {
            report.timestamp = report_with_timestamp(timestamp)?.timestamp;
        }
        report.time = self.time;
        for suite in &self.test_suites {
            report.add_test_suite(suite.bind(py).borrow().to_quick(py)?);
        }
        Ok(report)
    }

    fn from_quick(py: Python<'_>, report: QuickReport) -> PyResult<Self> {
        let test_suites = report
            .test_suites
            .into_iter()
            .map(|suite| Py::new(py, PyTestSuite::from_quick(py, suite)?))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            name: report.name.into_string(),
            uuid: report.uuid.map(|uuid| uuid.to_string()),
            timestamp: report
                .timestamp
                .as_ref()
                .map(|timestamp| timestamp.to_rfc3339()),
            time: report.time,
            test_suites,
        })
    }
}
