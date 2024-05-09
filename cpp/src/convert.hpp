#pragma once

#include "longportwhale.h"
#include "types.hpp"
#include <stdexcept>

namespace longportwhale {
namespace convert {

using longportwhale::trade::OrderSide;
using longportwhale::trade::OrderStatus;
using longportwhale::trade::OrderTag;
using longportwhale::trade::OrderType;
using longportwhale::trade::PushOrderChanged;
using longportwhale::trade::TopicType;
using longportwhale::trade::TriggerStatus;

inline lb_language_t
convert(Language language)
{
  switch (language) {
    case Language::ZH_CN:
      return Language_ZH_CN;
    case Language::ZH_HK:
      return Language_ZH_HK;
    case Language::EN:
      return Language_EN;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_topic_type_t
convert(TopicType ty)
{
  switch (ty) {
    case TopicType::Private:
      return TopicPrivate;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderStatus
convert(lb_order_status_t status)
{
  switch (status) {
    case OrderStatusUnknown:
      return OrderStatus::Unknown;
    case OrderStatusNotReported:
      return OrderStatus::NotReported;
    case OrderStatusReplacedNotReported:
      return OrderStatus::ReplacedNotReported;
    case OrderStatusProtectedNotReported:
      return OrderStatus::ProtectedNotReported;
    case OrderStatusVarietiesNotReported:
      return OrderStatus::VarietiesNotReported;
    case OrderStatusFilled:
      return OrderStatus::Filled;
    case OrderStatusWaitToNew:
      return OrderStatus::WaitToNew;
    case OrderStatusNew:
      return OrderStatus::New;
    case OrderStatusWaitToReplace:
      return OrderStatus::WaitToReplace;
    case OrderStatusPendingReplace:
      return OrderStatus::PendingReplace;
    case OrderStatusReplaced:
      return OrderStatus::Replaced;
    case OrderStatusPartialFilled:
      return OrderStatus::PartialFilled;
    case OrderStatusWaitToCancel:
      return OrderStatus::WaitToCancel;
    case OrderStatusPendingCancel:
      return OrderStatus::PendingCancel;
    case OrderStatusRejected:
      return OrderStatus::Rejected;
    case OrderStatusCanceled:
      return OrderStatus::Canceled;
    case OrderStatusExpired:
      return OrderStatus::Expired;
    case OrderStatusPartialWithdrawal:
      return OrderStatus::PartialWithdrawal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_status_t
convert(OrderStatus status)
{
  switch (status) {
    case OrderStatus::Unknown:
      return OrderStatusUnknown;
    case OrderStatus::NotReported:
      return OrderStatusNotReported;
    case OrderStatus::ReplacedNotReported:
      return OrderStatusReplacedNotReported;
    case OrderStatus::ProtectedNotReported:
      return OrderStatusProtectedNotReported;
    case OrderStatus::VarietiesNotReported:
      return OrderStatusVarietiesNotReported;
    case OrderStatus::Filled:
      return OrderStatusFilled;
    case OrderStatus::WaitToNew:
      return OrderStatusWaitToNew;
    case OrderStatus::New:
      return OrderStatusNew;
    case OrderStatus::WaitToReplace:
      return OrderStatusWaitToReplace;
    case OrderStatus::PendingReplace:
      return OrderStatusPendingReplace;
    case OrderStatus::Replaced:
      return OrderStatusReplaced;
    case OrderStatus::PartialFilled:
      return OrderStatusPartialFilled;
    case OrderStatus::WaitToCancel:
      return OrderStatusWaitToCancel;
    case OrderStatus::PendingCancel:
      return OrderStatusPendingCancel;
    case OrderStatus::Rejected:
      return OrderStatusRejected;
    case OrderStatus::Canceled:
      return OrderStatusCanceled;
    case OrderStatus::Expired:
      return OrderStatusExpired;
    case OrderStatus::PartialWithdrawal:
      return OrderStatusPartialWithdrawal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderSide
convert(lb_order_side_t side)
{
  switch (side) {
    case OrderSideUnknown:
      return OrderSide::Unknown;
    case OrderSideBuy:
      return OrderSide::Buy;
    case OrderSideSell:
      return OrderSide::Sell;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_side_t
convert(OrderSide side)
{
  switch (side) {
    case OrderSide::Unknown:
      return OrderSideUnknown;
    case OrderSide::Buy:
      return OrderSideBuy;
    case OrderSide::Sell:
      return OrderSideSell;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderType
convert(lb_order_type_t ty)
{
  switch (ty) {
    case OrderTypeUnknown:
      return OrderType::Unknown;
    case OrderTypeLO:
      return OrderType::LO;
    case OrderTypeELO:
      return OrderType::ELO;
    case OrderTypeMO:
      return OrderType::MO;
    case OrderTypeAO:
      return OrderType::AO;
    case OrderTypeALO:
      return OrderType::ALO;
    case OrderTypeODD:
      return OrderType::ODD;
    case OrderTypeLIT:
      return OrderType::LIT;
    case OrderTypeMIT:
      return OrderType::MIT;
    case OrderTypeTSLPAMT:
      return OrderType::TSLPAMT;
    case OrderTypeTSLPPCT:
      return OrderType::TSLPPCT;
    case OrderTypeTSMAMT:
      return OrderType::TSMAMT;
    case OrderTypeTSMPCT:
      return OrderType::TSMPCT;
    case OrderTypeSLO:
      return OrderType::SLO;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_type_t
convert(OrderType ty)
{
  switch (ty) {
    case OrderType::Unknown:
      return OrderTypeUnknown;
    case OrderType::LO:
      return OrderTypeLO;
    case OrderType::ELO:
      return OrderTypeELO;
    case OrderType::MO:
      return OrderTypeMO;
    case OrderType::AO:
      return OrderTypeAO;
    case OrderType::ALO:
      return OrderTypeALO;
    case OrderType::ODD:
      return OrderTypeODD;
    case OrderType::LIT:
      return OrderTypeLIT;
    case OrderType::MIT:
      return OrderTypeMIT;
    case OrderType::TSLPAMT:
      return OrderTypeTSLPAMT;
    case OrderType::TSLPPCT:
      return OrderTypeTSLPPCT;
    case OrderType::TSMAMT:
      return OrderTypeTSMAMT;
    case OrderType::TSMPCT:
      return OrderTypeTSMPCT;
    case OrderType::SLO:
      return OrderTypeSLO;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderTag
convert(lb_order_tag_t tag)
{
  switch (tag) {
    case OrderTagUnknown:
      return OrderTag::Unknown;
    case OrderTagNormal:
      return OrderTag::Normal;
    case OrderTagLongTerm:
      return OrderTag::LongTerm;
    case OrderTagGrey:
      return OrderTag::Grey;
    case OrderTagMarginCall:
      return OrderTag::Grey;
    case OrderTagOffline:
      return OrderTag::Offline;
    case OrderTagCreditor:
      return OrderTag::Creditor;
    case OrderTagDebtor:
      return OrderTag::Debtor;
    case OrderTagNonExercise:
      return OrderTag::NonExercise;
    case OrderTagAllocatedSub:
      return OrderTag::AllocatedSub;
    /// Trade Allocation
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline TriggerStatus
convert(lb_trigger_status_t status)
{
  switch (status) {
    case TriggerStatusUnknown:
      return TriggerStatus::Unknown;
    case TriggerStatusDeactive:
      return TriggerStatus::Deactive;
    case TriggerStatusActive:
      return TriggerStatus::Active;
    case TriggerStatusReleased:
      return TriggerStatus::Released;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline PushOrderChanged
convert(const lb_push_order_changed_t* info)
{
  return PushOrderChanged{
    convert(info->side),
    info->stock_name,
    info->submitted_quantity,
    info->symbol,
    convert(info->order_type),
    Decimal(info->submitted_price),
    info->executed_quantity,
    info->executed_price ? std::optional{ Decimal(info->executed_price) }
                         : std::nullopt,
    info->order_id,
    info->currency,
    convert(info->status),
    info->submitted_at,
    info->updated_at,
    info->trigger_price ? std::optional{ Decimal(info->trigger_price) }
                        : std::nullopt,
    info->msg,
    convert(info->tag),
    info->trigger_status ? std::optional{ convert(*info->trigger_status) }
                         : std::nullopt,
    info->trigger_at ? std::optional{ *info->trigger_at } : std::nullopt,
    info->trailing_amount ? std::optional{ Decimal(info->trailing_amount) }
                          : std::nullopt,
    info->trailing_percent ? std::optional{ Decimal(info->trailing_percent) }
                           : std::nullopt,
    info->limit_offset ? std::optional{ Decimal(info->limit_offset) }
                       : std::nullopt,
    info->account_no,
    info->last_share ? std::optional{ Decimal(info->last_share) }
                     : std::nullopt,
    info->last_price ? std::optional{ Decimal(info->last_price) }
                     : std::nullopt,
    info->remark,
  };
}

} // namespace convert
} // namespace longport
