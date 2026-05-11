#![forbid(unsafe_code)]

mod enums;
mod errors;
mod property;
mod report;
mod status;
mod suite;
mod test_case;
mod test_rerun;
mod utils;

use enums::{PyNonSuccessKind, PyRerunKind};
use errors::{DeserializationError, JuwunitError, SerializationError};
use property::PyProperty;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use report::PyReport;
use status::{PyErrorStatus, PyFailure, PySkipped, PySuccess};
use suite::PyTestSuite;
use test_case::PyTestCase;
use test_rerun::PyTestRerun;

#[pymodule]
fn _juwunit(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyNonSuccessKind>()?;
    module.add_class::<PyRerunKind>()?;
    module.add_class::<PyProperty>()?;
    module.add_class::<PyTestRerun>()?;
    module.add_class::<PySuccess>()?;
    module.add_class::<PyFailure>()?;
    module.add_class::<PyErrorStatus>()?;
    module.add_class::<PySkipped>()?;
    module.add_class::<PyTestCase>()?;
    module.add_class::<PyTestSuite>()?;
    module.add_class::<PyReport>()?;
    module.add("JuwunitError", py.get_type::<JuwunitError>())?;
    module.add(
        "DeserializationError",
        py.get_type::<DeserializationError>(),
    )?;
    module.add("SerializationError", py.get_type::<SerializationError>())?;
    Ok(())
}
