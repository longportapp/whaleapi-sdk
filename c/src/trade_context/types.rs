use std::os::raw::c_char;

use longportwhale::trade::{OrderSide, OrderStatus, OrderTag, OrderType, PushOrderChanged};
use time::OffsetDateTime;

use crate::{
    trade_context::enum_types::{COrderSide, COrderStatus, COrderTag, COrderType, CTriggerStatus},
    types::{CDecimal, CString, ToFFI},
};

/// Order changed message
#[repr(C)]
pub struct CPushOrderChanged {
    /// Order side
    pub side: COrderSide,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub submitted_quantity: i64,
    /// Order symbol
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Submitted price
    pub submitted_price: *const CDecimal,
    /// Executed quantity
    pub executed_quantity: i64,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Order ID
    pub order_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Submitted time
    pub submitted_at: i64,
    /// Last updated time
    pub updated_at: i64,
    /// Order trigger price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// Trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// Limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Account no
    pub account_no: *const c_char,
    /// Last share (maybe null)
    pub last_share: *const CDecimal,
    /// Last price (maybe null)
    pub last_price: *const CDecimal,
    /// Remark message
    pub remark: *const c_char,
}

pub struct CPushOrderChangedOwned {
    side: OrderSide,
    stock_name: CString,
    submitted_quantity: i64,
    symbol: CString,
    order_type: OrderType,
    submitted_price: CDecimal,
    executed_quantity: i64,
    executed_price: Option<CDecimal>,
    order_id: CString,
    currency: CString,
    status: OrderStatus,
    submitted_at: i64,
    updated_at: i64,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    trigger_status: Option<CTriggerStatus>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    account_no: CString,
    last_share: Option<CDecimal>,
    last_price: Option<CDecimal>,
    /// Remark message
    pub remark: CString,
}

impl From<PushOrderChanged> for CPushOrderChangedOwned {
    fn from(order_changed: PushOrderChanged) -> Self {
        let PushOrderChanged {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
            last_share,
            last_price,
            remark,
        } = order_changed;
        CPushOrderChangedOwned {
            side,
            stock_name: stock_name.into(),
            submitted_quantity,
            symbol: symbol.into(),
            order_type,
            submitted_price: submitted_price.into(),
            executed_quantity,
            executed_price: executed_price.map(Into::into),
            order_id: order_id.into(),
            currency: currency.into(),
            status,
            submitted_at: submitted_at.unix_timestamp(),
            updated_at: updated_at.unix_timestamp(),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            trigger_status: trigger_status.map(Into::into),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            account_no: account_no.into(),
            last_share: last_share.map(Into::into),
            last_price: last_price.map(Into::into),
            remark: remark.into(),
        }
    }
}

impl ToFFI for CPushOrderChangedOwned {
    type FFIType = CPushOrderChanged;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushOrderChangedOwned {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
            last_share,
            last_price,
            remark,
        } = self;
        CPushOrderChanged {
            side: (*side).into(),
            stock_name: stock_name.to_ffi_type(),
            submitted_quantity: *submitted_quantity,
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            submitted_price: submitted_price.to_ffi_type(),
            executed_quantity: *executed_quantity,
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            order_id: order_id.to_ffi_type(),
            currency: currency.to_ffi_type(),
            status: (*status).into(),
            submitted_at: *submitted_at,
            updated_at: *updated_at,
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            account_no: account_no.to_ffi_type(),
            last_share: last_share
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            last_price: last_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            remark: remark.to_ffi_type(),
        }
    }
}
