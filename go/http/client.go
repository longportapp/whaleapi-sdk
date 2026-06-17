// Package http is the WhaleAPI REST client.
package http

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"io"
	nhttp "net/http"
	"net/url"
	"strings"
	"time"

	"github.com/google/go-querystring/query"

	whale "github.com/longportapp/whaleapi-sdk/go"
	"github.com/longportapp/whaleapi-sdk/go/log"
)

type apiResponse struct {
	Code    int             `json:"code"`
	Message string          `json:"message"`
	Data    json.RawMessage `json:"data"`
	TraceID string          `json:"-"`
}

type otpResponse struct {
	Otp string `json:"otp"`
}

// Client is an HTTP client to access the WhaleAPI REST endpoints.
type Client struct {
	opts       *Options
	httpClient *nhttp.Client
}

// RequestOptions carries additional information for a request.
type RequestOptions struct {
	// Header is the request header.
	Header nhttp.Header
	body   interface{}
}

// RequestOption sets additional information on a request.
type RequestOption func(*RequestOptions)

// WithHeader sets the request header.
func WithHeader(h nhttp.Header) RequestOption {
	return func(o *RequestOptions) {
		o.Header = h
	}
}

// WithBody sets the request payload.
func WithBody(v interface{}) RequestOption {
	return func(o *RequestOptions) {
		if v != nil {
			o.body = v
		}
	}
}

// Get sends a GET request with query params.
func (c *Client) Get(ctx context.Context, path string, queryParams url.Values, resp interface{}, ropts ...RequestOption) error {
	return c.Call(ctx, "GET", path, queryParams, nil, resp, ropts...)
}

// Post sends a POST request with a JSON body.
func (c *Client) Post(ctx context.Context, path string, body interface{}, resp interface{}, ropts ...RequestOption) error {
	return c.Call(ctx, "POST", path, nil, body, resp, ropts...)
}

// Put sends a PUT request with a JSON body.
func (c *Client) Put(ctx context.Context, path string, body interface{}, resp interface{}, ropts ...RequestOption) error {
	return c.Call(ctx, "PUT", path, nil, body, resp, ropts...)
}

// Delete sends a DELETE request with query params.
func (c *Client) Delete(ctx context.Context, path string, queryParams interface{}, resp interface{}, ropts ...RequestOption) error {
	return c.Call(ctx, "DELETE", path, queryParams, nil, resp, ropts...)
}

// Request performs a generic HTTP request and decodes the response `data` field into resp.
//
// It mirrors the generic request method exposed by the other WhaleAPI SDKs:
//
//	var res struct{ OrderId string `json:"order_id"` }
//	err := cli.Request(ctx, "POST", "/v1/whaleapi/trade/order", nil, body, &res)
func (c *Client) Request(ctx context.Context, method, path string, headers map[string]string, body interface{}, resp interface{}) error {
	var ropts []RequestOption
	if len(headers) > 0 {
		h := nhttp.Header{}
		for k, v := range headers {
			h.Set(k, v)
		}
		ropts = append(ropts, WithHeader(h))
	}
	return c.Call(ctx, method, path, nil, body, resp, ropts...)
}

// GetOTP gets a one-time-password for the websocket connection.
// Reference: https://open.longportapp.com/en/docs/socket-token-api
func (c *Client) GetOTP(ctx context.Context, ropts ...RequestOption) (string, error) {
	res := &otpResponse{}
	if err := c.Get(ctx, "/v1/socket/token", nil, res, ropts...); err != nil {
		return "", err
	}
	return res.Otp, nil
}

// Call sends a signed request to the WhaleAPI server and decodes the `data` field into resp.
func (c *Client) Call(ctx context.Context, method, path string, queryParams interface{}, body interface{}, resp interface{}, ropts ...RequestOption) (err error) {
	var (
		br       io.Reader
		bb       []byte
		httpResp *nhttp.Response
		rb       []byte
	)

	ro := &RequestOptions{}
	for _, opt := range ropts {
		opt(ro)
	}

	if body == nil && ro.body != nil {
		body = ro.body
	}

	if body != nil {
		bb, err = json.Marshal(body)
		if err != nil {
			return err
		}
		br = bytes.NewBuffer(bb)
	}

	req, err := nhttp.NewRequestWithContext(ctx, method, c.opts.URL+path, br)
	if err != nil {
		return err
	}

	// set headers
	req.Header.Set("accept-language", string(c.opts.Language))
	req.Header.Set("x-api-key", c.opts.AppKey)
	req.Header.Set("authorization", c.opts.AccessToken)
	for k, v := range c.opts.ExtraHeaders {
		req.Header.Set(k, v)
	}
	if ro.Header != nil {
		for k, v := range ro.Header {
			req.Header[k] = v
		}
	}
	if len(bb) != 0 {
		req.Header.Set("content-type", "application/json; charset=utf-8")
	}

	// set query params
	if queryParams != nil {
		vals, ok := queryParams.(url.Values)
		if !ok {
			if vals, err = query.Values(queryParams); err != nil {
				return
			}
		}
		req.URL.RawQuery = vals.Encode()
	}

	// sign the request
	if err = signature(req, c.opts.AppSecret, bb); err != nil {
		return err
	}

	log.Debugf("http call method:%v url:%v body:%v", req.Method, req.URL, string(bb))
	httpResp, err = c.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer httpResp.Body.Close()
	log.Debugf("http call response headers:%v", httpResp.Header)

	if rb, err = io.ReadAll(httpResp.Body); err != nil {
		return err
	}
	log.Debugf("http call response body:%s", rb)

	apiResp := &apiResponse{}
	if v := httpResp.Header.Get("x-trace-id"); v != "" {
		apiResp.TraceID = v
	}

	if isJSON(httpResp.Header.Get("content-type")) {
		if err = jsonUnmarshal(bytes.NewReader(rb), apiResp); err != nil {
			return err
		}
	} else {
		apiResp.Message = string(rb)
	}

	if httpResp.StatusCode != nhttp.StatusOK || apiResp.Code != 0 {
		return NewError(httpResp.StatusCode, apiResp)
	}

	if resp == nil {
		return nil
	}

	if len(apiResp.Data) == 0 {
		return nil
	}
	return jsonUnmarshal(bytes.NewReader(apiResp.Data), resp)
}

func isJSON(ct string) bool {
	return strings.Contains(ct, "application/json")
}

func jsonUnmarshal(r io.Reader, v interface{}) error {
	d := json.NewDecoder(r)
	d.UseNumber()
	return d.Decode(v)
}

// New creates an HTTP client from options.
func New(opt ...Option) (*Client, error) {
	opts := newOptions(opt...)
	if opts.URL == "" {
		return nil, errors.New("http url is empty")
	}

	cli := &nhttp.Client{
		Timeout: opts.Timeout,
		Transport: &nhttp.Transport{
			IdleConnTimeout: 60 * time.Second,
		},
	}
	if opts.Client != nil {
		cli = opts.Client
	}

	return &Client{opts: opts, httpClient: cli}, nil
}

// NewFromCfg creates an HTTP client from a *longportwhale.Config.
func NewFromCfg(c *whale.Config) (*Client, error) {
	httpURL := c.HTTPURL
	if httpURL == "" {
		httpURL = whale.DefaultHTTPURL
	}
	return New(
		WithURL(httpURL),
		WithTimeout(c.HTTPTimeout),
		WithLanguage(c.Language),
		WithAppKey(c.AppKey),
		WithAppSecret(c.AppSecret),
		WithAccessToken(c.AccessToken),
		WithExtraHeaders(c.ExtraHeaders),
	)
}

// FromEnv creates an HTTP client from environment variables.
// See longportwhale.ConfigFromEnv for the variables used.
func FromEnv() (*Client, error) {
	cfg, err := whale.ConfigFromEnv()
	if err != nil {
		return nil, err
	}
	return NewFromCfg(cfg)
}
