from datetime import datetime, date, time
from decimal import Decimal
from typing import Any, Callable, List, Optional, Type 


class OpenApiException(Exception):
    """
    OpenAPI exception
    """

    code: Optional[int]
    """
    Error code
    """

    message: str
    """
    Error message
    """

    def __init__(self, code: int, message: str) -> None:
        ...


class HttpClient:
    """
    A HTTP client for longPort open api

    Args:
        http_url: HTTP API url
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
    """

    def __init__(self, http_url: str, app_key: str,
                 app_secret: str, access_token: str) -> None: ...

    @classmethod
    def from_env(cls: Type) -> HttpClient:
        """
        Create a new `HttpClient` from the given environment variables

        It first gets the environment variables from the `.env` file in the current directory.

        # Variables

        - `LONGPORT_HTTP_URL` - HTTP endpoint url
        - `LONGPORT_APP_KEY` - App key
        - `LONGPORT_APP_SECRET` - App secret
        - `LONGPORT_ACCESS_TOKEN` - Access token
        """

    def request(self, method: str, path: str, headers: Optional[dict[str, str]] = None, body: Optional[Any] = None) -> Any:
        """
        Performs a HTTP reqest

        Examples:
            ::

                from longport.openapi import HttpClient

                client = HttpClient(http_url, app_key,
                                    app_secret, access_token);

                # get
                resp = client.request("get", "/foo/bar");
                print(resp)

                # post
                client.request("get", "/foo/bar", { "foo": 1, "bar": 2 });
        """
        ...


class Config:
    """
    Configuration options for LongPort sdk

    Args:
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
        http_url: HTTP API url
        trade_ws_url: Websocket url for trade API
        language: Language identifier
    """

    def __init__(
        self,
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url: Optional[str] = None,
        trade_ws_url: Optional[str] = None,
        language: Optional[Type[Language]] = None,
    ) -> None: ...

    @classmethod
    def from_env(cls: Type) -> Config:
        """
        Create a new `Config` from the given environment variables

        It first gets the environment variables from the `.env` file in the current directory.

        # Variables

        - `LONGPORT_APP_KEY` - App key
        - `LONGPORT_APP_SECRET` - App secret
        - `LONGPORT_ACCESS_TOKEN` - Access token
        - `LONGPORT_HTTP_URL` - HTTP endpoint url
        - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url
        """

class Language:
    """
    Language identifier
    """

    class ZH_CN(Language):
        """
        zh-CN
        """

    class ZH_HK(Language):
        """
        zh-HK
        """

    class EN(Language):
        """
        en
        """

class TradeStatus:
    """
    Security Status
    """

    class Normal(TradeStatus):
        """
        Normal
        """

    class Halted(TradeStatus):
        """
        Suspension
        """

    class Delisted(TradeStatus):
        """
        Delisted
        """

    class Fuse(TradeStatus):
        """
        Fuse
        """

    class PrepareList(TradeStatus):
        """
        Prepare List
        """

    class CodeMoved(TradeStatus):
        """
        Code Moved
        """

    class ToBeOpened(TradeStatus):
        """
        To Be Opened
        """

    class SplitStockHalts(TradeStatus):
        """
        Split Stock Halts
        """

    class Expired(TradeStatus):
        """
        Expired
        """

    class WarrantPrepareList(TradeStatus):
        """
        Warrant To BeListed
        """

    class SuspendTrade(TradeStatus):
        """
        Suspend
        """

class OrderSide:
    """
    Order side
    """

    class Unknown(OrderSide):
        """
        Unknown
        """

    class Buy(OrderSide):
        """
        Buy
        """

    class Sell(OrderSide):
        """
        Sell
        """


class OrderType:
    """
    Order type
    """

    class Unknown(OrderType):
        """
        Unknown
        """

    class LO(OrderType):
        """
        Limit Order
        """

    class ELO(OrderType):
        """
        Enhanced Limit Order
        """

    class MO(OrderType):
        """
        Market Order
        """

    class AO(OrderType):
        """
        At-auction Order
        """

    class ALO(OrderType):
        """
        At-auction Limit Order
        """

    class ODD(OrderType):
        """
        Odd Lots
        """

    class LIT(OrderType):
        """
        Limit If Touched
        """

    class MIT(OrderType):
        """
        Market If Touched
        """

    class TSLPAMT(OrderType):
        """
        Trailing Limit If Touched (Trailing Amount)
        """

    class TSLPPCT(OrderType):
        """
        Trailing Limit If Touched (Trailing Percent)
        """

    class TSMAMT(OrderType):
        """
        Trailing Market If Touched (Trailing Amount)
        """

    class TSMPCT(OrderType):
        """
        Trailing Market If Touched (Trailing Percent)
        """

    class SLO(OrderType):
        """
        Special Limit Order
        """


class OrderStatus:
    """
    Order status
    """

    class Unknown(OrderStatus):
        """
        Unknown
        """

    class NotReported(OrderStatus):
        """
        Not reported
        """

    class ReplacedNotReported(OrderStatus):
        """
        Not reported (Replaced Order)
        """

    class ProtectedNotReported(OrderStatus):
        """
        Not reported (Protected Order)
        """

    class VarietiesNotReported(OrderStatus):
        """
        Not reported (Conditional Order)
        """

    class Filled(OrderStatus):
        """
        Filled
        """

    class WaitToNew(OrderStatus):
        """
        Wait To New
        """

    class New(OrderStatus):
        """
        New
        """

    class WaitToReplace(OrderStatus):
        """
        Wait To Replace
        """

    class PendingReplace(OrderStatus):
        """
        Pending Replace
        """

    class Replaced(OrderStatus):
        """
        Replaced
        """

    class PartialFilled(OrderStatus):
        """
        Partial Filled
        """

    class WaitToCancel(OrderStatus):
        """
        Wait To Cancel
        """

    class PendingCancel(OrderStatus):
        """
        Pending Cancel
        """

    class Rejected(OrderStatus):
        """
        Rejected
        """

    class Canceled(OrderStatus):
        """
        Canceled
        """

    class Expired(OrderStatus):
        """
        ExpiredStatus
        """

    class PartialWithdrawal(OrderStatus):
        """
        PartialWithdrawal
        """


class OrderTag:
    """
    Order tag
    """

    class Unknown(OrderTag):
        """
        Unknown
        """

    class Normal(OrderTag):
        """
        Normal Order
        """

    class LongTerm(OrderTag):
        """
        Long term Order
        """

    class Grey(OrderTag):
        """
        Grey Order
        """

    class MarginCall(OrderTag):
        """
        Force Selling
        """

    class Offline(OrderTag):
        """
        OTC
        """

    class Creditor(OrderTag):
        """
        Option Exercise Long
        """

    class Debtor(OrderTag):
        """
        Option Exercise Short
        """

    class NonExercise(OrderTag):
        """
        Wavier Of Option Exercise
        """

    class AllocatedSub(OrderTag):
        """
        Trade Allocation
        """


class TriggerStatus:
    """
    Trigger status
    """

    class Unknown(TriggerStatus):
        """
        Unknown
        """

    class Deactive(TriggerStatus):
        """
        Deactive
        """

    class Active(TriggerStatus):
        """
        Active
        """

    class Released(TriggerStatus):
        """
        Released
        """


class PushOrderChanged:
    """
    Order changed message
    """

    side: Type[OrderSide]
    """
    Order side
    """

    stock_name: str
    """
    Stock name
    """

    submitted_quantity: int
    """
    Submitted quantity
    """

    symbol: str
    """
    Order symbol
    """

    order_type: Type[OrderType]
    """
    Order type
    """

    submitted_price: Decimal
    """
    Submitted price
    """

    executed_quantity: int
    """
    Executed quantity
    """

    executed_price: Optional[Decimal]
    """
    Executed price
    """

    order_id: str
    """
    Order ID
    """

    currency: str
    """
    Currency
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    submitted_at: datetime
    """
    Submitted time
    """

    updated_at: datetime
    """
    Last updated time
    """

    trigger_price: Optional[Decimal]
    """
    Order trigger price
    """

    msg: str
    """
    Rejected message or remark
    """

    tag: Type[OrderTag]
    """
    Order tag
    """

    trigger_status: Optional[Type[TriggerStatus]]
    """
    Conditional order trigger status
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    Trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    Trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    Limit offset amount
    """

    account_no: str
    """
    Account no
    """

    last_share: Optional[Decimal]
    """
    Last share
    """

    last_price: Optional[Decimal]
    """
    Last price
    """

    remark: str
    """
    Remark message
    """


class TopicType:
    """
    Topic type
    """

    class Private(TopicType):
        """
        Private notification for trade
        """
        ...


class TradeContext:
    """
    Trade context

    Args:
        config: Configuration object
    """

    def __init__(self, config: Config) -> None: ...

    def set_on_order_changed(self, callback: Callable[[PushOrderChanged], None]) -> None:
        """
        Set order changed callback, after receiving the order changed event, it will call back to this function.
        """

    def subscribe(self, topics: List[Type[TopicType]]) -> None:
        """
        Subscribe

        Args:
            topics: Topic list

        Examples:
            ::

                from time import sleep
                from decimal import Decimal
                from longport.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType, PushOrderChanged, TopicType


                def on_order_changed(event: PushOrderChanged):
                    print(event)


                config = Config.from_env()
                ctx = TradeContext(config)
                ctx.set_on_order_changed(on_order_changed)
                ctx.subscribe([TopicType.Private])
                sleep(5)  # waiting for push event
        """

    def unsubscribe(self, topics: List[str]) -> None:
        """
        Unsubscribe

        Args:
            topics: Topic list
        """

