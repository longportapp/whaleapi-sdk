use strum_macros::{Display, EnumString};

/// Order type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit Order
    #[strum(serialize = "LO")]
    LO,
    /// Enhanced Limit Order
    #[strum(serialize = "ELO")]
    ELO,
    /// Market Order
    #[strum(serialize = "MO")]
    MO,
    /// At-auction Order
    #[strum(serialize = "AO")]
    AO,
    /// At-auction Limit Order
    #[strum(serialize = "ALO")]
    ALO,
    /// Odd Lots
    #[strum(serialize = "ODD")]
    ODD,
    /// Limit If Touched
    #[strum(serialize = "LIT")]
    LIT,
    /// Market If Touched
    #[strum(serialize = "MIT")]
    MIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[strum(serialize = "TSLPAMT")]
    TSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[strum(serialize = "TSLPPCT")]
    TSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[strum(serialize = "TSMAMT")]
    TSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[strum(serialize = "TSMPCT")]
    TSMPCT,
    /// Special Limit Order
    #[strum(serialize = "SLO")]
    SLO,
}

/// Order status
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Not reported
    #[strum(serialize = "NotReported")]
    NotReported,
    /// Not reported (Replaced Order)
    #[strum(serialize = "ReplacedNotReported")]
    ReplacedNotReported,
    /// Not reported (Protected Order)
    #[strum(serialize = "ProtectedNotReported")]
    ProtectedNotReported,
    /// Not reported (Conditional Order)
    #[strum(serialize = "VarietiesNotReported")]
    VarietiesNotReported,
    /// Filled
    #[strum(serialize = "FilledStatus")]
    Filled,
    /// Wait To New
    #[strum(serialize = "WaitToNew")]
    WaitToNew,
    /// New
    #[strum(serialize = "NewStatus")]
    New,
    /// Wait To Replace
    #[strum(serialize = "WaitToReplace")]
    WaitToReplace,
    /// Pending Replace
    #[strum(serialize = "PendingReplaceStatus")]
    PendingReplace,
    /// Replaced
    #[strum(serialize = "ReplacedStatus")]
    Replaced,
    /// Partial Filled
    #[strum(serialize = "PartialFilledStatus")]
    PartialFilled,
    /// Wait To Cancel
    #[strum(serialize = "WaitToCancel")]
    WaitToCancel,
    /// Pending Cancel
    #[strum(serialize = "PendingCancelStatus")]
    PendingCancel,
    /// Rejected
    #[strum(serialize = "RejectedStatus")]
    Rejected,
    /// Canceled
    #[strum(serialize = "CanceledStatus")]
    Canceled,
    /// Expired
    #[strum(serialize = "ExpiredStatus")]
    Expired,
    /// Partial Withdrawal
    #[strum(serialize = "PartialWithdrawal")]
    PartialWithdrawal,
}

/// Order side
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderSide {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Buy
    #[strum(serialize = "Buy")]
    Buy,
    /// Sell
    #[strum(serialize = "Sell")]
    Sell,
}

/// Order tag
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderTag {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Normal Order
    #[strum(serialize = "Normal")]
    Normal,
    /// Long term Order
    #[strum(serialize = "GTC")]
    LongTerm,
    /// Grey Order
    #[strum(serialize = "Grey")]
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
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TriggerStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Deactive
    #[strum(serialize = "DEACTIVE")]
    Deactive,
    /// Active
    #[strum(serialize = "ACTIVE")]
    Active,
    /// Released
    #[strum(serialize = "RELEASED")]
    Released,
}

impl_serde_for_enum_string!(OrderType, OrderStatus, OrderSide, OrderTag, TriggerStatus);

impl_default_for_enum_string!(OrderType, OrderStatus, OrderSide, OrderTag, TriggerStatus);
