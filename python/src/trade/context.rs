use std::sync::Arc;

use longportwhale::blocking::TradeContextSync;
use parking_lot::Mutex;
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};

use crate::{
    config::Config,
    error::ErrorNewType,
    trade::{push::handle_push_event, types::TopicType},
};

#[derive(Debug, Default)]
pub(crate) struct Callbacks {
    pub(crate) order_changed: Option<PyObject>,
}

#[pyclass]
pub(crate) struct TradeContext {
    ctx: TradeContextSync,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[pymethods]
impl TradeContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let ctx = TradeContextSync::try_new(Arc::new(config.0.clone()), {
            let callbacks = callbacks.clone();
            move |event| {
                handle_push_event(&callbacks.lock(), event);
            }
        })
        .map_err(ErrorNewType)?;
        Ok(Self { ctx, callbacks })
    }

    /// Set order changed callback, after receiving the order changed event, it
    /// will call back to this function.
    fn set_on_order_changed(&self, py: Python<'_>, callback: PyObject) {
        if callback.is_none(py) {
            self.callbacks.lock().order_changed = None;
        } else {
            self.callbacks.lock().order_changed = Some(callback);
        }
    }

    /// Subscribe
    fn subscribe(&self, topics: Vec<TopicType>) -> PyResult<()> {
        self.ctx
            .subscribe(topics.into_iter().map(Into::into))
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    fn unsubscribe(&self, topics: Vec<TopicType>) -> PyResult<()> {
        self.ctx
            .unsubscribe(topics.into_iter().map(Into::into))
            .map_err(ErrorNewType)?;
        Ok(())
    }
}
