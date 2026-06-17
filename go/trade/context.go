// Package trade provides the WhaleAPI trade push websocket client (TradeContext).
//
// WhaleAPI trade push reference:
// https://open.longportapp.com/en/docs/trade/trade-push
package trade

import (
	"context"

	whale "github.com/longportapp/whaleapi-sdk/go"
	"github.com/longportapp/whaleapi-sdk/go/http"
	"github.com/longportapp/whaleapi-sdk/go/longbridge"

	"github.com/pkg/errors"
)

// TradeContext is a client for the WhaleAPI trade push websocket.
//
// Example:
//
//	conf, _ := longportwhale.ConfigFromEnv()
//	tctx, _ := trade.NewFromCfg(conf)
//	tctx.OnOrderChanged(func(ev *trade.PushOrderChanged) {
//		fmt.Printf("order event: %+v\n", ev)
//	})
//	_, _ = tctx.Subscribe(context.Background(), []trade.TopicType{trade.TopicPrivate})
type TradeContext struct {
	opts *Options
	core *core
}

// OnOrderChanged sets the callback invoked when an order-changed event is pushed.
func (c *TradeContext) OnOrderChanged(f func(*PushOrderChanged)) {
	c.core.SetHandler(func(ev *PushEvent) {
		if ev != nil && ev.Data != nil {
			f(ev.Data)
		}
	})
}

// OnTrade sets the callback invoked with the full push event envelope.
func (c *TradeContext) OnTrade(f func(*PushEvent)) {
	c.core.SetHandler(f)
}

// Subscribe subscribes to the given topics. After subscribing, the handler set
// via OnOrderChanged / OnTrade will receive push events.
// Reference: https://open.longportapp.com/en/docs/trade/trade-push#subscribe
func (c *TradeContext) Subscribe(ctx context.Context, topics []TopicType) (*SubResponse, error) {
	return c.core.Subscribe(ctx, topicsToStrings(topics))
}

// Unsubscribe unsubscribes from the given topics.
// Reference: https://open.longportapp.com/en/docs/trade/trade-push#cancel-subscribe
func (c *TradeContext) Unsubscribe(ctx context.Context, topics []TopicType) (*UnsubResponse, error) {
	return c.core.Unsubscribe(ctx, topicsToStrings(topics))
}

// Close closes the websocket connection.
func (c *TradeContext) Close() error {
	return c.core.Close()
}

// NewFromCfg creates a TradeContext from a *longportwhale.Config.
func NewFromCfg(cfg *whale.Config) (*TradeContext, error) {
	httpClient, err := http.NewFromCfg(cfg)
	if err != nil {
		return nil, errors.Wrap(err, "create http client error")
	}
	lbOpts := longbridge.NewOptions(
		longbridge.WithAuthTimeout(cfg.AuthTimeout),
		longbridge.WithTimeout(cfg.Timeout),
		longbridge.WithReadBufferSize(cfg.ReadBufferSize),
		longbridge.WithReadQueueSize(cfg.ReadQueueSize),
		longbridge.WithWriteQueueSize(cfg.WriteQueueSize),
		longbridge.WithMinGzipSize(cfg.MinGzipSize),
	)
	return New(
		WithTradeURL(cfg.TradeWsURL),
		WithHttpClient(httpClient),
		WithLbOptions(lbOpts),
		WithLogLevel(cfg.LogLevel),
	)
}

// New creates a TradeContext from options. A connection to the trade server is
// established eagerly. A *http.Client (WithHttpClient) is required to fetch the
// websocket OTP.
func New(opt ...Option) (*TradeContext, error) {
	opts := newOptions(opt...)
	if opts.httpClient == nil {
		return nil, errors.New("http client is required (use WithHttpClient)")
	}
	core, err := newCore(opts)
	if err != nil {
		return nil, errors.Wrap(err, "failed to create core")
	}
	return &TradeContext{opts: opts, core: core}, nil
}
