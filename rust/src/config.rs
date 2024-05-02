pub(crate) use http::{header, HeaderValue, Request};
use longportwhale_httpcli::{HttpClient, HttpClientConfig};
use num_enum::IntoPrimitive;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::error::Result;

const TRADE_WS_URL: &str = "wss://openapi-trade.longportapp.com";

/// Language identifier
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum Language {
    /// zh-CN
    ZH_CN = 0,
    /// zh-HK
    ZH_HK = 2,
    /// en
    EN = 1,
}

impl Language {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Language::ZH_CN => "zh-CN",
            Language::ZH_HK => "zh-HK",
            Language::EN => "en",
        }
    }
}

/// Configuration options for LongPort sdk
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) http_cli_config: HttpClientConfig,
    pub(crate) trade_ws_url: String,
    pub(crate) language: Language,
}

impl Config {
    /// Create a new `Config`
    pub fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        access_token: impl Into<String>,
    ) -> Self {
        Self {
            http_cli_config: HttpClientConfig::new(app_key, app_secret, access_token),
            trade_ws_url: TRADE_WS_URL.to_string(),
            language: Language::EN,
        }
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
    /// - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longportapp.com`)
    /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
    ///   `wss://openapi-trade.longportapp.com`)
    pub fn from_env() -> Result<Self> {
        let _ = dotenv::dotenv();

        let http_cli_config = HttpClientConfig::from_env()?;
        let trade_ws_url = std::env::var("LONGPORT_TRADE_WS_URL")
            .unwrap_or_else(|_| {
                TRADE_WS_URL.to_string()
            });

        Ok(Config {
            http_cli_config,
            trade_ws_url,
            language: Language::EN,
        })
    }

    /// Specifies the url of the OpenAPI server.
    ///
    /// Default: `https://openapi.longportapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn http_url(mut self, url: impl Into<String>) -> Self {
        self.http_cli_config = self.http_cli_config.http_url(url);
        self
    }

    /// Specifies the url of the OpenAPI trade websocket server.
    ///
    /// Default: `wss://openapi-trade.longportapp.com`
    ///
    /// NOTE: Usually you don't need to change it.
    #[must_use]
    pub fn trade_ws_url(self, url: impl Into<String>) -> Self {
        Self {
            trade_ws_url: url.into(),
            ..self
        }
    }

    /// Specifies the language
    ///
    /// Default: `Language::EN`
    pub fn language(self, language: Language) -> Self {
        Self { language, ..self }
    }

    /// Create http client use the http client config
    pub fn create_http_client(&self) -> HttpClient {
        HttpClient::new(self.http_cli_config.clone())
            .header(header::ACCEPT_LANGUAGE, self.language.as_str())
    }

    fn create_ws_request(&self, url: &str) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        let mut request = url.into_client_request()?;
        request.headers_mut().append(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_str(self.language.as_str()).unwrap(),
        );
        Ok(request)
    }

    #[inline]
    pub(crate) fn create_trade_ws_request(
        &self,
    ) -> tokio_tungstenite::tungstenite::Result<Request<()>> {
        self.create_ws_request(&self.trade_ws_url)
    }
}
