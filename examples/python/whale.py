import time
import threading
import logging
import os
import sys
from longportwhale.openapi import Config, HttpClient, TradeContext, PushOrderChanged, TopicType

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger("verify_trade")

# Event to signal completion
completion_event = threading.Event()
order_id = None

def on_order(order_ev: PushOrderChanged):
    global order_id
    logger.info(f"Received order event: {order_ev}")
    if order_id and order_ev.order_id == order_id:
        logger.info("Received expected order event!")
        completion_event.set()

def main():
    global order_id
    
    logger.info("Starting verification...")

    # Load config
    try:
        conf = Config.from_env()
        httpcli = HttpClient.from_env()
    except Exception as e:
        logger.error(f"Failed to load config: {e}")
        sys.exit(1)

    # Initialize TradeContext
    tctx = TradeContext(conf)
    tctx.set_on_order_changed(on_order)
    
    # Subscribe
    logger.info("Subscribing to private events...")
    tctx.subscribe([TopicType.Private])
    time.sleep(2) # Wait for subscription

    # Test HTTP connection
    logger.info("Testing HTTP connection...")
    try:
        asset_req = {
            "account_no": test_account,
            "currency": "HKD"
        }
        asset_res = httpcli.request(method="POST", path="/v1/whaleapi/asset/detail_info", body=asset_req, headers={})
        logger.info(f"Asset query response: {asset_res}")
    except Exception as e:
        logger.error(f"Failed to query asset: {e}")
        # Continue to submit order to see if it persists

    # Submit Order
    test_account = "L6VQEU00121996"
    req = {
        "symbol": "700.HK",
        "order_type": "MO",
        "side": "Buy",
        "submitted_quantity": "100",
        "time_in_force": "Day",
        "account_no": test_account
    }

    logger.info(f"Submitting order: {req}")
    try:
        submit_res = httpcli.request(method="POST", path="/v1/whaleapi/trade/order", body=req, headers={})
        logger.info(f"Submit order response: {submit_res}")
        order_id = submit_res["order_id"]
    except Exception as e:
        logger.error(f"Failed to submit order: {e}")
        sys.exit(1)

    # Wait for event
    logger.info(f"Waiting for order event for order_id: {order_id}...")
    if completion_event.wait(timeout=60):
        logger.info("Verification successful!")
        sys.exit(0)
    else:
        logger.error("Timeout waiting for order event.")
        sys.exit(1)

if __name__ == "__main__":
    main()
