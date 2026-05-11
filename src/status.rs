use crate::enums::{PyNonSuccessKind, PyRerunKind};
use crate::test_rerun::PyTestRerun;
use crate::utils::sanitize_optional;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyAnyMethods, PyTuple};
use quick_junit::{
    NonSuccessKind as QuickNonSuccessKind, TestCaseStatus as QuickStatus, XmlString,
};

#[pyclass(module = "juwunit._juwunit", name = "Success")]
pub struct PySuccess {
    pub(crate) flaky_runs: Vec<Py<PyTestRerun>>,
}

#[pymethods]
impl PySuccess {
    #[new]
    #[pyo3(signature = (*, flaky_runs = Vec::new()))]
    fn new(flaky_runs: Vec<Py<PyTestRerun>>) -> Self {
        Self { flaky_runs }
    }

    #[getter]
    fn flaky_runs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.flaky_runs.iter().map(|rerun| rerun.clone_ref(py)))
    }

    fn add_rerun(&mut self, rerun: Py<PyTestRerun>) {
        self.flaky_runs.push(rerun);
    }

    fn add_reruns(&mut self, reruns: Vec<Py<PyTestRerun>>) {
        self.flaky_runs.extend(reruns);
    }
}

#[pyclass(module = "juwunit._juwunit", name = "Failure")]
pub struct PyFailure {
    pub(crate) message: Option<String>,
    pub(crate) ty: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) reruns: Vec<Py<PyTestRerun>>,
    pub(crate) rerun_kind: PyRerunKind,
}

#[pymethods]
impl PyFailure {
    #[new]
    #[pyo3(signature = (
        message = None,
        *,
        r#type = None,
        description = None,
        reruns = Vec::new(),
        rerun_kind = PyRerunKind::Rerun
    ))]
    fn new(
        message: Option<String>,
        r#type: Option<String>,
        description: Option<String>,
        reruns: Vec<Py<PyTestRerun>>,
        rerun_kind: PyRerunKind,
    ) -> Self {
        Self {
            message: sanitize_optional(message),
            ty: sanitize_optional(r#type),
            description: sanitize_optional(description),
            reruns,
            rerun_kind,
        }
    }

    #[getter]
    fn kind(&self) -> PyNonSuccessKind {
        PyNonSuccessKind::Failure
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
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[setter]
    fn set_description(&mut self, value: Option<String>) {
        self.description = sanitize_optional(value);
    }

    #[getter]
    fn rerun_kind(&self) -> PyRerunKind {
        self.rerun_kind
    }

    #[setter]
    fn set_rerun_kind(&mut self, value: PyRerunKind) {
        self.rerun_kind = value;
    }

    #[getter]
    fn reruns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.reruns.iter().map(|rerun| rerun.clone_ref(py)))
    }

    fn add_rerun(&mut self, rerun: Py<PyTestRerun>) {
        self.reruns.push(rerun);
    }

    fn add_reruns(&mut self, reruns: Vec<Py<PyTestRerun>>) {
        self.reruns.extend(reruns);
    }
}

#[pyclass(module = "juwunit._juwunit", name = "Error")]
pub struct PyErrorStatus {
    pub(crate) message: Option<String>,
    pub(crate) ty: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) reruns: Vec<Py<PyTestRerun>>,
    pub(crate) rerun_kind: PyRerunKind,
}

#[pymethods]
impl PyErrorStatus {
    #[new]
    #[pyo3(signature = (
        message = None,
        *,
        r#type = None,
        description = None,
        reruns = Vec::new(),
        rerun_kind = PyRerunKind::Rerun
    ))]
    fn new(
        message: Option<String>,
        r#type: Option<String>,
        description: Option<String>,
        reruns: Vec<Py<PyTestRerun>>,
        rerun_kind: PyRerunKind,
    ) -> Self {
        Self {
            message: sanitize_optional(message),
            ty: sanitize_optional(r#type),
            description: sanitize_optional(description),
            reruns,
            rerun_kind,
        }
    }

    #[getter]
    fn kind(&self) -> PyNonSuccessKind {
        PyNonSuccessKind::Error
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
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[setter]
    fn set_description(&mut self, value: Option<String>) {
        self.description = sanitize_optional(value);
    }

    #[getter]
    fn rerun_kind(&self) -> PyRerunKind {
        self.rerun_kind
    }

    #[setter]
    fn set_rerun_kind(&mut self, value: PyRerunKind) {
        self.rerun_kind = value;
    }

    #[getter]
    fn reruns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.reruns.iter().map(|rerun| rerun.clone_ref(py)))
    }

    fn add_rerun(&mut self, rerun: Py<PyTestRerun>) {
        self.reruns.push(rerun);
    }

    fn add_reruns(&mut self, reruns: Vec<Py<PyTestRerun>>) {
        self.reruns.extend(reruns);
    }
}

#[pyclass(module = "juwunit._juwunit", name = "Skipped")]
pub struct PySkipped {
    pub(crate) message: Option<String>,
    pub(crate) ty: Option<String>,
    pub(crate) description: Option<String>,
}

#[pymethods]
impl PySkipped {
    #[new]
    #[pyo3(signature = (message = None, *, r#type = None, description = None))]
    fn new(message: Option<String>, r#type: Option<String>, description: Option<String>) -> Self {
        Self {
            message: sanitize_optional(message),
            ty: sanitize_optional(r#type),
            description: sanitize_optional(description),
        }
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
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[setter]
    fn set_description(&mut self, value: Option<String>) {
        self.description = sanitize_optional(value);
    }
}

pub(crate) fn ensure_status(py: Python<'_>, status: &Py<PyAny>) -> PyResult<()> {
    let status = status.bind(py);
    if status.is_instance_of::<PySuccess>()
        || status.is_instance_of::<PyFailure>()
        || status.is_instance_of::<PyErrorStatus>()
        || status.is_instance_of::<PySkipped>()
    {
        Ok(())
    } else {
        Err(PyTypeError::new_err(
            "status must be a Success, Failure, Error, or Skipped instance",
        ))
    }
}

pub(crate) fn status_to_quick(py: Python<'_>, status: &Py<PyAny>) -> PyResult<QuickStatus> {
    let status_ref = status.bind(py);
    if status_ref.is_instance_of::<PySuccess>() {
        let success: PyRef<'_, PySuccess> = status_ref.extract()?;
        let mut quick_status = QuickStatus::success();
        for rerun in &success.flaky_runs {
            quick_status.add_rerun(rerun.bind(py).borrow().to_quick()?);
        }
        return Ok(quick_status);
    }
    if status_ref.is_instance_of::<PyFailure>() {
        let failure: PyRef<'_, PyFailure> = status_ref.extract()?;
        return non_success_to_quick(
            py,
            QuickNonSuccessKind::Failure,
            failure.message.as_deref(),
            failure.ty.as_deref(),
            failure.description.as_deref(),
            failure.rerun_kind,
            &failure.reruns,
        );
    }
    if status_ref.is_instance_of::<PyErrorStatus>() {
        let error: PyRef<'_, PyErrorStatus> = status_ref.extract()?;
        return non_success_to_quick(
            py,
            QuickNonSuccessKind::Error,
            error.message.as_deref(),
            error.ty.as_deref(),
            error.description.as_deref(),
            error.rerun_kind,
            &error.reruns,
        );
    }
    if status_ref.is_instance_of::<PySkipped>() {
        let skipped: PyRef<'_, PySkipped> = status_ref.extract()?;
        let mut quick_status = QuickStatus::skipped();
        if let Some(message) = &skipped.message {
            quick_status.set_message(message.clone());
        }
        if let Some(ty) = &skipped.ty {
            quick_status.set_type(ty.clone());
        }
        if let Some(description) = &skipped.description {
            quick_status.set_description(description.clone());
        }
        return Ok(quick_status);
    }
    Err(PyTypeError::new_err(
        "status must be a Success, Failure, Error, or Skipped instance",
    ))
}

fn non_success_to_quick(
    py: Python<'_>,
    kind: QuickNonSuccessKind,
    message: Option<&str>,
    ty: Option<&str>,
    description: Option<&str>,
    rerun_kind: PyRerunKind,
    reruns: &[Py<PyTestRerun>],
) -> PyResult<QuickStatus> {
    let mut quick_status = QuickStatus::non_success(kind);
    if let Some(message) = message {
        quick_status.set_message(message.to_owned());
    }
    if let Some(ty) = ty {
        quick_status.set_type(ty.to_owned());
    }
    if let Some(description) = description {
        quick_status.set_description(description.to_owned());
    }
    quick_status.set_rerun_kind(rerun_kind.into());
    for rerun in reruns {
        quick_status.add_rerun(rerun.bind(py).borrow().to_quick()?);
    }
    Ok(quick_status)
}

pub(crate) fn status_from_quick(py: Python<'_>, status: QuickStatus) -> PyResult<Py<PyAny>> {
    match status {
        QuickStatus::Success { flaky_runs } => Ok(Py::new(
            py,
            PySuccess {
                flaky_runs: flaky_runs
                    .into_iter()
                    .map(|rerun| Py::new(py, PyTestRerun::from_quick(rerun)))
                    .collect::<PyResult<Vec<_>>>()?,
            },
        )?
        .into_any()),
        QuickStatus::NonSuccess {
            kind,
            message,
            ty,
            description,
            reruns,
        } => {
            let reruns_py = reruns
                .runs
                .into_iter()
                .map(|rerun| Py::new(py, PyTestRerun::from_quick(rerun)))
                .collect::<PyResult<Vec<_>>>()?;
            match kind {
                QuickNonSuccessKind::Failure => Ok(Py::new(
                    py,
                    PyFailure {
                        message: message.map(XmlString::into_string),
                        ty: ty.map(XmlString::into_string),
                        description: description.map(XmlString::into_string),
                        reruns: reruns_py,
                        rerun_kind: reruns.kind.into(),
                    },
                )?
                .into_any()),
                QuickNonSuccessKind::Error => Ok(Py::new(
                    py,
                    PyErrorStatus {
                        message: message.map(XmlString::into_string),
                        ty: ty.map(XmlString::into_string),
                        description: description.map(XmlString::into_string),
                        reruns: reruns_py,
                        rerun_kind: reruns.kind.into(),
                    },
                )?
                .into_any()),
            }
        }
        QuickStatus::Skipped {
            message,
            ty,
            description,
        } => Ok(Py::new(
            py,
            PySkipped {
                message: message.map(XmlString::into_string),
                ty: ty.map(XmlString::into_string),
                description: description.map(XmlString::into_string),
            },
        )?
        .into_any()),
    }
}
