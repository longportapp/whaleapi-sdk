// Package longbridge holds low-level options for the websocket connection.
package longbridge

import (
	"time"
)

// Defaults for the websocket connection.
var (
	DefaultWriteQueueSize = 16
	DefaultReadBufferSize = 4096
	DefaultReadQueueSize  = 16
	DefaultMinGzipSize    = 1024
	DefaultDialTimeout    = time.Second * 5
	DefaultAuthTimeout    = time.Second * 10
)

// Options for the websocket connection.
type Options struct {
	AuthTimeout    time.Duration
	Timeout        time.Duration
	WriteQueueSize int
	ReadQueueSize  int
	ReadBufferSize int
	MinGzipSize    int
}

// Option configures Options.
type Option func(*Options)

func WithAuthTimeout(timeout time.Duration) Option {
	return func(o *Options) {
		if timeout > 0 {
			o.AuthTimeout = timeout
		}
	}
}

func WithTimeout(timeout time.Duration) Option {
	return func(o *Options) {
		if timeout > 0 {
			o.Timeout = timeout
		}
	}
}

func WithWriteQueueSize(size int) Option {
	return func(o *Options) {
		if size > 0 {
			o.WriteQueueSize = size
		}
	}
}

func WithReadQueueSize(size int) Option {
	return func(o *Options) {
		if size > 0 {
			o.ReadQueueSize = size
		}
	}
}

func WithReadBufferSize(size int) Option {
	return func(o *Options) {
		if size > 0 {
			o.ReadBufferSize = size
		}
	}
}

func WithMinGzipSize(size int) Option {
	return func(o *Options) {
		if size > 0 {
			o.MinGzipSize = size
		}
	}
}

// NewOptions creates Options with defaults applied.
func NewOptions(opt ...Option) *Options {
	opts := Options{
		AuthTimeout:    DefaultAuthTimeout,
		Timeout:        DefaultDialTimeout,
		WriteQueueSize: DefaultWriteQueueSize,
		ReadQueueSize:  DefaultReadQueueSize,
		ReadBufferSize: DefaultReadBufferSize,
		MinGzipSize:    DefaultMinGzipSize,
	}
	for _, o := range opt {
		o(&opts)
	}
	return &opts
}
