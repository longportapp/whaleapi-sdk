#include "types.hpp"
#include <iostream>

namespace longportwhale {
namespace trade {

std::ostream&
operator<<(std::ostream& os, const OrderStatus& status)
{
  switch (status) {
      /// Not reported
    case OrderStatus::NotReported:
      os << "NotReported";
      break;

      /// Not reported (Replaced Order)
    case OrderStatus::ReplacedNotReported:
      os << "ReplacedNotReported";
      break;
      /// Not reported (Protected Order)
    case OrderStatus::ProtectedNotReported:
      os << "ProtectedNotReported";
      break;
      /// Not reported (Conditional Order)
    case OrderStatus::VarietiesNotReported:
      os << "VarietiesNotReported";
      break;
      /// Filled
    case OrderStatus::Filled:
      os << "Filled";
      break;
      /// Wait To New
    case OrderStatus::WaitToNew:
      os << "WaitToNew";
      /// New
    case OrderStatus::New:
      os << "New";
      break;
      /// Wait To Replace
    case OrderStatus::WaitToReplace:
      os << "WaitToReplace";
      break;
    case OrderStatus::PendingReplace:
      os << "PendingReplace";
      break;
      /// Replaced
    case OrderStatus::Replaced:
      os << "Replaced";
      break;
      /// Partial Filled
    case OrderStatus::PartialFilled:
      os << "PartialFilled";
      break;
      /// Wait To Cancel
    case OrderStatus::WaitToCancel:
      os << "WaitToCancel";
      break;
      /// Pending Cancel
    case OrderStatus::PendingCancel:
      os << "PendingCancel";
      break;
      /// Rejected
    case OrderStatus::Rejected:
      os << "Rejected";
      break;

    case OrderStatus::Canceled:
      os << "Canceled";
      break;

    case OrderStatus::Expired:
      os << "Expired";
      break;

    case OrderStatus::PartialWithdrawal:
      os << "PartialWithdrawal";
      break;
    default:
      os << "Unknow";
  }

  return os;
}

std::ostream&
operator<<(std::ostream& os, const OrderSide& side)
{
  switch (side) {
    case OrderSide::Buy:
      os << "Buy";
      break;
    case OrderSide::Sell:
      os << "Sell";
      break;
    default:
      os << "Unknown";
      break;
  }
  return os;
}

std::ostream&
operator<<(std::ostream& os, const OrderTag& type)
{
  switch (type) {
    case OrderTag::Unknown:
      os << "Unknown";
      break;
    case OrderTag::Normal:
      os << "Normal Order";
      break;
    case OrderTag::LongTerm:
      os << "Long-term Order";
      break;
    case OrderTag::Grey:
      os << "Grey Order";
      break;
    case OrderTag::MarginCall:
      os << "Force Selling";
      break;
    case OrderTag::Offline:
      os << "OTC";
      break;
    case OrderTag::Creditor:
      os << "Option Exercise Long";
      break;
    case OrderTag::Debtor:
      os << "Option Exercise Short";
      break;
    case OrderTag::NonExercise:
      os << "Waiver Of Option Exercise";
      break;
    case OrderTag::AllocatedSub:
      os << "Trade Allocation";
      break;
  }
  return os;
}

} // namespace trade
} // namespace longbridgewhale
