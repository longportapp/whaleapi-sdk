# LongPort OpenAPI SDK for C++

`longport` provides an easy-to-use interface for invokes [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Quickstart

_Install LongPort OpenAPI SDK_

[`Download C++ SDK`]([`Download C SDK`](https://github.com/longportapp/openapi-sdk/releases))

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longportwhale::quote;

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << status.message() << std::endl;
    return -1;
  }

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << res.status().message()
                << std::endl;
      return;
    }

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };
    res.context().quote(symbols, [](auto res) {
      if (!res) {
        std::cout << "failed to get quote: " << res.status().message()
                  << std::endl;
        return;
      }

      for (auto it = res->cbegin(); it != res->cend(); ++it) {
        std::cout << it->symbol << " timestamp=" << it->timestamp
                  << " last_done=" << (double)it->last_done
                  << " prev_close=" << (double)it->prev_close
                  << " open=" << (double)it->open
                  << " high=" << (double)it->high << " low=" << (double)it->low
                  << " volume=" << it->volume << " turnover=" << it->turnover
                  << std::endl;
      }
    });
  });

  std::cin.get();
  return 0;
}
```

## Quote API _(Subscribe quotes)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longportwhale::quote;

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << status.message() << std::endl;
    return -1;
  }

  QuoteContext ctx;

  QuoteContext::create(config, [&](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << res.status().message()
                << std::endl;
      return;
    }

    ctx = res.context();

    res.context().set_on_quote([](auto event) {
      std::cout << event->symbol << " timestamp=" << event->timestamp
                << " last_done=" << (double)event->last_done
                << " open=" << (double)event->open
                << " high=" << (double)event->high
                << " low=" << (double)event->low << " volume=" << event->volume
                << " turnover=" << event->turnover << std::endl;
    });

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };

    res.context().subscribe(symbols, SubFlags::QUOTE(), true, [](auto res) {
      if (!res) {
        std::cout << "failed to subscribe quote: " << res.status().message()
                  << std::endl;
        return;
      }
    });
  });

  std::cin.get();
  return 0;
}
```

## Trade API _(Submit order)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longportwhale::trade;

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << status.message() << std::endl;
    return -1;
  }

  TradeContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create trade context: " << res.status().message()
                << std::endl;
      return;
    }

    SubmitOrderOptions opts{
      "700.HK",     OrderType::LO,        OrderSide::Buy,
      200,          TimeInForceType::Day, Decimal(50.0),
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt,
    };
    res.context().submit_order(opts, [](auto res) {
      if (!res) {
        std::cout << "failed to submit order: " << res.status().message()
                  << std::endl;
        return;
      }
      std::cout << "order id: " << res->order_id << std::endl;
    });
  });

  std::cin.get();
  return 0;
}
```

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
