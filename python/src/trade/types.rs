use longportwhale_python_macros::{PyEnum, PyObject};
use pyo3::pyclass;

use crate::{decimal::PyDecimal, time::PyOffsetDateTimeWrapper};

/// Topic type
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::TopicType")]
pub(crate) enum TopicType {
    /// Private notification for trade
    Private,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::OrderStatus")]
pub(crate) enum OrderStatus {
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

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::OrderSide")]
pub(crate) enum OrderSide {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::OrderType")]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum OrderType {
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
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::OrderTag")]
pub(crate) enum OrderTag {
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
#[pyclass]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longportwhale::trade::TriggerStatus")]
pub(crate) enum TriggerStatus {
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
#[pyclass]
#[derive(Debug, PyObject)]
#[py(remote = "longportwhale::trade::PushOrderChanged")]
pub(crate) struct PushOrderChanged {
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
    submitted_price: PyDecimal,
    /// Executed quantity
    executed_quantity: i64,
    /// Executed price
    #[py(opt)]
    executed_price: Option<PyDecimal>,
    /// Order ID
    order_id: String,
    /// Currency
    currency: String,
    /// Order status
    status: OrderStatus,
    /// Submitted time
    submitted_at: PyOffsetDateTimeWrapper,
    /// Last updated time
    updated_at: PyOffsetDateTimeWrapper,
    /// Order trigger price
    #[py(opt)]
    trigger_price: Option<PyDecimal>,
    /// Rejected message or remark
    msg: String,
    /// Order tag
    tag: OrderTag,
    /// Conditional order trigger status
    #[py(opt)]
    trigger_status: Option<TriggerStatus>,
    /// Conditional order trigger time
    #[py(opt)]
    trigger_at: Option<PyOffsetDateTimeWrapper>,
    /// Trailing amount
    #[py(opt)]
    trailing_amount: Option<PyDecimal>,
    /// Trailing percent
    #[py(opt)]
    trailing_percent: Option<PyDecimal>,
    /// Limit offset amount
    #[py(opt)]
    limit_offset: Option<PyDecimal>,
    /// Account no
    account_no: String,
    /// Last share
    #[py(opt)]
    last_share: Option<PyDecimal>,
    /// Last price
    #[py(opt)]
    last_price: Option<PyDecimal>,
    /// Remark message
    remark: String,
}
