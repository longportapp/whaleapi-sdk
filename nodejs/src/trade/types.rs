use chrono::{DateTime, Utc};
use longportwhale_nodejs_macros::{JsEnum, JsObject};

use crate::decimal::Decimal;

/// Topic type
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::TopicType")]
pub enum TopicType {
    /// Private notification for trade
    Private,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::OrderStatus")]
pub enum OrderStatus {
    /// Unknown
    Unknown,
    /// Not reported
    NotReported,
    /// Not reported (Replaced Order)
    ReplacedNotReported,
    /// Not reported (Protected Order)
    ProtectedNotReported,
    /// Not reported (Conditional Order)
    VarietiesNotReported,
    /// Filled
    Filled,
    /// Wait To New
    WaitToNew,
    /// New
    New,
    /// Wait To Replace
    WaitToReplace,
    /// Pending Replace
    PendingReplace,
    /// Replaced
    Replaced,
    /// Partial Filled
    PartialFilled,
    /// Wait To Cancel
    WaitToCancel,
    /// Pending Cancel
    PendingCancel,
    /// Rejected
    Rejected,
    /// Canceled
    Canceled,
    /// Expired
    Expired,
    /// Partial Withdrawal
    PartialWithdrawal,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::OrderSide")]
pub enum OrderSide {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::OrderType")]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
    /// Unknown
    Unknown,
    /// Limit Order
    LO,
    /// Enhanced Limit Order
    ELO,
    /// Market Order
    MO,
    /// At-auction Order
    AO,
    /// At-auction Limit Order
    ALO,
    /// Odd Lots
    ODD,
    /// Limit If Touched
    LIT,
    /// Market If Touched
    MIT,
    /// Trailing Limit If Touched (Trailing Amount)
    TSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    TSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    TSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    TSMPCT,
    /// Special Limit Order
    SLO,
}

/// Order tag
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::OrderTag")]
pub enum OrderTag {
    /// Unknown
    Unknown,
    /// Normal Order
    Normal,
    /// Long term Order
    LongTerm,
    /// Grey Order
    Grey,
    /// Force Selling
    MarginCall,
    /// OTC
    Offline,
    /// Option Exercise Long
    Creditor,
    /// Option Exercise Short
    Debtor,
    /// Wavier Of Option Exercise
    NonExercise,
    /// Trade Allocation
    AllocatedSub,
}

/// Trigger status
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longportwhale::trade::TriggerStatus")]
pub enum TriggerStatus {
    /// Unknown
    Unknown,
    /// Deactive
    Deactive,
    /// Active
    Active,
    /// Released
    Released,
}

/// Order changed message
#[napi_derive::napi]
#[derive(Debug, JsObject)]
#[js(remote = "longportwhale::trade::PushOrderChanged")]
pub struct PushOrderChanged {
    /// Order side
    side: OrderSide,
    /// Stock name
    stock_name: String,
    /// Submitted quantity
    submitted_quantity: i64,
    /// Order symbol
    symbol: String,
    /// Order type
    order_type: OrderType,
    /// Submitted price
    submitted_price: Decimal,
    /// Executed quantity
    executed_quantity: i64,
    /// Executed price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Order ID
    order_id: String,
    /// Currency
    currency: String,
    /// Order status
    status: OrderStatus,
    /// Submitted time
    #[js(datetime)]
    submitted_at: DateTime<Utc>,
    /// Last updated time
    #[js(datetime)]
    updated_at: DateTime<Utc>,
    /// Order trigger price
    #[js(opt)]
    trigger_price: Option<Decimal>,
    /// Rejected message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Conditional order trigger status
    #[js(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[js(opt, datetime)]
    trigger_at: Option<DateTime<Utc>>,
    /// Trailing amount
    #[js(opt)]
    trailing_amount: Option<Decimal>,
    /// Trailing percent
    #[js(opt)]
    trailing_percent: Option<Decimal>,
    /// Limit offset amount
    #[js(opt)]
    limit_offset: Option<Decimal>,
    /// Account no
    account_no: String,
    /// Last share
    #[js(opt)]
    last_share: Option<Decimal>,
    /// Last price
    #[js(opt)]
    last_price: Option<Decimal>,
    /// Remark message
    remark: String,
}
