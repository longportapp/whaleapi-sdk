#include "trade_context.hpp"
#include "convert.hpp"

namespace longportwhale {
namespace trade {

using longportwhale::convert::convert;

TradeContext::TradeContext()
  : ctx_(nullptr)
{
}

TradeContext::TradeContext(const lb_trade_context_t* ctx)
{

  ctx_ = ctx;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
}

TradeContext::TradeContext(const TradeContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
}

TradeContext::TradeContext(TradeContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

TradeContext::~TradeContext()
{
  if (ctx_) {
    lb_trade_context_release(ctx_);
  }
}

TradeContext&
TradeContext::operator=(const TradeContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
  return *this;
}

size_t
TradeContext::ref_count() const
{
  return ctx_ ? lb_trade_context_ref_count(ctx_) : 0;
}

void
TradeContext::create(const Config& config,
                     AsyncCallback<TradeContext, void> callback)
{
  lb_trade_context_new(
    config,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      auto* ctx_ptr = (lb_trade_context_t*)res->ctx;
      TradeContext ctx(ctx_ptr);
      if (ctx_ptr) {
        lb_trade_context_release(ctx_ptr);
      }
      (*callback_ptr)(
        AsyncResult<TradeContext, void>(ctx, Status(res->error), nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::subscribe(const std::vector<TopicType>& topics,
                        AsyncCallback<TradeContext, void> callback) const
{
  std::vector<lb_topic_type_t> topics2;
  std::transform(topics.cbegin(),
                 topics.cend(),
                 std::back_inserter(topics2),
                 [](auto topic) { return convert(topic); });

  lb_trade_context_subscribe(
    ctx_,
    topics2.data(),
    topics2.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::unsubscribe(const std::vector<TopicType>& topics,
                          AsyncCallback<TradeContext, void> callback) const
{
  std::vector<lb_topic_type_t> topics2;
  std::transform(topics.cbegin(),
                 topics.cend(),
                 std::back_inserter(topics2),
                 [](auto topic) { return convert(topic); });

  lb_trade_context_unsubscribe(
    ctx_,
    topics2.data(),
    topics2.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::set_on_order_changed(
  PushCallback<TradeContext, PushOrderChanged> callback) const
{
  lb_trade_context_set_on_order_changed(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<TradeContext, PushOrderChanged>(userdata);
      PushOrderChanged event2 = convert(event);
      (*callback_ptr)(
        PushEvent<TradeContext, PushOrderChanged>(TradeContext(ctx), &event2));
    },
    new PushCallback<TradeContext, PushOrderChanged>(callback),
    [](auto p) { delete (PushCallback<TradeContext, PushOrderChanged>*)p; });
}

} // namespace trade
} // namespace longportwhale
