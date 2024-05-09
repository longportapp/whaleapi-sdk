#include "longportwhale.h"
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

const char* test_account = "L6VQEU00121996";

// submit order example
void
do_trade(lb_http_client_t*, lb_config_t*);

// query asset example
// void
// query_asset(*lb_http_client_t* cli);

// create user example
// void
// create_user(*lb_http_client_t* cli);

// handle response
void
on_response(const struct lb_async_result_t*);

// error handle
void
on_error(const struct lb_async_result_t*);

// trade context created callback
void
on_trade_context_created(const struct lb_async_result_t*);

void
on_order_event(const struct lb_trade_context_t*,
               const struct lb_push_order_changed_t*,
               void*);

void
print_decimal(const lb_decimal_t*);

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;

  lb_config_t* config = lb_config_from_env(&err);
  if (err) {
    printf("failed to create config from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  lb_http_client_t* http_client = lb_http_client_from_env(&err);
  if (err) {
    printf("failed to create http client from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  do_trade(http_client, config);

  getchar();
  lb_config_free(config);
  lb_http_client_free(http_client);
  return 0;
}

void
do_trade(lb_http_client_t* cli, lb_config_t* conf)
{

  lb_trade_context_new(conf, on_trade_context_created, NULL);
  lb_http_header_t headers[] = { { "accept-language", "zh-CN" } };

  char* body = "{"
               "\"symbol\":\"700.HK\","
               "\"order_type\":\"MO\","
               "\"side\": \"Buy\","
               "\"submitted_quantity\":\"100\","
               "\"time_in_force\":\"Day\","
               "\"account_no\":\"";

  strcat(body, test_account);
  strcat(body, "\"}");

  lb_http_client_request(
    cli, "post", "/v1/whaleapi/trade/order", headers, body, on_response, NULL);
}

void
on_response(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_http_result_t* resp = (const lb_http_result_t*)res->data;
  printf("%s\n", lb_http_result_response_body(resp));
}

void
on_error(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to subscribe topic: %s\n", lb_error_message(res->error));
    return;
  }
}

void
on_trade_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create trade context: %s\n",
           lb_error_message(res->error));
    return;
  }

  *((const lb_trade_context_t**)res->userdata) = res->ctx;

  const enum lb_topic_type_t topics[] = { TopicPrivate };

  lb_trade_context_subscribe(res->ctx, topics, 1, NULL, NULL);
  lb_trade_context_set_on_order_changed(res->ctx, on_order_event, NULL, NULL);
}

void
on_order_event(const struct lb_trade_context_t* ctx,
               const struct lb_push_order_changed_t* order,
               void* user_data)
{
  if (order == NULL) {
    printf("change event is NULL\n");
    return;
  }

  printf("Order Side: %d\n", order->side);
  printf("Stock Name: %s\n", order->stock_name);
  printf("Submitted Quantity: %" PRId64 "\n", order->submitted_quantity);
  printf("Symbol: %s\n", order->symbol);
  printf("Order Type: %d\n", order->order_type);
  printf("Submitted Price: ");
  print_decimal(order->submitted_price);
  printf("Executed Quantity: %" PRId64 "\n", order->executed_quantity);
  printf("Executed Price: ");
  print_decimal(order->executed_price);
  printf("Order ID: %s\n", order->order_id);
  printf("Currency: %s\n", order->currency);
  printf("Order Status: %d\n", order->status);
  printf("Submitted At: %" PRId64 "\n", order->submitted_at);
  printf("Updated At: %" PRId64 "\n", order->updated_at);
  printf("Trigger Price: ");
  print_decimal(order->trigger_price);
  printf("Message: %s\n", order->msg);
  printf("Order Tag: %d\n", order->tag);
  if (order->trigger_status) {
    printf("Trigger Status: %d\n", *order->trigger_status);
  }
  if (order->trigger_at) {
    printf("Trigger At: %" PRId64 "\n", *order->trigger_at);
  }
  printf("Trailing Amount: ");
  print_decimal(order->trailing_amount);
  printf("Trailing Percent: ");
  print_decimal(order->trailing_percent);
  printf("Limit Offset Amount: ");
  print_decimal(order->limit_offset);
  printf("Account No: %s\n", order->account_no);
  printf("Last Share: ");
  print_decimal(order->last_share);
  printf("Last Price: ");
  print_decimal(order->last_price);
  printf("Remark: %s\n", order->remark);
}

void
print_decimal(const lb_decimal_t* decimal)
{
  if (decimal) {
    printf("Value: %f", lb_decimal_to_double(decimal));
  } else {
    printf("NULL\n");
  }
}
