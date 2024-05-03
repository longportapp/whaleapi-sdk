#pragma once

#include "decimal.hpp"
#include <optional>

namespace longportwhale {

struct Date
{
  int32_t year;
  uint8_t month;
  uint8_t day;
};

struct Time
{
  uint8_t hour;
  uint8_t minute;
  uint8_t second;
};

struct DateTime
{
  Date date;
  Time time;
};

/// Language identifer
enum class Language
{
  /// zh-CN
  ZH_CN,
  /// zh-HK
  ZH_HK,
  /// en
  EN,
};

/// Market
enum class Market
{
  /// Unknown
  Unknown,
  /// US market
  US,
  /// HK market
  HK,
  /// CN market
  CN,
  /// SG market
  SG,
};

namespace trade {

/// Topic type
enum class TopicType
{
  /// Private notification for trade
  Private,
};

/// Order status
enum class OrderStatus
{
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
};

/// Order side
enum class OrderSide
{
  /// Unknown
  Unknown,
  /// Buy
  Buy,
  /// Sell
  Sell,
};

/// Order type
enum class OrderType
{
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
};

/// Order tag
enum class OrderTag
{
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
};

/// Trigger status
enum class TriggerStatus
{
  /// Unknown
  Unknown,
  /// Deactive
  Deactive,
  /// Active
  Active,
  /// Released
  Released,
};

/// Order changed message
struct PushOrderChanged
{
  /// Order side
  OrderSide side;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  int64_t submitted_quantity;
  /// Order symbol
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Submitted price
  Decimal submitted_price;
  /// Executed quantity
  int64_t executed_quantity;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Order ID
  std::string order_id;
  /// Currency
  std::string currency;
  /// Order status
  OrderStatus status;
  /// Submitted time
  int64_t submitted_at;
  /// Last updated time
  int64_t updated_at;
  /// Order trigger price
  std::optional<Decimal> trigger_price;
  /// Rejected message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// Trailing amount
  std::optional<Decimal> trailing_amount;
  /// Trailing percent
  std::optional<Decimal> trailing_percent;
  /// Limit offset amount
  std::optional<Decimal> limit_offset;
  /// Account no
  std::string account_no;
  /// Last share
  std::optional<Decimal> last_share;
  /// Last price
  std::optional<Decimal> last_price;
  /// Remark message
  std::string remark;
};
} // namespace trade

} // namespace longportwhale
