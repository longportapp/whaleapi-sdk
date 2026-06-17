# WhaleAPI SDK for Go

Go SDK for Longbridge WhaleAPI. It provides only what the WhaleAPI exposes:

- a generic **HTTP client** (`http`)
- a **trade push websocket** (`trade`, `TradeContext`)

Requires Go `>= 1.24`.

## Install

```bash
go get github.com/longportapp/whaleapi-sdk/go
```

## Configuration

Configuration is read from environment variables (a `.env` file in the working
directory is loaded automatically):

| Variable                | Description                | Default                                  |
|-------------------------|----------------------------|------------------------------------------|
| `LONGPORT_APP_KEY`      | App key                    | —                                        |
| `LONGPORT_APP_SECRET`   | App secret                 | —                                        |
| `LONGPORT_ACCESS_TOKEN` | Access token               | —                                        |
| `LONGPORT_HTTP_URL`     | HTTP endpoint url          | `https://openapi.longportapp.com`        |
| `LONGPORT_TRADE_WS_URL` | Trade websocket endpoint   | `wss://openapi-trade.longportapp.com`    |
| `LONGPORT_LANGUAGE`     | Language (`en`/`zh-CN`/...)| `en`                                     |

## Usage

```go
package main

import (
	"context"
	"fmt"

	longportwhale "github.com/longportapp/whaleapi-sdk/go"
	"github.com/longportapp/whaleapi-sdk/go/http"
	"github.com/longportapp/whaleapi-sdk/go/trade"
)

func main() {
	conf, _ := longportwhale.ConfigFromEnv()
	httpcli, _ := http.FromEnv()

	// Trade push websocket.
	tctx, _ := trade.NewFromCfg(conf)
	defer tctx.Close()
	tctx.OnOrderChanged(func(ev *trade.PushOrderChanged) {
		fmt.Printf("order event: %+v\n", ev)
	})
	tctx.Subscribe(context.Background(), []trade.TopicType{trade.TopicPrivate})

	// Generic HTTP request.
	var res struct {
		OrderId string `json:"order_id"`
	}
	body := map[string]string{
		"symbol":             "700.HK",
		"order_type":         "MO",
		"side":               "Buy",
		"submitted_quantity": "100",
		"time_in_force":      "Day",
		"account_no":         "your-account-no",
	}
	httpcli.Request(context.Background(), "POST", "/v1/whaleapi/trade/order", nil, body, &res)
	fmt.Println("order id:", res.OrderId)
}
```

See [`examples/go/whale`](../examples/go/whale) for a complete example.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
