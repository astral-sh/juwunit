use crate::enums::PyNonSuccessKind;
use crate::utils::{
    normalize_timestamp, report_with_timestamp, sanitize_optional, timestamp_to_python,
};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use quick_junit::{TestRerun as QuickTestRerun, XmlString};
use std::time::Duration;

#[pyclass(module = "juwunit._juwunit", name = "TestRerun")]
pub struct PyTestRerun {
    pub(crate) kind: PyNonSuccessKind,
    pub(crate) timestamp: Option<String>,
    pub(crate) time: Option<Duration>,
    pub(crate) message: Option<String>,
    pub(crate) ty: Option<String>,
    pub(crate) stack_trace: Option<String>,
    pub(crate) system_out: Option<String>,
    pub(crate) system_err: Option<String>,
    pub(crate) description: Option<String>,
}

#[pymethods]
impl PyTestRerun {
    #[new]
    #[pyo3(signature = (
        kind,
        *,
        timestamp = None,
        time = None,
        message = None,
        r#type = None,
        stack_trace = None,
        system_out = None,
        system_err = None,
        description = None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        kind: PyNonSuccessKind,
        timestamp: Option<&Bound<'_, PyAny>>,
        time: Option<Duration>,
        message: Option<String>,
        r#type: Option<String>,
        stack_trace: Option<String>,
        system_out: Option<String>,
        system_err: Option<String>,
        description: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            kind,
            timestamp: normalize_timestamp(py, timestamp)?,
            time,
            message: sanitize_optional(message),
            ty: sanitize_optional(r#type),
            stack_trace: sanitize_optional(stack_trace),
            system_out: sanitize_optional(system_out),
            system_err: sanitize_optional(system_err),
            description: sanitize_optional(description),
        })
    }

    #[getter]
    fn kind(&self) -> PyNonSuccessKind {
        self.kind
    }

    #[setter]
    fn set_kind(&mut self, value: PyNonSuccessKind) {
        self.kind = value;
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
    fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    #[setter]
    fn set_message(&mut self, value: Option<String>) {
        self.message = sanitize_optional(value);
    }

    #[getter(r#type)]
    fn ty(&self) -> Option<&str> {
        self.ty.as_deref()
    }

    #[setter(r#type)]
    fn set_ty(&mut self, value: Option<String>) {
        self.ty = sanitize_optional(value);
    }

    #[getter]
    fn stack_trace(&self) -> Option<&str> {
        self.stack_trace.as_deref()
    }

    #[setter]
    fn set_stack_trace(&mut self, value: Option<String>) {
        self.stack_trace = sanitize_optional(value);
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
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[setter]
    fn set_description(&mut self, value: Option<String>) {
        self.description = sanitize_optional(value);
    }
}

impl PyTestRerun {
    pub(crate) fn to_quick(&self) -> PyResult<QuickTestRerun> {
        let mut rerun = QuickTestRerun::new(self.kind.into());
        if let Some(timestamp) = &self.timestamp {
            rerun.timestamp = report_with_timestamp(timestamp)?.timestamp;
        }
        rerun.time = self.time;
        if let Some(message) = &self.message {
            rerun.set_message(message.clone());
        }
        if let Some(ty) = &self.ty {
            rerun.set_type(ty.clone());
        }
        if let Some(stack_trace) = &self.stack_trace {
            rerun.set_stack_trace(stack_trace.clone());
        }
        if let Some(system_out) = &self.system_out {
            rerun.set_system_out(system_out.clone());
        }
        if let Some(system_err) = &self.system_err {
            rerun.set_system_err(system_err.clone());
        }
        if let Some(description) = &self.description {
            rerun.set_description(description.clone());
        }
        Ok(rerun)
    }

    pub(crate) fn from_quick(rerun: QuickTestRerun) -> Self {
        Self {
            kind: rerun.kind.into(),
            timestamp: rerun
                .timestamp
                .as_ref()
                .map(|timestamp| timestamp.to_rfc3339()),
            time: rerun.time,
            message: rerun.message.map(XmlString::into_string),
            ty: rerun.ty.map(XmlString::into_string),
            stack_trace: rerun.stack_trace.map(XmlString::into_string),
            system_out: rerun.system_out.map(XmlString::into_string),
            system_err: rerun.system_err.map(XmlString::into_string),
            description: rerun.description.map(XmlString::into_string),
        }
    }
}
