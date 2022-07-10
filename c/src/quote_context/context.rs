use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{
    quote::{PushEvent, PushEventDetail, SubFlags},
    QuoteContext,
};
use parking_lot::Mutex;

use crate::{
    async_call::{execute_async, CAsyncCallback, CAsyncResult},
    callback::{CFreeUserDataFunc, Callback},
    config::CConfig,
    quote_context::{
        enum_types::{CAdjustType, CPeriod},
        types::{
            CCandlestickOwned, CCapitalDistributionResponseOwned, CCapitalFlowLineOwned,
            CIntradayLineOwned, CIssuerInfoOwned, CMarketTradingDaysOwned,
            CMarketTradingSessionOwned, COptionQuoteOwned, CParticipantInfoOwned, CPushBrokers,
            CPushBrokersOwned, CPushCandlestick, CPushCandlestickOwned, CPushDepth,
            CPushDepthOwned, CPushQuote, CPushQuoteOwned, CPushTrades, CPushTradesOwned,
            CRealtimeQuoteOwned, CSecurityBrokersOwned, CSecurityDepthOwned, CSecurityQuoteOwned,
            CSecurityStaticInfoOwned, CStrikePriceInfoOwned, CSubscriptionOwned, CTradeOwned,
            CWarrantQuoteOwned,
        },
    },
    types::{cstr_array_to_rust, cstr_to_rust, CCow, CDate, CMarket, CVec, ToFFI},
};

pub type COnQuoteCallback = extern "C" fn(*const CQuoteContext, *const CPushQuote, *mut c_void);

pub type COnDepthCallback = extern "C" fn(*const CQuoteContext, *const CPushDepth, *mut c_void);

pub type COnBrokersCallback = extern "C" fn(*const CQuoteContext, *const CPushBrokers, *mut c_void);

pub type COnTradesCallback = extern "C" fn(*const CQuoteContext, *const CPushTrades, *mut c_void);

pub type COnCandlestickCallback =
    extern "C" fn(*const CQuoteContext, *const CPushCandlestick, *mut c_void);

#[derive(Default)]
struct Callbacks {
    quote: Option<Callback<COnQuoteCallback>>,
    depth: Option<Callback<COnDepthCallback>>,
    brokers: Option<Callback<COnBrokersCallback>>,
    trades: Option<Callback<COnTradesCallback>>,
    candlestick: Option<Callback<COnCandlestickCallback>>,
}

pub struct CQuoteContextState {
    callbacks: Callbacks,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
}

unsafe impl Send for CQuoteContextState {}

/// Quote context
pub struct CQuoteContext {
    ctx: QuoteContext,
    state: Mutex<CQuoteContextState>,
}

impl Drop for CQuoteContext {
    fn drop(&mut self) {
        let state = self.state.lock();
        if let Some(free_userdata) = state.free_userdata {
            free_userdata(state.userdata);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_new(
    config: *const CConfig,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let config = (*config).0.clone();
    let userdata_pointer = userdata as usize;

    execute_async(
        callback,
        std::ptr::null_mut::<c_void>(),
        userdata,
        async move {
            let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
            let state = Mutex::new(CQuoteContextState {
                userdata: std::ptr::null_mut(),
                callbacks: Callbacks::default(),
                free_userdata: None,
            });
            let arc_ctx = Arc::new(CQuoteContext { ctx, state });
            let weak_ctx = Arc::downgrade(&arc_ctx);
            let ctx = Arc::into_raw(arc_ctx);

            tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    let ctx = match weak_ctx.upgrade() {
                        Some(ctx) => ctx,
                        None => return,
                    };

                    let state = ctx.state.lock();
                    match event {
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Quote(quote),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.quote {
                                let quote_owned: CPushQuoteOwned = (symbol, quote).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &quote_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Depth(depth),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.depth {
                                let depth_owned: CPushDepthOwned = (symbol, depth).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &depth_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Brokers(brokers),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.brokers {
                                let brokers_owned: CPushBrokersOwned = (symbol, brokers).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &brokers_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Trade(trades),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.trades {
                                let trades_owned: CPushTradesOwned = (symbol, trades).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &trades_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                        PushEvent {
                            symbol,
                            detail: PushEventDetail::Candlestick(candlestick),
                            ..
                        } => {
                            if let Some(callback) = &state.callbacks.candlestick {
                                let candlestick_owned: CPushCandlestickOwned =
                                    (symbol, candlestick).into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &candlestick_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                    }
                }
            });

            Ok(CAsyncResult {
                ctx: ctx as *const c_void,
                error: std::ptr::null(),
                data: std::ptr::null_mut(),
                length: 0,
                userdata: userdata_pointer as *mut c_void,
            })
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_retain(ctx: *const CQuoteContext) {
    Arc::increment_strong_count(ctx);
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_release(ctx: *const CQuoteContext) {
    let _ = Arc::from_raw(ctx);
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_userdata(
    ctx: *const CQuoteContext,
    userdata: *mut c_void,
) {
    (*ctx).state.lock().userdata = userdata;
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_userdata(ctx: *const CQuoteContext) -> *mut c_void {
    (*ctx).state.lock().userdata
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_free_userdata_func(
    ctx: *const CQuoteContext,
    f: CFreeUserDataFunc,
) {
    (*ctx).state.lock().free_userdata = f;
}

/// Set quote callback, after receiving the quote data push, it will call back
/// to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_on_quote(
    ctx: *const CQuoteContext,
    callback: COnQuoteCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.quote = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set depth callback, after receiving the depth data push, it will call
/// back to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_on_depth(
    ctx: *const CQuoteContext,
    callback: COnDepthCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.depth = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set brokers callback, after receiving the brokers data push, it will
/// call back to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_on_brokers(
    ctx: *const CQuoteContext,
    callback: COnBrokersCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.brokers = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set trades callback, after receiving the trades data push, it will call
/// back to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_on_trades(
    ctx: *const CQuoteContext,
    callback: COnTradesCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.trades = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

/// Set candlestick callback, after receiving the trades data push, it will
/// call back to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_set_on_candlestick(
    ctx: *const CQuoteContext,
    callback: COnCandlestickCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.candlestick = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_subscribe(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    sub_types: u8,
    is_first_push: bool,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .subscribe(
                symbols,
                SubFlags::from_bits(sub_types).unwrap_or_else(SubFlags::empty),
                is_first_push,
            )
            .await
    });
}

/// Unsubscribe
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_unsubscribe(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    sub_types: u8,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .unsubscribe(
                symbols,
                SubFlags::from_bits(sub_types).unwrap_or_else(SubFlags::empty),
            )
            .await
    });
}

/// Subscribe security candlesticks
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_subscribe_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .subscribe_candlesticks(symbol, period.into())
            .await
    });
}

/// Unsubscribe security candlesticks
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_unsubscribe_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        ctx_inner
            .unsubscribe_candlesticks(symbol, period.into())
            .await
    });
}

/// Get subscription information
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_subscrptions(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSubscriptionOwned> = ctx_inner.subscriptions().await?.into();
        Ok(rows)
    });
}

/// Get basic information of securities
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_static_info(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSecurityStaticInfoOwned> = ctx_inner.static_info(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of securities
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CSecurityQuoteOwned> = ctx_inner.quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of option securities
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_option_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<COptionQuoteOwned> = ctx_inner.option_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get quote of warrant securities
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_warrant_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CWarrantQuoteOwned> = ctx_inner.warrant_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get security depth
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_depth(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityDepthOwned> = CCow::new(
            ctx_inner
                .depth(symbol)
                .await
                .map(CSecurityDepthOwned::from)?,
        );
        Ok(resp)
    });
}

/// Get security brokers
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_brokers(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityBrokersOwned> = CCow::new(
            ctx_inner
                .brokers(symbol)
                .await
                .map(CSecurityBrokersOwned::from)?,
        );
        Ok(resp)
    });
}

/// Get participants
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_participants(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CParticipantInfoOwned> = ctx_inner.participants().await?.into();
        Ok(rows)
    });
}

/// Get security trades
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_trades(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTradeOwned> = ctx_inner.trades(symbol, count).await?.into();
        Ok(rows)
    });
}

/// Get security intraday lines
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_intraday(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CIntradayLineOwned> = ctx_inner.intraday(symbol).await?.into();
        Ok(rows)
    });
}

/// Get security candlesticks
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    count: usize,
    adjust_type: CAdjustType,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .candlesticks(symbol, period.into(), count, adjust_type.into())
            .await?
            .into();
        Ok(rows)
    });
}

/// Get option chain expiry date list
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_option_chain_expiry_date_list(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CDate> = ctx_inner
            .option_chain_expiry_date_list(symbol)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get option chain info by date
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_option_chain_info_by_date(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    expiry_date: *const CDate,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    let expiry_date = (*expiry_date).into();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CStrikePriceInfoOwned> = ctx_inner
            .option_chain_info_by_date(symbol, expiry_date)
            .await?
            .into();
        Ok(rows)
    });
}

/// Get warrant issuers
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_warrant_issuers(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CIssuerInfoOwned> = ctx_inner.warrant_issuers().await?.into();
        Ok(rows)
    });
}

/// Get trading session of the day
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_trading_session(
    ctx: *const CQuoteContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CMarketTradingSessionOwned> = ctx_inner.trading_session().await?.into();
        Ok(rows)
    });
}

/// Get market trading days
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_trading_days(
    ctx: *const CQuoteContext,
    market: CMarket,
    begin: *const CDate,
    end: *const CDate,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let market = market.into();
    let begin = *begin;
    let end = *end;
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CMarketTradingDaysOwned> = CCow::new(
            ctx_inner
                .trading_days(market, begin.into(), end.into())
                .await?,
        );
        Ok(resp)
    });
}

/// Get capital flow intraday
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_capital_flow(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCapitalFlowLineOwned> = ctx_inner.capital_flow(symbol).await?.into();
        Ok(rows)
    });
}

/// Get capital distribution
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_capital_distribution(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CCapitalDistributionResponseOwned> =
            CCow::new(ctx_inner.capital_distribution(symbol).await?);
        Ok(resp)
    });
}

/// Get quote of securities
///
/// Get real-time quotes of the subscribed symbols, it always returns the data
/// in the local storage.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_realtime_quote(
    ctx: *const CQuoteContext,
    symbols: *const *const c_char,
    num_symbols: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbols = cstr_array_to_rust(symbols, num_symbols);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CRealtimeQuoteOwned> = ctx_inner.realtime_quote(symbols).await?.into();
        Ok(rows)
    });
}

/// Get real-time depth
///
/// Get real-time depth of the subscribed symbols, it always returns the data in
/// the local storage.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_realtime_depth(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityDepthOwned> = CCow::new(ctx_inner.realtime_depth(symbol).await?);
        Ok(resp)
    });
}

/// Get real-time trades
///
/// Get real-time trades of the subscribed symbols, it always returns the data
/// in the local storage.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_realtime_trades(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CTradeOwned> = ctx_inner.realtime_trades(symbol, count).await?.into();
        Ok(rows)
    });
}

/// Get real-time broker queue
///
/// Get real-time broker queue of the subscribed symbols, it always returns the
/// data in the local storage.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_realtime_brokers(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CSecurityBrokersOwned> =
            CCow::new(ctx_inner.realtime_brokers(symbol).await?);
        Ok(resp)
    });
}

/// Get real-time candlesticks
///
/// Get real-time candlesticks of the subscribed symbols, it always returns the
/// data in the local storage.
#[no_mangle]
pub unsafe extern "C" fn lb_quote_context_realtime_candlesticks(
    ctx: *const CQuoteContext,
    symbol: *const c_char,
    period: CPeriod,
    count: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let symbol = cstr_to_rust(symbol);
    execute_async(callback, ctx, userdata, async move {
        let rows: CVec<CCandlestickOwned> = ctx_inner
            .realtime_candlesticks(symbol, period.into(), count)
            .await?
            .into();
        Ok(rows)
    });
}
