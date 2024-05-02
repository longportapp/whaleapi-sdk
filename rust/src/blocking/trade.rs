use std::sync::Arc;

use crate::{
    blocking::runtime::BlockingRuntime,
    trade::{
        PushEvent, TopicType, TradeContext,
    },
    Config, Result,
};

/// Trade context
pub struct TradeContextSync {
    rt: BlockingRuntime<TradeContext>,
}

impl TradeContextSync {
    /// Create a `TradeContextSync` object
    pub fn try_new<F>(config: Arc<Config>, push_callback: F) -> Result<Self>
    where
        F: FnMut(PushEvent) + Send + 'static,
    {
        let rt = BlockingRuntime::try_new(move || TradeContext::try_new(config), push_callback)?;
        Ok(Self { rt })
    }

    /// Subscribe topics
    pub fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.subscribe(topics).await })
    }

    /// Unsubscribe topics
    pub fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(topics).await })
    }
}
