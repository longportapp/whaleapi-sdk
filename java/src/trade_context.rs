use std::sync::Arc;

use jni::{
    errors::Result,
    objects::{GlobalRef, JClass, JObject, JValueOwned},
    sys::jobjectArray,
    JNIEnv, JavaVM,
};
use longportwhale::{
    trade::{PushEvent, TopicType},
    Config, TradeContext,
};
use parking_lot::Mutex;

use crate::{
    async_util,
    error::jni_result,
    init::TRADE_CONTEXT_CLASS,
    types::{set_field, FromJValue, IntoJValue, ObjectArray},
};

#[derive(Default)]
struct Callbacks {
    order_changed: Option<GlobalRef>,
}

struct ContextObj {
    ctx: TradeContext,
    callbacks: Arc<Mutex<Callbacks>>,
}

fn send_push_event(jvm: &JavaVM, callbacks: &Callbacks, event: PushEvent) -> Result<()> {
    let mut env = jvm.attach_current_thread().unwrap();

    match event {
        PushEvent::OrderChanged(order_changed) => {
            if let Some(handler) = &callbacks.order_changed {
                let event = order_changed.into_jvalue(&mut env)?;
                env.call_method(
                    handler,
                    "onOrderChanged",
                    "(Lcom/longportwhale/trade/PushOrderChanged;)V",
                    &[event.borrow()],
                )?;
            }
        }
    }

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longportwhale_SdkNative_newTradeContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
    callback: JObject,
) {
    struct ContextObjRef(i64);

    impl IntoJValue for ContextObjRef {
        fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
            let ctx_obj = env.new_object(TRADE_CONTEXT_CLASS.get().unwrap(), "()V", &[])?;
            set_field(env, &ctx_obj, "raw", self.0)?;
            Ok(JValueOwned::from(ctx_obj))
        }
    }

    jni_result(&mut env, (), |env| {
        let config = Arc::new((*(config as *const Config)).clone());
        let jvm = env.get_java_vm()?;

        async_util::execute(env, callback, async move {
            let (ctx, mut receiver) = TradeContext::try_new(config).await?;
            let callbacks = Arc::new(Mutex::new(Callbacks::default()));

            tokio::spawn({
                let callbacks = callbacks.clone();
                async move {
                    while let Some(event) = receiver.recv().await {
                        let callbacks = callbacks.lock();
                        let _ = send_push_event(&jvm, &callbacks, event);
                    }
                }
            });

            Ok(ContextObjRef(
                Box::into_raw(Box::new(ContextObj { ctx, callbacks })) as i64,
            ))
        })?;

        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longportwhale_SdkNative_freeTradeContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longportwhale_SdkNative_tradeContextSetOnOrderChanged(
    mut env: JNIEnv,
    _class: JClass,
    ctx: i64,
    handler: JObject,
) {
    let context = &*(ctx as *const ContextObj);
    jni_result(&mut env, (), |env| {
        if !handler.is_null() {
            context.callbacks.lock().order_changed = Some(env.new_global_ref(handler)?);
        } else {
            context.callbacks.lock().order_changed = None;
        }
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longportwhale_SdkNative_tradeContextSubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    topics: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let topics: ObjectArray<TopicType> =
            FromJValue::from_jvalue(env, JObject::from_raw(topics).into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.subscribe(topics.0).await?)
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_longportwhale_SdkNative_tradeContextUnsubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    topics: jobjectArray,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let topics: ObjectArray<TopicType> =
            FromJValue::from_jvalue(env, JObject::from_raw(topics).into())?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.unsubscribe(topics.0).await?)
        })?;
        Ok(())
    })
}
