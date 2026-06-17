package trade

import (
	"github.com/shopspring/decimal"
)

// TopicType is a trade push topic.
type TopicType string

const (
	// TopicPrivate is the private notification topic for trade.
	TopicPrivate TopicType = "private"
)

type (
	OrderType     string
	OrderSide     string
	OrderStatus   string
	OrderTag      string
	TriggerStatus string
)

const (
	// Order type
	OrderTypeLO      OrderType = "LO"      // Limit Order
	OrderTypeELO     OrderType = "ELO"     // Enhanced Limit Order
	OrderTypeMO      OrderType = "MO"      // Market Order
	OrderTypeAO      OrderType = "AO"      // At-auction Order
	OrderTypeALO     OrderType = "ALO"     // At-auction Limit Order
	OrderTypeODD     OrderType = "ODD"     // Odd Lots Order
	OrderTypeLIT     OrderType = "LIT"     // Limit If Touched
	OrderTypeMIT     OrderType = "MIT"     // Market If Touched
	OrderTypeTSLPAMT OrderType = "TSLPAMT" // Trailing Limit If Touched (Trailing Amount)
	OrderTypeTSLPPCT OrderType = "TSLPPCT" // Trailing Limit If Touched (Trailing Percent)
	OrderTypeTSMAMT  OrderType = "TSMAMT"  // Trailing Market If Touched (Trailing Amount)
	OrderTypeTSMPCT  OrderType = "TSMPCT"  // Trailing Market If Touched (Trailing Percent)
	OrderTypeSLO     OrderType = "SLO"     // Special Limit Order

	// Order side
	OrderSideBuy  OrderSide = "Buy"
	OrderSideSell OrderSide = "Sell"

	// Order status
	OrderNotReported          OrderStatus = "NotReported"
	OrderReplacedNotReported  OrderStatus = "ReplacedNotReported"
	OrderProtectedNotReported OrderStatus = "ProtectedNotReported"
	OrderVarietiesNotReported OrderStatus = "VarietiesNotReported"
	OrderFilledStatus         OrderStatus = "FilledStatus"
	OrderWaitToNew            OrderStatus = "WaitToNew"
	OrderNewStatus            OrderStatus = "NewStatus"
	OrderWaitToReplace        OrderStatus = "WaitToReplace"
	OrderPendingReplaceStatus OrderStatus = "PendingReplaceStatus"
	OrderReplacedStatus       OrderStatus = "ReplacedStatus"
	OrderPartialFilledStatus  OrderStatus = "PartialFilledStatus"
	OrderWaitToCancel         OrderStatus = "WaitToCancel"
	OrderPendingCancelStatus  OrderStatus = "PendingCancelStatus"
	OrderRejectedStatus       OrderStatus = "RejectedStatus"
	OrderCanceledStatus       OrderStatus = "CanceledStatus"
	OrderExpiredStatus        OrderStatus = "ExpiredStatus"
	OrderPartialWithdrawal    OrderStatus = "PartialWithdrawal"

	// Order tag
	OrderTagNormal       OrderTag = "Normal"       // Normal Order
	OrderTagLongTerm     OrderTag = "GTC"          // Long term Order
	OrderTagGrey         OrderTag = "Grey"         // Grey Order
	OrderTagMarginCall   OrderTag = "MarginCall"   // Force Selling
	OrderTagOffline      OrderTag = "Offline"      // OTC
	OrderTagCreditor     OrderTag = "Creditor"     // Option Exercise Long
	OrderTagDebtor       OrderTag = "Debtor"       // Option Exercise Short
	OrderTagNonExercise  OrderTag = "NonExercise"  // Wavier Of Option Exercise
	OrderTagAllocatedSub OrderTag = "AllocatedSub" // Trade Allocation

	// Trigger status
	TriggerStatusDeactive TriggerStatus = "DEACTIVE"
	TriggerStatusActive   TriggerStatus = "ACTIVE"
	TriggerStatusReleased TriggerStatus = "RELEASED"
)

// PushEvent is a trade context callback event.
type PushEvent struct {
	Event string
	Data  *PushOrderChanged
}

// PushOrderChanged is the order-changed event details.
type PushOrderChanged struct {
	AccountNo        string
	Currency         string
	ExecutedPrice    *decimal.Decimal
	ExecutedQuantity *decimal.Decimal
	LastPrice        *decimal.Decimal
	LastShare        *decimal.Decimal
	LimitOffset      string
	Msg              string
	OrderId          string
	OrderType        OrderType
	Side             OrderSide
	Status           OrderStatus
	StockName        string
	SubmittedAt      string
	Price            *decimal.Decimal
	Quantity         *decimal.Decimal
	Symbol           string
	Tag              OrderTag
	TrailingAmount   *decimal.Decimal
	TrailingPercent  string
	TriggerAt        string
	TriggerPrice     *decimal.Decimal
	TriggerStatus    TriggerStatus
	UpdatedAt        string
	Remark           string
}

// SubResponse is the subscribe response.
type SubResponse struct {
	Success []string
	Fail    []*SubResponseFail
	Current []string
}

// SubResponseFail contains a failed subscription's reason.
type SubResponseFail struct {
	Topic  string
	Reason string
}

// UnsubResponse is the unsubscribe response.
type UnsubResponse struct {
	Current []string
}

func topicsToStrings(topics []TopicType) []string {
	out := make([]string, 0, len(topics))
	for _, t := range topics {
		out = append(out, string(t))
	}
	return out
}
