use jni::{
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
    JNIEnv,
};
use once_cell::sync::OnceCell;

use crate::types::ClassLoader;

pub(crate) static LONG_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static STRING_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static DECIMAL_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_INSTANT_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_OFFSETDATETIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALDATE_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALTIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALDATETIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_ZONE_ID: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TRADE_CONTEXT_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static OPENAPI_EXCEPTION_CLASS: OnceCell<GlobalRef> = OnceCell::new();

fn init_timezone_id(env: &mut JNIEnv) {
    let utc = env.new_string("UTC").unwrap();
    let zone_id = env
        .call_static_method(
            "java/time/ZoneId",
            "of",
            "(Ljava/lang/String;)Ljava/time/ZoneId;",
            &[JValue::from(&utc)],
        )
        .expect("create zone id");
    let _ = TIME_ZONE_ID.set(env.new_global_ref(zone_id.l().unwrap()).unwrap());
}

macro_rules! init_class {
    ($env:expr, $(($id:ident, $ty:literal)),*) => {
        $(
        let cls = Desc::<JClass>::lookup($ty, &mut $env).expect($ty);
        let _ = $id.set($env.new_global_ref::<&JClass>(&*cls).unwrap());
        )*
    };
}

macro_rules! init_class_by_classloader {
    ($env:expr, $($id:ty),*) => {
        $(
            <$id>::init(&mut $env);
        )*
    }
}

#[no_mangle]
pub extern "system" fn Java_com_longportwhale_SdkNative_init<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
) {
    init_class!(
        env,
        (LONG_CLASS, "java/lang/Long"),
        (STRING_CLASS, "java/lang/String"),
        (DECIMAL_CLASS, "java/math/BigDecimal"),
        (TIME_INSTANT_CLASS, "java/time/Instant"),
        (TIME_OFFSETDATETIME_CLASS, "java/time/OffsetDateTime"),
        (TIME_LOCALDATE_CLASS, "java/time/LocalDate"),
        (TIME_LOCALTIME_CLASS, "java/time/LocalTime"),
        (TIME_LOCALDATETIME_CLASS, "java/time/LocalDateTime"),
        (OPENAPI_EXCEPTION_CLASS, "com/longportwhale/OpenApiException"),
        (TRADE_CONTEXT_CLASS, "com/longportwhale/trade/TradeContext")
    );

    init_timezone_id(&mut env);

    // enum types
    init_class_by_classloader!(
        env,
        longportwhale::Language,
        longportwhale::Market,
        longportwhale::trade::OrderSide,
        longportwhale::trade::OrderType,
        longportwhale::trade::OrderStatus,
        longportwhale::trade::OrderTag,
        longportwhale::trade::TriggerStatus,
        longportwhale::trade::TopicType
    );

    // classes
    init_class_by_classloader!(env, longportwhale::trade::PushOrderChanged);
}
