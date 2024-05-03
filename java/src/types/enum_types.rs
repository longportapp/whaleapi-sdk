use longportwhale_java_macros::impl_java_enum;

impl_java_enum!(
    "com/longportwhale/Language",
    longportwhale::Language,
    [ZH_CN, ZH_HK, EN]
);

impl_java_enum!(
    "com/longportwhale/Market",
    longportwhale::Market,
    [Unknown, US, HK, CN, SG]
);

impl_java_enum!(
    "com/longportwhale/trade/OrderSide",
    longportwhale::trade::OrderSide,
    [Unknown, Buy, Sell]
);

impl_java_enum!(
    "com/longportwhale/trade/OrderType",
    longportwhale::trade::OrderType,
    [Unknown, LO, ELO, MO, AO, ALO, ODD, LIT, MIT, TSLPAMT, TSLPPCT, TSMAMT, TSMPCT, SLO]
);

impl_java_enum!(
    "com/longportwhale/trade/OrderStatus",
    longportwhale::trade::OrderStatus,
    [
        Unknown,
        NotReported,
        ReplacedNotReported,
        ProtectedNotReported,
        VarietiesNotReported,
        Filled,
        WaitToNew,
        New,
        WaitToReplace,
        PendingReplace,
        Replaced,
        PartialFilled,
        WaitToCancel,
        PendingCancel,
        Rejected,
        Canceled,
        Expired,
        PartialWithdrawal,
    ]
);

impl_java_enum!(
    "com/longportwhale/trade/OrderTag",
    longportwhale::trade::OrderTag,
    [
        Unknown,
        Normal,
        LongTerm,
        Grey,
        MarginCall,
        Offline,
        Creditor,
        Debtor,
        NonExercise,
        AllocatedSub,
    ]
);

impl_java_enum!(
    "com/longportwhale/trade/TriggerStatus",
    longportwhale::trade::TriggerStatus,
    [Unknown, Deactive, Active, Released]
);

impl_java_enum!(
    "com/longportwhale/trade/TopicType",
    longportwhale::trade::TopicType,
    [Private]
);

