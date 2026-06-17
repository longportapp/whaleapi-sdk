use std::{ffi::c_void, sync::Arc};

use longportwhale::{trade::PushEvent, TradeContext};
use parking_lot::Mutex;

use crate::{
    async_call::{execute_async, CAsyncCallback, CAsyncResult},
    callback::{CFreeUserDataFunc, Callback},
    config::CConfig,
    trade_context::{
        enum_types::CTopicType,
        types::{CPushOrderChanged, CPushOrderChangedOwned},
    },
    types::ToFFI,
};

pub type COnOrderChangedCallback =
    extern "C" fn(*const CTradeContext, *const CPushOrderChanged, *mut c_void);

#[derive(Default)]
struct Callbacks {
    order_changed: Option<Callback<COnOrderChangedCallback>>,
}

pub struct CTradeContextState {
    callbacks: Callbacks,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
}

unsafe impl Send for CTradeContextState {}

/// Trade context
pub struct CTradeContext {
    ctx: TradeContext,
    state: Mutex<CTradeContextState>,
}

impl Drop for CTradeContext {
    fn drop(&mut self) {
        let state = self.state.lock();
        if let Some(free_userdata) = state.free_userdata {
            free_userdata(state.userdata);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_new(
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
            let (ctx, mut receiver) = TradeContext::try_new(config).await?;
            let state = Mutex::new(CTradeContextState {
                userdata: std::ptr::null_mut(),
                callbacks: Callbacks::default(),
                free_userdata: None,
            });
            let arc_ctx = Arc::new(CTradeContext { ctx, state });
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
                        PushEvent::OrderChanged(order_changed) => {
                            if let Some(callback) = &state.callbacks.order_changed {
                                let order_changed_owned: CPushOrderChangedOwned =
                                    order_changed.into();
                                (callback.f)(
                                    Arc::as_ptr(&ctx),
                                    &order_changed_owned.to_ffi_type(),
                                    callback.userdata,
                                );
                            }
                        }
                    }
                }
            });

            Ok(CAsyncResult {
                ctx: ctx as *const c_void,
                error: std::ptr::null_mut(),
                data: std::ptr::null_mut(),
                length: 0,
                userdata: userdata_pointer as *mut c_void,
            })
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_retain(ctx: *const CTradeContext) {
    Arc::increment_strong_count(ctx);
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_release(ctx: *const CTradeContext) {
    let _ = Arc::from_raw(ctx);
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_ref_count(ctx: *const CTradeContext) -> usize {
    Arc::increment_strong_count(ctx);
    let ctx = Arc::from_raw(ctx);
    Arc::strong_count(&ctx)
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_set_userdata(
    ctx: *const CTradeContext,
    userdata: *mut c_void,
) {
    (*ctx).state.lock().userdata = userdata;
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_userdata(ctx: *const CTradeContext) -> *mut c_void {
    (*ctx).state.lock().userdata
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_set_free_userdata_func(
    ctx: *const CTradeContext,
    f: CFreeUserDataFunc,
) {
    (*ctx).state.lock().free_userdata = f;
}

/// Set order changed callback, after receiving the order changed event, it will
/// call back to this function.
#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_set_on_order_changed(
    ctx: *const CTradeContext,
    callback: COnOrderChangedCallback,
    userdata: *mut c_void,
    free_userdata: CFreeUserDataFunc,
) {
    (*ctx).state.lock().callbacks.order_changed = Some(Callback {
        f: callback,
        userdata,
        free_userdata,
    });
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_subscribe(
    ctx: *const CTradeContext,
    topics: *const CTopicType,
    num_topics: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let topics = std::slice::from_raw_parts(topics, num_topics)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.subscribe(topics).await
    });
}

#[no_mangle]
pub unsafe extern "C" fn lb_trade_context_unsubscribe(
    ctx: *const CTradeContext,
    topics: *const CTopicType,
    num_topics: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let topics = std::slice::from_raw_parts(topics, num_topics)
        .iter()
        .copied()
        .map(Into::into)
        .collect::<Vec<_>>();
    execute_async(callback, ctx, userdata, async move {
        ctx_inner.unsubscribe(topics).await
    });
}
