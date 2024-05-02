import time
import threading
import urllib.parse
import logging
from longportwhale.openapi import Config, HttpClient, TradeContext, PushOrderChanged, TopicType

logger = logging.getLogger("whale")
logging.basicConfig(level=logging.DEBUG)

event = threading.Event()

conf = Config.from_env()
httpcli = HttpClient.from_env()
tctx = TradeContext(conf)

test_account = "L6VQEU00121996"

def main():
    tctx.set_on_order_changed(on_order)
    tctx.subscribe([TopicType.Private])
    # create_member()
    do_trade()
    query_asset()

    while not event.is_set():
        time.sleep(1)


def on_order(order_ev: PushOrderChanged):
    logger.info("get order status changed event: {}".format(order_ev))


def create_member():
    req = {
        # open id is your system member uniq id
        # we will binding open id to longport system member id
        "open_id": f"whale_python_demo_{time.time()}"
    }
    res = httpcli.request(method="POST", path="/v1/whaleapi/auth/open_id/register", body=req, headers = {
        # can set language by accept-language header
        "Accept-Language": "zh-CN"
    })
    logger.info("create member response: {}".format(res))

def query_asset():
    req = {
        "account_no": test_account,
        "currency": "HKD"
    }
    res = httpcli.request(method="POST", path="/v1/whaleapi/asset/detail_info", body=req, headers = {
        "Accept-Language": "zh-HK"
    })
    logger.info("query user asset response: {}".format(res))


def do_trade():
    req = {
        "symbol": "700.HK",
        "order_type": "MO",  # Market Order
        "side": "Buy",
        "submitted_quantity": "100",
        "time_in_force": "Day",
        "account_no": test_account  # Test Account Number
    }

    logger.info("start calling submit order")
    submit_res = httpcli.request(method="POST", path="/v1/whaleapi/trade/order", body=req, headers = {})
    logger.info("submit order response: {}".format(submit_res))

    # get order detail
    order_detail_req = {
        "order_id": submit_res["order_id"],
        "account_no": test_account
    }
    # query params should do url encode
    req_qs = urllib.parse.urlencode(order_detail_req)
    order_detail_res = httpcli.request(method="GET", path="/v1/whaleapi/trade/order?" + req_qs, headers = {
        "Accept-Language": "zh-CN"
    })
    logger.info("order detail: {}".format(order_detail_res))


main()

event.wait()
