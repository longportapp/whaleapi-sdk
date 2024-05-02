use napi::Result;

use crate::{error::ErrorNewType, types::Language};

/// Configuration parameters
#[napi_derive::napi(object)]
pub struct ConfigParams {
    /// App Key
    pub app_key: String,
    /// App Secret
    pub app_secret: String,
    /// Access Token
    pub access_token: String,
    /// HTTP API url
    /// test env is https://openapi.longbridge.xyz
    pub http_url: Option<String>,
    /// Websocket url for trade API (default:
    /// "wss://openapi-trade.longportapp.com")
    pub trade_ws_url: Option<String>,
    /// Language identifier (default: Language.EN)
    pub language: Option<Language>,
}

/// Configuration for LongPort sdk
#[napi_derive::napi]
pub struct Config(pub(crate) longportwhale::Config);

#[napi_derive::napi]
impl Config {
    /// Create a new `Config`
    #[napi(constructor)]
    pub fn new(params: ConfigParams) -> Self {
        let mut config =
            longportwhale::Config::new(params.app_key, params.app_secret, params.access_token);

        if let Some(http_url) = params.http_url {
            config = config.http_url(http_url);
        }

        if let Some(trade_ws_url) = params.trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }

        if let Some(language) = params.language {
            config = config.language(language.into());
        }

        Self(config)
    }

    /// Create a new `Config` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGPORT_APP_KEY` - App key
    /// - `LONGPORT_APP_SECRET` - App secret
    /// - `LONGPORT_ACCESS_TOKEN` - Access token
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url
    /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url
    #[napi(factory)]
    pub fn from_env() -> Result<Self> {
        Ok(Self(longportwhale::Config::from_env().map_err(ErrorNewType)?))
    }
}
