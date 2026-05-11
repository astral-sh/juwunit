use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(_juwunit, JuwunitError, PyException);
create_exception!(_juwunit, DeserializationError, JuwunitError);
create_exception!(_juwunit, SerializationError, JuwunitError);
