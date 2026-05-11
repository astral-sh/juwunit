use pyo3::prelude::*;
use quick_junit::{FlakyOrRerun as QuickRerunKind, NonSuccessKind as QuickNonSuccessKind};

#[pyclass(
    module = "juwunit._juwunit",
    name = "NonSuccessKind",
    eq,
    eq_int,
    rename_all = "SCREAMING_SNAKE_CASE",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyNonSuccessKind {
    Failure,
    Error,
}

impl From<PyNonSuccessKind> for QuickNonSuccessKind {
    fn from(value: PyNonSuccessKind) -> Self {
        match value {
            PyNonSuccessKind::Failure => QuickNonSuccessKind::Failure,
            PyNonSuccessKind::Error => QuickNonSuccessKind::Error,
        }
    }
}

impl From<QuickNonSuccessKind> for PyNonSuccessKind {
    fn from(value: QuickNonSuccessKind) -> Self {
        match value {
            QuickNonSuccessKind::Failure => PyNonSuccessKind::Failure,
            QuickNonSuccessKind::Error => PyNonSuccessKind::Error,
        }
    }
}

#[pyclass(
    module = "juwunit._juwunit",
    name = "RerunKind",
    eq,
    eq_int,
    rename_all = "SCREAMING_SNAKE_CASE",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyRerunKind {
    Flaky,
    Rerun,
}

impl From<PyRerunKind> for QuickRerunKind {
    fn from(value: PyRerunKind) -> Self {
        match value {
            PyRerunKind::Flaky => QuickRerunKind::Flaky,
            PyRerunKind::Rerun => QuickRerunKind::Rerun,
        }
    }
}

impl From<QuickRerunKind> for PyRerunKind {
    fn from(value: QuickRerunKind) -> Self {
        match value {
            QuickRerunKind::Flaky => PyRerunKind::Flaky,
            QuickRerunKind::Rerun => PyRerunKind::Rerun,
        }
    }
}
