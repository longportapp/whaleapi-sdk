use longportwhale_c_macros::CEnum;

/// Topic type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::TopicType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTopicType {
    /// Trading
    #[c(remote = "Private")]
    TopicPrivate,
}

/// Order side
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::OrderSide")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderSide {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderSideUnknown,
    /// Buy
    #[c(remote = "Buy")]
    OrderSideBuy,
    /// Sell
    #[c(remote = "Sell")]
    OrderSideSell,
}

/// Order type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::OrderType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderType {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTypeUnknown,
    /// Limit Order
    #[c(remote = "LO")]
    OrderTypeLO,
    /// Enhanced Limit Order
    #[c(remote = "ELO")]
    OrderTypeELO,
    /// Market Order
    #[c(remote = "MO")]
    OrderTypeMO,
    /// At-auction Order
    #[c(remote = "AO")]
    OrderTypeAO,
    /// At-auction Limit Order
    #[c(remote = "ALO")]
    OrderTypeALO,
    /// Odd Lots
    #[c(remote = "ODD")]
    OrderTypeODD,
    /// Limit If Touched
    #[c(remote = "LIT")]
    OrderTypeLIT,
    /// Market If Touched
    #[c(remote = "MIT")]
    OrderTypeMIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[c(remote = "TSLPAMT")]
    OrderTypeTSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[c(remote = "TSLPPCT")]
    OrderTypeTSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[c(remote = "TSMAMT")]
    OrderTypeTSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[c(remote = "TSMPCT")]
    OrderTypeTSMPCT,
    /// Special Limit Order
    #[c(remote = "SLO")]
    OrderTypeSLO,
}

/// Order status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::OrderStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderStatusUnknown,
    /// Not reported
    #[c(remote = "NotReported")]
    OrderStatusNotReported,
    /// Not reported (Replaced Order)
    #[c(remote = "ReplacedNotReported")]
    OrderStatusReplacedNotReported,
    /// Not reported (Protected Order)
    #[c(remote = "ProtectedNotReported")]
    OrderStatusProtectedNotReported,
    /// Not reported (Conditional Order)
    #[c(remote = "VarietiesNotReported")]
    OrderStatusVarietiesNotReported,
    /// Filled
    #[c(remote = "Filled")]
    OrderStatusFilled,
    /// Wait To New
    #[c(remote = "WaitToNew")]
    OrderStatusWaitToNew,
    /// New
    #[c(remote = "New")]
    OrderStatusNew,
    /// Wait To Replace
    #[c(remote = "WaitToReplace")]
    OrderStatusWaitToReplace,
    /// Pending Replace
    #[c(remote = "PendingReplace")]
    OrderStatusPendingReplace,
    /// Replaced
    #[c(remote = "Replaced")]
    OrderStatusReplaced,
    /// Partial Filled
    #[c(remote = "PartialFilled")]
    OrderStatusPartialFilled,
    /// Wait To Cancel
    #[c(remote = "WaitToCancel")]
    OrderStatusWaitToCancel,
    /// Pending Cancel
    #[c(remote = "PendingCancel")]
    OrderStatusPendingCancel,
    /// Rejected
    #[c(remote = "Rejected")]
    OrderStatusRejected,
    /// Canceled
    #[c(remote = "Canceled")]
    OrderStatusCanceled,
    /// Expired
    #[c(remote = "Expired")]
    OrderStatusExpired,
    /// Partial Withdrawal
    #[c(remote = "PartialWithdrawal")]
    OrderStatusPartialWithdrawal,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::OrderTag")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderTag {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTagUnknown,
    /// Normal Order
    #[c(remote = "Normal")]
    OrderTagNormal,
    /// Long term Order
    #[c(remote = "LongTerm")]
    OrderTagLongTerm,
    /// Grey Order
    #[c(remote = "Grey")]
    OrderTagGrey,
    /// Force Selling
    #[c(remote = "MarginCall")]
    OrderTagMarginCall,
    /// OTC
    #[c(remote = "Offline")]
    OrderTagOffline,
    /// Option Exercise Long
    #[c(remote = "Creditor")]
    OrderTagCreditor,
    /// Option Exercise Short
    #[c(remote = "Debtor")]
    OrderTagDebtor,
    /// Wavier Of Option Exercise
    #[c(remote = "NonExercise")]
    OrderTagNonExercise,
    /// Trade Allocation
    #[c(remote = "AllocatedSub")]
    OrderTagAllocatedSub,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longportwhale::trade::TriggerStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTriggerStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    TriggerStatusUnknown,
    /// Deactive
    #[c(remote = "Deactive")]
    TriggerStatusDeactive,
    /// Active
    #[c(remote = "Active")]
    TriggerStatusActive,
    /// Released
    #[c(remote = "Released")]
    TriggerStatusReleased,
}
