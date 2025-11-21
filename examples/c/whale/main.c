#include "longportwhale.h"
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#include <unistd.h>
#endif

const char* test_account = "L6VQEU00121996";
volatile int is_finished = 0;

// Forward declarations
void
do_trade(lb_http_client_t*, lb_config_t*);
void
on_response(const struct lb_async_result_t*);
void
on_subscribe_complete(const struct lb_async_result_t*);
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

  printf("Starting trade example...\n");
  do_trade(http_client, config);

  printf("Waiting for order event...\n");

  // Wait up to 30 seconds for the process to complete
  int timeout = 30;
  while (!is_finished && timeout > 0) {
#ifdef WIN32
    Sleep(1000);
#else
    sleep(1);
#endif
    timeout--;
    if (timeout % 5 == 0) {
      printf("Waiting... %d seconds remaining\n", timeout);
    }
  }

  if (!is_finished) {
    printf("Timeout waiting for order event.\n");
  } else {
    printf("Order event received. Exiting.\n");
  }

  lb_config_free(config);
  lb_http_client_free(http_client);
  return 0;
}

void
do_trade(lb_http_client_t* cli, lb_config_t* conf)
{
  printf("Creating trade context...\n");
  // Pass http_client as userdata so callback can use it
  lb_trade_context_new(conf, on_trade_context_created, cli);
}

void
on_response(const struct lb_async_result_t* res)
{
  printf("on_response called!\n");
  fflush(stdout);
  if (res->error) {
    printf("ERROR - Order submission failed: %s\n",
           lb_error_message(res->error));
    return;
  }

  const lb_http_result_t* resp = (const lb_http_result_t*)res->data;
  printf("ORDER RESPONSE: %s\n", lb_http_result_response_body(resp));
}

void
on_subscribe_complete(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("ERROR - Subscription failed: %s\n", lb_error_message(res->error));
    return;
  }

  printf("Subscription complete! Now submitting order...\n");

  // Get http_client from userdata
  lb_http_client_t* cli = (lb_http_client_t*)res->userdata;
  if (cli == NULL) {
    printf("ERROR: http_client is NULL in on_subscribe_complete\n");
    return;
  }

  lb_http_header_t headers[] = { { "accept-language", "zh-CN" },
                                 { NULL, NULL } };

  char body[512];
  snprintf(body,
           sizeof(body),
           "{"
           "\"symbol\":\"700.HK\","
           "\"order_type\":\"MO\","
           "\"side\":\"Buy\","
           "\"submitted_quantity\":\"100\","
           "\"time_in_force\":\"Day\","
           "\"account_no\":\"%s\""
           "}",
           test_account);

  printf("Sending order request...\n");
  printf("Body: %s\n", body);
  fflush(stdout);

  lb_http_client_request(
    cli, "POST", "/v1/whaleapi/trade/order", headers, body, on_response, NULL);
  printf("Request sent.\n");
  fflush(stdout);
}

void
on_trade_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("ERROR - TradeContext creation failed: %s\n",
           lb_error_message(res->error));
    return;
  }

  printf("TradeContext created! Subscribing to order events...\n");

  const enum lb_topic_type_t topics[] = { TopicPrivate };

  // Pass http_client from our userdata to subscribe callback
  lb_http_client_t* cli = (lb_http_client_t*)res->userdata;

  lb_trade_context_subscribe(res->ctx, topics, 1, on_subscribe_complete, cli);
  lb_trade_context_set_on_order_changed(res->ctx, on_order_event, NULL, NULL);
}

void
on_order_event(const struct lb_trade_context_t* ctx,
               const struct lb_push_order_changed_t* order,
               void* user_data)
{
  printf("\n=== RECEIVED ORDER EVENT ===\n");
  if (order == NULL) {
    printf("Order event is NULL\n");
    return;
  }

  printf("Order ID: %s\n", order->order_id);
  printf("Symbol: %s\n", order->symbol);
  printf("Stock Name: %s\n", order->stock_name);
  printf("Side: %d\n", order->side);
  printf("Order Type: %d\n", order->order_type);
  printf("Order Status: %d\n", order->status);
  printf("Submitted Quantity: %" PRId64 "\n", order->submitted_quantity);
  printf("Executed Quantity: %" PRId64 "\n", order->executed_quantity);
  printf("Currency: %s\n", order->currency);
  printf("Account No: %s\n", order->account_no);
  printf("Message: %s\n", order->msg);
  printf("Remark: %s\n", order->remark);
  printf("============================\n\n");

  is_finished = 1;
}

void
print_decimal(const lb_decimal_t* decimal)
{
  if (decimal) {
    printf("%f", lb_decimal_to_double(decimal));
  } else {
    printf("NULL");
  }
}
