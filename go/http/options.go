package http

import (
	"net/http"
	"time"

	whale "github.com/longportapp/whaleapi-sdk/go"
)

// Options for the HTTP client.
type Options struct {
	URL         string
	AppKey      string
	AppSecret   string
	AccessToken string
	Timeout     time.Duration
	Client      *http.Client
	Language    whale.Language

	// ExtraHeaders are additional HTTP headers sent with every request.
	ExtraHeaders map[string]string
}

// Option configures the HTTP client.
type Option func(*Options)

// WithClient uses a custom *http.Client.
func WithClient(cli *http.Client) Option {
	return func(opts *Options) {
		if cli != nil {
			opts.Client = cli
		}
	}
}

// WithURL sets the HTTP url.
func WithURL(url string) Option {
	return func(opts *Options) {
		if url != "" {
			opts.URL = url
		}
	}
}

// WithAppKey sets the app key.
func WithAppKey(appKey string) Option {
	return func(opts *Options) {
		if appKey != "" {
			opts.AppKey = appKey
		}
	}
}

// WithAppSecret sets the app secret.
func WithAppSecret(appSecret string) Option {
	return func(opts *Options) {
		if appSecret != "" {
			opts.AppSecret = appSecret
		}
	}
}

// WithAccessToken sets the access token.
func WithAccessToken(accessToken string) Option {
	return func(opts *Options) {
		if accessToken != "" {
			opts.AccessToken = accessToken
		}
	}
}

// WithTimeout sets the HTTP client timeout. Effective only when WithClient is not set.
func WithTimeout(timeout time.Duration) Option {
	return func(opts *Options) {
		if timeout > 0 {
			opts.Timeout = timeout
		}
	}
}

// WithLanguage sets the request language.
func WithLanguage(language whale.Language) Option {
	return func(opts *Options) {
		if language != "" {
			opts.Language = language
		}
	}
}

// WithExtraHeaders sets additional HTTP headers to send with every request.
func WithExtraHeaders(headers map[string]string) Option {
	return func(opts *Options) {
		if len(headers) > 0 {
			opts.ExtraHeaders = headers
		}
	}
}

func newOptions(opt ...Option) *Options {
	opts := Options{
		Timeout:  whale.DefaultHTTPTimeout,
		URL:      whale.DefaultHTTPURL,
		Language: whale.LanguageEN,
	}
	for _, o := range opt {
		o(&opts)
	}
	return &opts
}
