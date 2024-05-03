#pragma once

#include <optional>
#include <string>
#include <utility>

#include "async_result.hpp"
#include "status.hpp"
#include "types.hpp"

typedef struct lb_config_t lb_config_t;

namespace longportwhale {

class Config
{
private:
  lb_config_t* config_;

public:
  Config();
  Config(lb_config_t* config);
  Config(const Config&) = delete;
  Config(Config&& other);

  /** Config
   *
   * @param app_key App key
   * @param app_secret App secret
   * @param access_token Access token
   * @param http_url HTTP  endpoint url (Default:
   * https://api.longbridgewhale.com)
   * @param trade_ws_url Trade websocket endpoint url (Default:
   * wss://openapi-trade.longportapp.com)
   * @param language Language identifer (Default: Language::EN)
   */
  Config(const std::string& app_key,
         const std::string& app_secret,
         const std::string& access_token,
         const std::optional<std::string>& http_url,
         const std::optional<std::string>& trade_ws_url,
         const std::optional<Language>& language);

  ~Config();

  operator const lb_config_t*() const;
  static Status from_env(Config& config);
};

} // namespace longportwhale
