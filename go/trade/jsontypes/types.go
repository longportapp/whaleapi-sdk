// Package jsontypes holds the raw JSON shapes pushed by the WhaleAPI trade server.
package jsontypes

// PushEvent is the trade push event envelope.
type PushEvent struct {
	Event string            `json:"event"`
	Data  *PushOrderChanged `json:"data"`
}

// PushOrderChanged is the order-changed event details (raw JSON form).
type PushOrderChanged struct {
	AccountNo        string `json:"account_no"`
	Currency         string `json:"currency"`
	ExecutedPrice    string `json:"executed_price"`
	ExecutedQuantity string `json:"executed_quantity"`
	LastPrice        string `json:"last_price"`
	LastShare        string `json:"last_share"`
	LimitOffset      string `json:"limit_offset"`
	Msg              string `json:"msg"`
	OrderId          string `json:"order_id"`
	OrderType        string `json:"order_type"`
	Side             string `json:"side"`
	Status           string `json:"status"`
	StockName        string `json:"stock_name"`
	SubmittedAt      string `json:"submitted_at"`
	Price            string `json:"submitted_price"`
	Quantity         string `json:"submitted_quantity"`
	Symbol           string `json:"symbol"`
	Tag              string `json:"tag"`
	TrailingAmount   string `json:"trailing_amount"`
	TrailingPercent  string `json:"trailing_percent"`
	TriggerAt        string `json:"trigger_at"`
	TriggerPrice     string `json:"trigger_price"`
	TriggerStatus    string `json:"trigger_status"`
	UpdatedAt        string `json:"updated_at"`
	Remark           string `json:"remark"`
}
