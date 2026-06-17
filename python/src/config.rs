use pyo3::{prelude::*, types::PyType};

use crate::{error::ErrorNewType, types::Language};

#[pyclass(name = "Config")]
pub(crate) struct Config(pub(crate) longportwhale::Config);

#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (
        app_key,
        app_secret,
        access_token,
        http_url = None,
        trade_ws_url = None,
        language = None,
    ))]
    fn py_new(
        app_key: String,
        app_secret: String,
        access_token: String,
        http_url: Option<String>,
        trade_ws_url: Option<String>,
        language: Option<Language>,
    ) -> Self {
        let mut config = longportwhale::Config::new(app_key, app_secret, access_token);
        if let Some(http_url) = http_url {
            config = config.http_url(http_url);
        }
        if let Some(trade_ws_url) = trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }
        if let Some(language) = language {
            config = config.language(language.into());
        }
        Self(config)
    }

    #[classmethod]
    fn from_env(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Ok(Self(
            longportwhale::Config::from_env().map_err(ErrorNewType)?,
        ))
    }
}
