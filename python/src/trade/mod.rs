mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<types::TopicType>()?;
    parent.add_class::<types::OrderStatus>()?;
    parent.add_class::<types::OrderSide>()?;
    parent.add_class::<types::OrderType>()?;
    parent.add_class::<types::OrderTag>()?;
    parent.add_class::<types::TriggerStatus>()?;
    parent.add_class::<types::PushOrderChanged>()?;
    parent.add_class::<context::TradeContext>()?;
    Ok(())
}
