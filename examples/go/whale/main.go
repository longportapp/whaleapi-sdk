package main

import (
	"context"
	"fmt"
	"os"
	"time"

	longportwhale "github.com/longportapp/whaleapi-sdk/go"
	"github.com/longportapp/whaleapi-sdk/go/http"
	"github.com/longportapp/whaleapi-sdk/go/trade"
)

const testAccount = "L6VQEU00121996"

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
	fmt.Println("Verification successful!")
}

func run() error {
	fmt.Println("Starting verification...")

	// Load config and create the HTTP client from environment variables.
	conf, err := longportwhale.ConfigFromEnv()
	if err != nil {
		return fmt.Errorf("failed to load config: %w", err)
	}
	httpcli, err := http.FromEnv()
	if err != nil {
		return fmt.Errorf("failed to create http client: %w", err)
	}

	// Create the trade context and register the order-changed callback.
	tctx, err := trade.NewFromCfg(conf)
	if err != nil {
		return fmt.Errorf("failed to create trade context: %w", err)
	}
	defer tctx.Close()

	events := make(chan *trade.PushOrderChanged, 16)
	tctx.OnOrderChanged(func(ev *trade.PushOrderChanged) {
		fmt.Printf("Received order event: %+v\n", ev)
		events <- ev
	})

	// Subscribe to private notifications.
	fmt.Println("Subscribing to private events...")
	if _, err := tctx.Subscribe(context.Background(), []trade.TopicType{trade.TopicPrivate}); err != nil {
		return fmt.Errorf("failed to subscribe: %w", err)
	}
	time.Sleep(2 * time.Second) // wait for the subscription to take effect

	// Submit an order via the generic HTTP client.
	req := map[string]string{
		"symbol":             "700.HK",
		"order_type":         "MO",
		"side":               "Buy",
		"submitted_quantity": "100",
		"time_in_force":      "Day",
		"account_no":         testAccount,
	}
	fmt.Printf("Submitting order: %v\n", req)

	var submitRes struct {
		OrderId string `json:"order_id"`
	}
	if err := httpcli.Request(context.Background(), "POST", "/v1/whaleapi/trade/order", nil, req, &submitRes); err != nil {
		return fmt.Errorf("failed to submit order: %w", err)
	}
	fmt.Printf("Order submitted. Order ID: %s\n", submitRes.OrderId)

	// Wait for the matching order-changed push event.
	fmt.Printf("Waiting for order event for order_id: %s...\n", submitRes.OrderId)
	timeout := time.After(60 * time.Second)
	for {
		select {
		case ev := <-events:
			if ev.OrderId == submitRes.OrderId {
				fmt.Println("Received expected order event!")
				return nil
			}
		case <-timeout:
			return fmt.Errorf("timeout waiting for order event")
		}
	}
}
