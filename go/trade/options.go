package trade

import (
	whale "github.com/longportapp/whaleapi-sdk/go"
	"github.com/longportapp/whaleapi-sdk/go/http"
	"github.com/longportapp/whaleapi-sdk/go/log"
	"github.com/longportapp/whaleapi-sdk/go/longbridge"

	protocol "github.com/longbridge/openapi-protocol/go"
)

// Options for the trade context.
type Options struct {
	tradeURL           string
	httpClient         *http.Client
	lbOpts             *longbridge.Options
	logLevel           string
	logger             log.Logger
	reconnectCallbacks []func(resubFlag bool)
}

// Option configures the trade context.
type Option func(*Options)

// WithTradeURL sets the trade websocket url.
func WithTradeURL(url string) Option {
	return func(o *Options) {
		if url != "" {
			o.tradeURL = url
		}
	}
}

// WithHttpClient sets the HTTP client used to fetch the websocket OTP.
func WithHttpClient(client *http.Client) Option {
	return func(o *Options) {
		if client != nil {
			o.httpClient = client
		}
	}
}

// WithLbOptions sets the low-level websocket options.
func WithLbOptions(opts *longbridge.Options) Option {
	return func(o *Options) {
		if opts != nil {
			o.lbOpts = opts
		}
	}
}

// WithLogLevel sets the log level.
func WithLogLevel(level string) Option {
	return func(o *Options) {
		if level != "" {
			o.logLevel = level
		}
	}
}

// WithLogger sets the logger.
func WithLogger(logger log.Logger) Option {
	return func(o *Options) {
		if logger != nil {
			o.logger = logger
		}
	}
}

// OnReconnect registers a callback invoked after a reconnect (with whether
// resubscribe succeeded).
func OnReconnect(fn func(successResub bool)) Option {
	return func(o *Options) {
		o.reconnectCallbacks = append(o.reconnectCallbacks, fn)
	}
}

func newOptions(opt ...Option) *Options {
	opts := Options{
		tradeURL: whale.DefaultTradeWsURL,
		lbOpts:   longbridge.NewOptions(),
		logger:   &protocol.DefaultLogger{},
	}
	for _, o := range opt {
		o(&opts)
	}
	return &opts
}
