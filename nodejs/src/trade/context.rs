use std::sync::Arc;

use longportwhale::trade::PushEvent;
use napi::{threadsafe_function::ThreadsafeFunctionCallMode, JsFunction, Result};
use parking_lot::Mutex;

use crate::{
    config::Config,
    error::ErrorNewType,
    trade::types::{PushOrderChanged, TopicType},
    utils::JsCallback,
};

#[derive(Default)]
struct Callbacks {
    order_changed: Option<JsCallback<PushOrderChanged>>,
}

/// Trade context
#[napi_derive::napi]
#[derive(Clone)]
pub struct TradeContext {
    ctx: longportwhale::trade::TradeContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

#[napi_derive::napi]
impl TradeContext {
    #[napi]
    pub async fn new(config: &Config) -> Result<TradeContext> {
        let callbacks = Arc::new(Mutex::new(Callbacks::default()));
        let (ctx, mut receiver) =
            longportwhale::trade::TradeContext::try_new(Arc::new(config.0.clone()))
                .await
                .map_err(ErrorNewType)?;

        tokio::spawn({
            let callbacks = callbacks.clone();
            async move {
                while let Some(msg) = receiver.recv().await {
                    let callbacks = callbacks.lock();
                    match msg {
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(callback) = &callbacks.order_changed {
                                if let Ok(order_changed) = order_changed.try_into() {
                                    callback.call(
                                        Ok(order_changed),
                                        ThreadsafeFunctionCallMode::Blocking,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(TradeContext { ctx, callbacks })
    }

    /// Set order changed callback, after receiving the order changed event, it
    /// will call back to this function.
    #[napi(ts_args_type = "callback: (err: null | Error, event: PushOrderChanged) => void")]
    pub fn set_on_order_changed(&self, callback: JsFunction) -> Result<()> {
        self.callbacks.lock().order_changed =
            Some(callback.create_threadsafe_function(32, |ctx| Ok(vec![ctx.value]))?);
        Ok(())
    }

    /// Subscribe
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const {
    ///  Config,
    ///  TradeContext,
    ///  Decimal,
    ///  OrderSide,
    ///  TimeInForceType,
    ///  OrderType,
    ///  TopicType,
    /// } = require("longport");
    ///
    /// let config = Config.fromEnv();
    /// let ctx = await TradeContext.new(config)
    /// ctx.setOnQuote((_, event) => console.log(event.toString()));
    /// await ctx.subscribe([TopicType.Private]);
    /// ```
    #[napi]
    pub async fn subscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        self.ctx
            .subscribe(topics.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }

    /// Unsubscribe
    #[napi]
    pub async fn unsubscribe(&self, topics: Vec<TopicType>) -> Result<()> {
        self.ctx
            .unsubscribe(topics.into_iter().map(Into::into))
            .await
            .map_err(ErrorNewType)?;
        Ok(())
    }
}
