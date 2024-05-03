#include <iostream>
#include <longportwhale.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longportwhale;

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
  trade::TradeContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << res.status().message()
                << std::endl;
      return;
    }
  });

  std::cin.get();
  return 0;
}
