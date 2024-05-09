#include <iostream>
#include <longportwhale.hpp>
#include <string>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longportwhale;

const std::string test_account = "L6VQEU00121996";

void
do_trade(HttpClient& cli, Config& conf)
{
  trade::TradeContext::create(conf, [](auto res) {
    if (!res) {
      std::cout << "failed: " << res.status().message() << std::endl;
      return;
    }

    trade::TradeContext ctx = res.context();

    std::vector<trade::TopicType> topics = { trade::TopicType::Private };

    ctx.subscribe(topics, [](auto res) {
      if (!res) {
        std::cout << "subscribe failed:" << res.status().message() << std::endl;
      }
    });

    ctx.set_on_order_changed([](auto order) {
      std::cout << "Order Side: " << &order->side << std::endl;
      std::cout << "Stock Name: " << order->stock_name << std::endl;
      std::cout << "Submitted Quantity: " << order->submitted_quantity
                << std::endl;
      std::cout << "Symbol: " << order->symbol << std::endl;
      std::cout << "Order Type: " << &order->order_type << std::endl;
      std::cout << "Submitted Price: " << &order->submitted_price << std::endl;
      std::cout << "Executed Quantity: " << &order->executed_quantity
                << std::endl;
      // std::cout << "Executed Price: ";
      // if (order->executed_price)
      //   std::cout << *order->executed_price;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Order ID: " << order->order_id << std::endl;
      // std::cout << "Currency: " << order->currency << std::endl;
      // std::cout << "Order Status: " << order->status << std::endl;
      // std::cout << "Submitted At: " << order->submitted_at << std::endl;
      // std::cout << "Updated At: " << order->updated_at << std::endl;
      // std::cout << "Trigger Price: ";
      // if (order->trigger_price)
      //   std::cout << *order->trigger_price;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Message: " << order->msg << std::endl;
      // std::cout << "Order Tag: " << order->tag << std::endl;
      // std::cout << "Trigger Status: ";
      // if (order->trigger_status)
      //   std::cout << *order->trigger_status;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Trigger At: ";
      // if (order->trigger_at)
      //   std::cout << *order->trigger_at;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Trailing Amount: ";
      // if (order->trailing_amount)
      //   std::cout << *order->trailing_amount;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Trailing Percent: ";
      // if (order->trailing_percent)
      //   std::cout << *order->trailing_percent;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Limit Offset: ";
      // if (order->limit_offset)
      //   std::cout << *order->limit_offset;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Account No: " << order->account_no << std::endl;
      // std::cout << "Last Share: ";
      // if (order->last_share)
      //   std::cout << *order->last_share;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Last Price: ";
      // if (order->last_price)
      //   std::cout << *order->last_price;
      // else
      //   std::cout << "None";
      // std::cout << std::endl;
      // std::cout << "Remark: " << order->remark << std::endl;
    });
  });

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
              [](auto res) {
                if (!res) {
                  std::cout << "failed: " << res.status().message()
                            << std::endl;
                  return;
                }
                std::cout << "trade response:" << res->response_body
                          << std::endl;
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
