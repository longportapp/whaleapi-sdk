use std::sync::Arc;

use longportwhale_wscli::WsClientError;
use tokio::sync::{mpsc, oneshot};

use crate::{
    trade::{
        core::{Command, Core},
        PushEvent, TopicType,
    },
    Config, Result,
};

/// Trade context
#[derive(Clone)]
pub struct TradeContext {
    command_tx: mpsc::UnboundedSender<Command>,
}

impl TradeContext {
    /// Create a `TradeContext`
    pub async fn try_new(
        config: Arc<Config>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<PushEvent>)> {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (push_tx, push_rx) = mpsc::unbounded_channel();
        tokio::spawn(Core::try_new(config, command_rx, push_tx).await?.run());
        Ok((
            TradeContext {
                command_tx,
            },
            push_rx,
        ))
    }

    /// Subscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/trade-push#subscribe>
    pub async fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Subscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }

    /// Unsubscribe
    ///
    /// Reference: <https://open.longportapp.com/en/docs/trade/trade-push#cancel-subscribe>
    pub async fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType>,
    {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.command_tx
            .send(Command::Unsubscribe {
                topics: topics.into_iter().collect(),
                reply_tx,
            })
            .map_err(|_| WsClientError::ClientClosed)?;
        reply_rx.await.map_err(|_| WsClientError::ClientClosed)?
    }
}
