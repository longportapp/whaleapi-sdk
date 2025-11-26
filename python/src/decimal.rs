use std::fmt::{self, Debug};

use once_cell::sync::Lazy;
use pyo3::{exceptions::PyBaseException, prelude::*, types::PyType};
use rust_decimal::Decimal;

static DECIMAL_TYPE: Lazy<Py<PyAny>> = Lazy::new(|| {
    Python::attach(|py| {
        let decimal_module = py.import("decimal")?;
        let decimal_type = decimal_module.getattr("Decimal")?;
        Ok::<_, PyErr>(decimal_type.unbind())
    })
    .expect("import decimal")
});

#[derive(Copy, Clone)]
pub(crate) struct PyDecimal(Decimal);

impl Debug for PyDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Decimal> for PyDecimal {
    #[inline]
    fn from(value: Decimal) -> Self {
        PyDecimal(value)
    }
}

impl From<PyDecimal> for Decimal {
    #[inline]
    fn from(value: PyDecimal) -> Self {
        value.0
    }
}

impl<'py> FromPyObject<'_, 'py> for PyDecimal {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = obj.extract::<f64>() {
            // convert from PyFloat
            Ok(Self(Decimal::try_from(value).map_err(|err| {
                PyBaseException::new_err(format!("cannot create decimal value: {err}"))
            })?))
        } else if let Ok(value) = obj.extract::<i64>() {
            // convert from PyInt/PyLong
            Ok(Self(Decimal::from(value)))
        } else {
            // convert from decimal.Decimal
            Python::attach(|py| {
                let decimal_type = DECIMAL_TYPE.cast_bound::<PyType>(py).expect("decimal type");
                if obj.is_instance(decimal_type)? {
                    let str_obj = obj.str()?;
                    let value = str_obj.to_str().expect("decimal str");
                    Ok(Self(value.parse().expect("valid decimal")))
                } else {
                    Err(PyBaseException::new_err("expected a decimal"))
                }
            })
        }
    }
}

impl<'py> IntoPyObject<'py> for PyDecimal {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(DECIMAL_TYPE
            .call1(py, (self.0.to_string(),))?
            .into_bound(py))
    }
}
