#include <iostream>
#include <longportwhale.hpp>
#include <optional>
#include <string>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longportwhale;

const std::string test_account = "L6VQEU00121996";
std::optional<trade::TradeContext> g_ctx;

void
do_trade(HttpClient& cli, Config& conf)
{
  trade::TradeContext::create(
    conf, [&cli](AsyncResult<trade::TradeContext, void> res) {
      if (!res) {
        std::cout << "failed: " << res.status().message() << std::endl;
        return;
      }

      g_ctx = res.context();
      trade::TradeContext ctx = *g_ctx;

      std::vector<trade::TopicType> topics = { trade::TopicType::Private };

      // Set event handler BEFORE subscribing
      ctx.set_on_order_changed(
        [](PushEvent<trade::TradeContext, trade::PushOrderChanged> order) {
          std::cout << "ORDER EVENT RECEIVED!" << std::endl;
          std::cout << "Order Side: " << (int)order->side << std::endl;
          std::cout << "Stock Name: " << order->stock_name << std::endl;
          std::cout << "Submitted Quantity: " << order->submitted_quantity
                    << std::endl;
          std::cout << "Symbol: " << order->symbol << std::endl;
          std::cout << "Order Type: " << (int)order->order_type << std::endl;
          std::cout << "Submitted Price: " << order->submitted_price.to_double()
                    << std::endl;
          std::cout << "Executed Quantity: " << order->executed_quantity
                    << std::endl;
          std::cout << "Order ID: " << order->order_id << std::endl;
          std::cout << "Status: " << (int)order->status << std::endl;
        });

      // Subscribe and THEN submit order
      ctx.subscribe(
        topics, [ctx, &cli](AsyncResult<trade::TradeContext, void> res) {
          if (!res) {
            std::cout << "subscribe failed:" << res.status().message()
                      << std::endl;
            return;
          }

          std::cout << "Subscribe success! Now submitting order..."
                    << std::endl;

          std::string body = R"({
            "symbol": "700.HK",
            "order_type": "MO", 
            "side": "Buy",
            "submitted_quantity": "100",
            "time_in_force": "Day",
            "account_no": ")";

          body = body + test_account + "\"}";

          cli.request("post",
                      "/v1/whaleapi/trade/order",
                      std::nullopt,
                      std::make_optional(body),
                      [](AsyncResult<void*, HttpResult> res) {
                        if (!res) {
                          std::cout << "order submission failed: "
                                    << res.status().message() << std::endl;
                          return;
                        }
                        std::cout << "trade response:" << res->response_body
                                  << std::endl;
                      });
        });
    });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  HttpClient http_cli;
  Status status = http_cli.from_env();
  if (!status) {
    std::cout << "failed to load http configuration from environment: "
              << status.message() << std::endl;
    return -1;
  }

  Config* conf = new Config();
  Status conf_status = Config::from_env(*conf);

  if (!conf_status) {
    std::cout << "failed to load http configuration from environment: "
              << conf_status.message() << std::endl;
    return -1;
  }

  do_trade(http_cli, *conf);

  std::cin.get();
  return 0;
}
