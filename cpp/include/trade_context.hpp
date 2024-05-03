#pragma once

#include "async_result.hpp"
#include "config.hpp"
#include "push.hpp"
#include "types.hpp"

typedef struct lb_trade_context_t lb_trade_context_t;

namespace longportwhale {
namespace trade {

/// Trade context
class TradeContext
{
private:
  const lb_trade_context_t* ctx_;

public:
  TradeContext();
  TradeContext(const lb_trade_context_t* ctx);
  TradeContext(const TradeContext& ctx);
  TradeContext(TradeContext&& ctx);
  ~TradeContext();

  TradeContext& operator=(const TradeContext& ctx);

  size_t ref_count() const;

  static void create(const Config& config,
                     AsyncCallback<TradeContext, void> callback);

  /// Subscribe
  void subscribe(const std::vector<TopicType>& topics,
                 AsyncCallback<TradeContext, void> callback) const;

  /// Unsubscribe
  void unsubscribe(const std::vector<TopicType>& topics,
                   AsyncCallback<TradeContext, void> callback) const;

  /// Set order changed callback, after receiving the order changed event, it
  /// will call back to this function.
  void set_on_order_changed(
    PushCallback<TradeContext, PushOrderChanged> callback) const;
};

} // namespace trade
} // namespace longportwhale
