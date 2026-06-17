#ifndef _LONGPORTWHALE_H_
#define _LONGPORTWHALE_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Language identifer
 */
typedef enum lb_language_t {
  /**
   * zh-CN
   */
  Language_ZH_CN,
  /**
   * zh-HK
   */
  Language_ZH_HK,
  /**
   * en
   */
  Language_EN,
} lb_language_t;

/**
 * Order side
 */
typedef enum lb_order_side_t {
  /**
   * Unknown
   */
  OrderSideUnknown,
  /**
   * Buy
   */
  OrderSideBuy,
  /**
   * Sell
   */
  OrderSideSell,
} lb_order_side_t;

/**
 * Order status
 */
typedef enum lb_order_status_t {
  /**
   * Unknown
   */
  OrderStatusUnknown,
  /**
   * Not reported
   */
  OrderStatusNotReported,
  /**
   * Not reported (Replaced Order)
   */
  OrderStatusReplacedNotReported,
  /**
   * Not reported (Protected Order)
   */
  OrderStatusProtectedNotReported,
  /**
   * Not reported (Conditional Order)
   */
  OrderStatusVarietiesNotReported,
  /**
   * Filled
   */
  OrderStatusFilled,
  /**
   * Wait To New
   */
  OrderStatusWaitToNew,
  /**
   * New
   */
  OrderStatusNew,
  /**
   * Wait To Replace
   */
  OrderStatusWaitToReplace,
  /**
   * Pending Replace
   */
  OrderStatusPendingReplace,
  /**
   * Replaced
   */
  OrderStatusReplaced,
  /**
   * Partial Filled
   */
  OrderStatusPartialFilled,
  /**
   * Wait To Cancel
   */
  OrderStatusWaitToCancel,
  /**
   * Pending Cancel
   */
  OrderStatusPendingCancel,
  /**
   * Rejected
   */
  OrderStatusRejected,
  /**
   * Canceled
   */
  OrderStatusCanceled,
  /**
   * Expired
   */
  OrderStatusExpired,
  /**
   * Partial Withdrawal
   */
  OrderStatusPartialWithdrawal,
} lb_order_status_t;

/**
 * Order tag
 */
typedef enum lb_order_tag_t {
  /**
   * Unknown
   */
  OrderTagUnknown,
  /**
   * Normal Order
   */
  OrderTagNormal,
  /**
   * Long term Order
   */
  OrderTagLongTerm,
  /**
   * Grey Order
   */
  OrderTagGrey,
  /**
   * Force Selling
   */
  OrderTagMarginCall,
  /**
   * OTC
   */
  OrderTagOffline,
  /**
   * Option Exercise Long
   */
  OrderTagCreditor,
  /**
   * Option Exercise Short
   */
  OrderTagDebtor,
  /**
   * Wavier Of Option Exercise
   */
  OrderTagNonExercise,
  /**
   * Trade Allocation
   */
  OrderTagAllocatedSub,
} lb_order_tag_t;

/**
 * Order type
 */
typedef enum lb_order_type_t {
  /**
   * Unknown
   */
  OrderTypeUnknown,
  /**
   * Limit Order
   */
  OrderTypeLO,
  /**
   * Enhanced Limit Order
   */
  OrderTypeELO,
  /**
   * Market Order
   */
  OrderTypeMO,
  /**
   * At-auction Order
   */
  OrderTypeAO,
  /**
   * At-auction Limit Order
   */
  OrderTypeALO,
  /**
   * Odd Lots
   */
  OrderTypeODD,
  /**
   * Limit If Touched
   */
  OrderTypeLIT,
  /**
   * Market If Touched
   */
  OrderTypeMIT,
  /**
   * Trailing Limit If Touched (Trailing Amount)
   */
  OrderTypeTSLPAMT,
  /**
   * Trailing Limit If Touched (Trailing Percent)
   */
  OrderTypeTSLPPCT,
  /**
   * Trailing Market If Touched (Trailing Amount)
   */
  OrderTypeTSMAMT,
  /**
   * Trailing Market If Touched (Trailing Percent)
   */
  OrderTypeTSMPCT,
  /**
   * Special Limit Order
   */
  OrderTypeSLO,
} lb_order_type_t;

/**
 * Topic type
 */
typedef enum lb_topic_type_t {
  /**
   * Trading
   */
  TopicPrivate,
} lb_topic_type_t;

/**
 * Order tag
 */
typedef enum lb_trigger_status_t {
  /**
   * Unknown
   */
  TriggerStatusUnknown,
  /**
   * Deactive
   */
  TriggerStatusDeactive,
  /**
   * Active
   */
  TriggerStatusActive,
  /**
   * Released
   */
  TriggerStatusReleased,
} lb_trigger_status_t;

/**
 * Configuration options for LongPort sdk
 */
typedef struct lb_config_t lb_config_t;

typedef struct lb_decimal_t lb_decimal_t;

typedef struct lb_error_t lb_error_t;

/**
 * A HTTP client for LongPort OpenApi
 */
typedef struct lb_http_client_t lb_http_client_t;

typedef struct lb_http_result_t lb_http_result_t;

/**
 * Trade context
 */
typedef struct lb_trade_context_t lb_trade_context_t;

/**
 * HTTP Header
 */
typedef struct lb_http_header_t {
  const char *name;
  const char *value;
} lb_http_header_t;

typedef struct lb_async_result_t {
  const void *ctx;
  const struct lb_error_t *error;
  void *data;
  uintptr_t length;
  void *userdata;
} lb_async_result_t;

typedef void (*lb_async_callback_t)(const struct lb_async_result_t*);

typedef void (*lb_free_userdata_func_t)(void*);

/**
 * Order changed message
 */
typedef struct lb_push_order_changed_t {
  /**
   * Order side
   */
  enum lb_order_side_t side;
  /**
   * Stock name
   */
  const char *stock_name;
  /**
   * Submitted quantity
   */
  int64_t submitted_quantity;
  /**
   * Order symbol
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Submitted price
   */
  const struct lb_decimal_t *submitted_price;
  /**
   * Executed quantity
   */
  int64_t executed_quantity;
  /**
   * Executed price (maybe null)
   */
  const struct lb_decimal_t *executed_price;
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Order status
   */
  enum lb_order_status_t status;
  /**
   * Submitted time
   */
  int64_t submitted_at;
  /**
   * Last updated time
   */
  int64_t updated_at;
  /**
   * Order trigger price (maybe null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Rejected message or remark
   */
  const char *msg;
  /**
   * Order tag
   */
  enum lb_order_tag_t tag;
  /**
   * Conditional order trigger status (maybe null)
   */
  const enum lb_trigger_status_t *trigger_status;
  /**
   * Conditional order trigger time (maybe null)
   */
  const int64_t *trigger_at;
  /**
   * Trailing amount (maybe null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * Trailing percent (maybe null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * Limit offset amount (maybe null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Account no
   */
  const char *account_no;
  /**
   * Last share (maybe null)
   */
  const struct lb_decimal_t *last_share;
  /**
   * Last price (maybe null)
   */
  const struct lb_decimal_t *last_price;
  /**
   * Remark message
   */
  const char *remark;
} lb_push_order_changed_t;

typedef void (*lb_order_changed_callback_t)(const struct lb_trade_context_t*,
                                            const struct lb_push_order_changed_t*,
                                            void*);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new `Config` from the given environment variables
 *
 * It first gets the environment variables from the `.env` file in the
 * current directory.
 *
 * # Variables
 *
 * - `LONGPORT_APP_KEY` - App key
 * - `LONGPORT_APP_SECRET` - App secret
 * - `LONGPORT_ACCESS_TOKEN` - Access token
 * - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longportapp.com`)
 * - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
 *   `wss://openapi-trade.longportapp.com`)
 */
struct lb_config_t *lb_config_from_env(struct lb_error_t **error);

struct lb_config_t *lb_config_new(const char *app_key,
                                  const char *app_secret,
                                  const char *access_token,
                                  const char *http_url,
                                  const char *trade_ws_url,
                                  const enum lb_language_t *language);

/**
 * Free the config object
 */
void lb_config_free(struct lb_config_t *config);

/**
 * Free the error object
 */
void lb_error_free(struct lb_error_t *error);

const char *lb_error_message(const struct lb_error_t *error);

int64_t lb_error_code(const struct lb_error_t *error);

/**
 * Create a HTTP client
 */
struct lb_http_client_t *lb_http_client_new(const char *http_url,
                                            const char *app_key,
                                            const char *app_secret,
                                            const char *access_token);

/**
 * Free the http client
 */
void lb_http_client_free(struct lb_http_client_t *http_client);

/**
 * Create a new `HttpClient` from the given environment variables
 *
 * It first gets the environment variables from the `.env` file in the
 * current directory.
 *
 * # Variables
 *
 * - `LONGPORT_HTTP_URL` - HTTP endpoint url
 * - `LONGPORT_APP_KEY` - App key
 * - `LONGPORT_APP_SECRET` - App secret
 * - `LONGPORT_ACCESS_TOKEN` - Access token
 */
struct lb_http_client_t *lb_http_client_from_env(struct lb_error_t **error);

/**
 * Performs a HTTP request
 */
void lb_http_client_request(struct lb_http_client_t *http_client,
                            const char *method,
                            const char *path,
                            const struct lb_http_header_t *headers,
                            const char *request_body,
                            lb_async_callback_t callback,
                            void *userdata);

/**
 * Free the HTTP result
 */
void lb_http_result_free(struct lb_http_result_t *http_result);

const char *lb_http_result_response_body(const struct lb_http_result_t *http_result);

void lb_trade_context_new(const struct lb_config_t *config,
                          lb_async_callback_t callback,
                          void *userdata);

void lb_trade_context_retain(const struct lb_trade_context_t *ctx);

void lb_trade_context_release(const struct lb_trade_context_t *ctx);

uintptr_t lb_trade_context_ref_count(const struct lb_trade_context_t *ctx);

void lb_trade_context_set_userdata(const struct lb_trade_context_t *ctx, void *userdata);

void *lb_trade_context_userdata(const struct lb_trade_context_t *ctx);

void lb_trade_context_set_free_userdata_func(const struct lb_trade_context_t *ctx,
                                             lb_free_userdata_func_t f);

/**
 * Set order changed callback, after receiving the order changed event, it will
 * call back to this function.
 */
void lb_trade_context_set_on_order_changed(const struct lb_trade_context_t *ctx,
                                           lb_order_changed_callback_t callback,
                                           void *userdata,
                                           lb_free_userdata_func_t free_userdata);

void lb_trade_context_subscribe(const struct lb_trade_context_t *ctx,
                                const enum lb_topic_type_t *topics,
                                uintptr_t num_topics,
                                lb_async_callback_t callback,
                                void *userdata);

void lb_trade_context_unsubscribe(const struct lb_trade_context_t *ctx,
                                  const enum lb_topic_type_t *topics,
                                  uintptr_t num_topics,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Create a decimal value with a 64 bit `m` representation and corresponding
 * `e` scale.
 */
struct lb_decimal_t *lb_decimal_new(int64_t num, uint32_t scale);

/**
 * Clone the decimal value
 */
struct lb_decimal_t *lb_decimal_clone(const struct lb_decimal_t *value);

/**
 * Create a decimal value from string
 */
struct lb_decimal_t *lb_decimal_from_str(const char *value);

/**
 * Create a decimal value from double
 */
struct lb_decimal_t *lb_decimal_from_double(double value);

/**
 * Free the decimal value
 */
void lb_decimal_free(struct lb_decimal_t *value);

double lb_decimal_to_double(const struct lb_decimal_t *value);

/**
 * Computes the absolute value.
 */
void lb_decimal_abs(struct lb_decimal_t *value);

/**
 * Returns the smallest integer greater than or equal to a number.
 */
void lb_decimal_ceil(struct lb_decimal_t *value);

/**
 * Returns the largest integer less than or equal to a number.
 */
void lb_decimal_floor(struct lb_decimal_t *value);

/**
 * Returns a new Decimal representing the fractional portion of the number.
 */
void lb_decimal_fract(struct lb_decimal_t *value);

/**
 * Returns `true` if the decimal is negative.
 */
bool lb_decimal_is_negative(const struct lb_decimal_t *value);

/**
 * Returns `true` if the decimal is positive.
 */
bool lb_decimal_is_positive(const struct lb_decimal_t *value);

/**
 * Returns `true` if this Decimal number is equivalent to zero.
 */
bool lb_decimal_is_zero(const struct lb_decimal_t *value);

/**
 * Returns the maximum of the two numbers.
 */
const struct lb_decimal_t *lb_decimal_max(const struct lb_decimal_t *a,
                                          const struct lb_decimal_t *b);

/**
 * Returns the minimum of the two numbers.
 */
const struct lb_decimal_t *lb_decimal_min(const struct lb_decimal_t *a,
                                          const struct lb_decimal_t *b);

/**
 * Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
 */
void lb_decimal_normalize(struct lb_decimal_t *value);

/**
 * Returns a new Decimal number with no fractional portion (i.e. an
 * integer). Rounding currently follows “Bankers Rounding” rules. e.g.
 * `6.5` -> `6`, `7.5` -> `8`
 */
void lb_decimal_round(struct lb_decimal_t *value);

/**
 * Returns a new Decimal integral with no fractional portion. This is a
 * true truncation whereby no rounding is performed.
 */
void lb_decimal_trunc(struct lb_decimal_t *value);

/**
 * Performs the `+` operation.
 */
void lb_decimal_add(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `-` operation.
 */
void lb_decimal_sub(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `*` operation.
 */
void lb_decimal_mul(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `/` operation.
 */
void lb_decimal_div(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `%` operation.
 */
void lb_decimal_rem(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the unary `-` operation.
 */
void lb_decimal_neg(struct lb_decimal_t *value);

/**
 * Returns `true` if the value of this Decimal is greater than the value of
 * `x`, otherwise returns `false`.
 */
bool lb_decimal_gt(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is greater than or equal to
 * the value of `x`, otherwise returns `false`.
 */
bool lb_decimal_gte(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal equals the value of `x`,
 * otherwise returns `false`.
 */
bool lb_decimal_eq(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is less than the value of
 * `x`, otherwise returns `false`.
 */
bool lb_decimal_lt(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is less than or equal to the
 * value of `x`, otherwise returns `false`.
 */
bool lb_decimal_lte(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Compares the values of two Decimals.
 *
 * Returns `-1` if the value of this Decimal is less than the value of
 * `x`.
 *
 * Returns `1` if the value of this Decimal is greater than the value of
 * `x`.
 *
 * Returns `0` if the value of this Decimal equals the value of `x`.
 */
int32_t lb_decimal_cmp(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Computes the sine of a number (in radians)
 */
void lb_decimal_sin(struct lb_decimal_t *value);

/**
 * Computes the cosine of a number (in radians)
 */
void lb_decimal_cos(struct lb_decimal_t *value);

/**
 * Computes the tangent of a number (in radians). Panics upon overflow or
 * upon approaching a limit.
 */
void lb_decimal_tan(struct lb_decimal_t *value);

/**
 * The square root of a Decimal. Uses a standard Babylonian method.
 */
void lb_decimal_sqrt(struct lb_decimal_t *value);

/**
 * Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
 * whole then the approximation e<sup>y*ln(x)</sup> is used.
 */
void lb_decimal_pow(struct lb_decimal_t *value, const struct lb_decimal_t *exp);

/**
 * Calculates the natural logarithm for a Decimal calculated using Taylor’s
 * series.
 */
void lb_decimal_ln(struct lb_decimal_t *value);

/**
 * Calculates the base 10 logarithm of a specified Decimal number.
 */
void lb_decimal_log10(struct lb_decimal_t *value);

/**
 * The estimated exponential function, ex. Stops calculating when it is
 * within tolerance of roughly `0.0000002`.
 */
void lb_decimal_exp(struct lb_decimal_t *value);

/**
 * The estimated exponential function, e<sup>x</sup> using the `tolerance`
 * provided as a hint as to when to stop calculating. A larger
 * tolerance will cause the number to stop calculating sooner at the
 * potential cost of a slightly less accurate result.
 */
void lb_decimal_exp_with_tolerance(struct lb_decimal_t *value,
                                   const struct lb_decimal_t *tolerance);

/**
 * Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
 */
void lb_decimal_erf(struct lb_decimal_t *value);

/**
 * The Cumulative distribution function for a Normal distribution
 */
void lb_decimal_normal_cdf(struct lb_decimal_t *value);

/**
 * The Probability density function for a Normal distribution.
 */
void lb_decimal_norm_pdf(struct lb_decimal_t *value);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _LONGPORTWHALE_H_ */
