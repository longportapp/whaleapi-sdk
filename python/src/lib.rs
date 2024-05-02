mod config;
mod decimal;
mod error;
mod http_client;
mod time;
mod trade;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn longportwhale(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let openapi = PyModule::new(py, "openapi")?;

    openapi.add_class::<config::Config>()?;
    openapi.add_class::<types::Language>()?;
    openapi.add_class::<http_client::HttpClient>()?;
    trade::register_types(openapi)?;

    m.add_submodule(openapi)?;
    Ok(())
}
