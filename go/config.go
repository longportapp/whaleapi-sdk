package longportwhale

import (
	"os"
	"time"

	_ "github.com/joho/godotenv/autoload"
	"github.com/pkg/errors"
)

// Default endpoints and timeouts.
const (
	// DefaultHTTPURL is the default WhaleAPI HTTP endpoint.
	DefaultHTTPURL = "https://openapi.longportapp.com"
	// DefaultTradeWsURL is the default WhaleAPI trade websocket endpoint.
	DefaultTradeWsURL = "wss://openapi-trade.longportapp.com"
	// DefaultHTTPTimeout is the default HTTP request timeout.
	DefaultHTTPTimeout = 15 * time.Second
)

// Config holds the WhaleAPI SDK configuration.
type Config struct {
	// AppKey is the application key.
	AppKey string
	// AppSecret is the application secret.
	AppSecret string
	// AccessToken is the access token.
	AccessToken string
	// HTTPURL is the HTTP API url. Default: DefaultHTTPURL.
	HTTPURL string
	// TradeWsURL is the trade websocket url. Default: DefaultTradeWsURL.
	TradeWsURL string
	// Language is the request language. Default: LanguageEN.
	Language Language

	// HTTPTimeout is the HTTP request timeout. Default: DefaultHTTPTimeout.
	HTTPTimeout time.Duration
	// LogLevel is the log level for the trade websocket client.
	LogLevel string

	// Websocket tuning options (optional).
	AuthTimeout    time.Duration
	Timeout        time.Duration
	WriteQueueSize int
	ReadQueueSize  int
	ReadBufferSize int
	MinGzipSize    int

	// ExtraHeaders are additional HTTP headers sent with every request.
	ExtraHeaders map[string]string
}

// NewConfig creates a Config with the given credentials and default endpoints.
func NewConfig(appKey, appSecret, accessToken string) *Config {
	return &Config{
		AppKey:      appKey,
		AppSecret:   appSecret,
		AccessToken: accessToken,
		HTTPURL:     DefaultHTTPURL,
		TradeWsURL:  DefaultTradeWsURL,
		Language:    LanguageEN,
	}
}

// ConfigFromEnv creates a Config from environment variables.
//
// It first loads variables from a .env file in the current directory (if any).
//
// Variables:
//   - LONGPORT_APP_KEY       App key
//   - LONGPORT_APP_SECRET    App secret
//   - LONGPORT_ACCESS_TOKEN  Access token
//   - LONGPORT_HTTP_URL      HTTP endpoint url (default: DefaultHTTPURL)
//   - LONGPORT_TRADE_WS_URL  Trade websocket endpoint url (default: DefaultTradeWsURL)
//   - LONGPORT_LANGUAGE      Language (default: en)
func ConfigFromEnv() (*Config, error) {
	cfg := &Config{
		AppKey:      os.Getenv("LONGPORT_APP_KEY"),
		AppSecret:   os.Getenv("LONGPORT_APP_SECRET"),
		AccessToken: os.Getenv("LONGPORT_ACCESS_TOKEN"),
		HTTPURL:     getenvDefault("LONGPORT_HTTP_URL", DefaultHTTPURL),
		TradeWsURL:  getenvDefault("LONGPORT_TRADE_WS_URL", DefaultTradeWsURL),
		Language:    LanguageEN,
	}
	if lang := os.Getenv("LONGPORT_LANGUAGE"); lang != "" {
		cfg.Language = Language(lang)
	}
	if err := cfg.Check(); err != nil {
		return nil, err
	}
	return cfg, nil
}

// WithLanguage sets the language and returns the config for chaining.
func (c *Config) WithLanguage(language Language) *Config {
	if language != "" {
		c.Language = language
	}
	return c
}

// WithHTTPURL sets the HTTP url and returns the config for chaining.
func (c *Config) WithHTTPURL(url string) *Config {
	if url != "" {
		c.HTTPURL = url
	}
	return c
}

// WithTradeWsURL sets the trade websocket url and returns the config for chaining.
func (c *Config) WithTradeWsURL(url string) *Config {
	if url != "" {
		c.TradeWsURL = url
	}
	return c
}

// WithHeader adds an extra HTTP header and returns the config for chaining.
func (c *Config) WithHeader(key, value string) *Config {
	if c.ExtraHeaders == nil {
		c.ExtraHeaders = make(map[string]string)
	}
	c.ExtraHeaders[key] = value
	return c
}

// Check validates that required credentials are present.
func (c *Config) Check() error {
	if c.AccessToken == "" {
		return errors.New("missing access token (set LONGPORT_ACCESS_TOKEN)")
	}
	if c.AppKey == "" {
		return errors.New("missing app key (set LONGPORT_APP_KEY)")
	}
	if c.AppSecret == "" {
		return errors.New("missing app secret (set LONGPORT_APP_SECRET)")
	}
	return nil
}

func getenvDefault(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}
