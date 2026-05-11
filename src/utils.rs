use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyAnyMethods, PyDict, PyDictMethods};
use quick_junit::{Report as QuickReport, XmlString};

pub(crate) fn sanitize_xml_text(value: String) -> String {
    XmlString::new(value).into_string()
}

pub(crate) fn sanitize_optional(value: Option<String>) -> Option<String> {
    value.map(sanitize_xml_text)
}

pub(crate) fn extract_extra(extra: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<(String, String)>> {
    let mut values = Vec::new();
    if let Some(extra) = extra {
        for (key, value) in extra.iter() {
            values.push((
                sanitize_xml_text(key.extract()?),
                sanitize_xml_text(value.extract()?),
            ));
        }
    }
    Ok(values)
}

pub(crate) fn extra_to_dict<'py>(
    py: Python<'py>,
    extra: &[(String, String)],
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (key, value) in extra {
        dict.set_item(key, value)?;
    }
    Ok(dict)
}

pub(crate) fn normalize_uuid(
    py: Python<'_>,
    value: Option<&Bound<'_, PyAny>>,
) -> PyResult<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let uuid_cls = py.import("uuid")?.getattr("UUID")?;
    if !value.is_instance(&uuid_cls)? {
        return Err(PyTypeError::new_err("uuid must be a uuid.UUID instance"));
    }
    Ok(Some(value.str()?.to_str()?.to_owned()))
}

pub(crate) fn uuid_to_python<'py>(
    py: Python<'py>,
    value: Option<&str>,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    let Some(value) = value else {
        return Ok(None);
    };
    Ok(Some(py.import("uuid")?.getattr("UUID")?.call1((value,))?))
}

pub(crate) fn normalize_timestamp(
    py: Python<'_>,
    value: Option<&Bound<'_, PyAny>>,
) -> PyResult<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let datetime_cls = py.import("datetime")?.getattr("datetime")?;
    if !value.is_instance(&datetime_cls)? {
        return Err(PyTypeError::new_err(
            "timestamp must be a datetime.datetime instance",
        ));
    }
    if value.call_method0("utcoffset")?.is_none() {
        return Err(PyValueError::new_err("timestamp must be timezone-aware"));
    }
    Ok(Some(value.call_method0("isoformat")?.extract()?))
}

pub(crate) fn timestamp_to_python<'py>(
    py: Python<'py>,
    value: Option<&str>,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    let Some(value) = value else {
        return Ok(None);
    };
    Ok(Some(
        py.import("datetime")?
            .getattr("datetime")?
            .call_method1("fromisoformat", (value,))?,
    ))
}

pub(crate) fn report_with_timestamp(timestamp: &str) -> PyResult<QuickReport> {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="timestamp" tests="0" failures="0" errors="0" timestamp="{timestamp}"/>
"#
    );
    QuickReport::deserialize_from_str(&xml)
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

pub(crate) fn extract_xml(xml: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(xml) = xml.extract::<String>() {
        return Ok(xml);
    }
    if let Ok(xml) = xml.extract::<Vec<u8>>() {
        return String::from_utf8(xml)
            .map_err(|_| PyValueError::new_err("XML bytes must be valid UTF-8"));
    }
    Err(PyTypeError::new_err("XML input must be str or bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_text_removes_invalid_xml_characters() {
        assert_eq!(
            sanitize_xml_text("ok\x1b[31mred\x00done".to_owned()),
            "okreddone"
        );
    }

    #[test]
    fn parse_timestamp_via_quick_junit() {
        let report = report_with_timestamp("2026-05-08T20:53:15.186+00:00").unwrap();
        assert!(report.timestamp.is_some());
    }
}
