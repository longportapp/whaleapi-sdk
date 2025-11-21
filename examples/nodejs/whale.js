import { Config, TradeContext, HttpClient, TopicType } from 'longportwhale';
import pino from 'pino';

const logger = pino();

const conf = Config.fromEnv();
const httpcli = HttpClient.fromEnv();
const test_account = "L6VQEU00121996";

let orderId = null;
let completed = false;

function onOrder(err, ev) {
    if (err) {
        logger.error(err);
        return;
    }

    // Use toString() for logging (recommended workaround)
    logger.info(`Received order event: ${ev.toString()}`);

    // Access properties directly (recommended for production)
    const eventOrderId = ev.orderId;
    const eventSymbol = ev.symbol;
    const eventStatus = ev.status;

    logger.info(`Order details - ID: ${eventOrderId}, Symbol: ${eventSymbol}, Status: ${eventStatus}`);

    // Check if it matches our order
    if (orderId && eventOrderId === orderId) {
        logger.info("✅ Received expected order event!");
        completed = true;
    }
}

async function main() {
    logger.info("Starting verification...");

    const tctx = await TradeContext.new(conf);
    tctx.setOnOrderChanged(onOrder);

    logger.info("Subscribing to private events...");
    await tctx.subscribe([TopicType.Private]);

    // Wait for subscription
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Submit Order
    const req = {
        "symbol": "700.HK",
        "order_type": "MO",
        "side": "Buy",
        "submitted_quantity": "100",
        "time_in_force": "Day",
        "account_no": test_account
    };

    logger.info(`Submitting order: ${JSON.stringify(req)}`);
    try {
        const submitRes = await httpcli.request('POST', '/v1/whaleapi/trade/order', null, req);
        logger.info(`Submit order response: ${JSON.stringify(submitRes)}`);
        orderId = submitRes.order_id;
    } catch (e) {
        logger.error(`Failed to submit order: ${e}`);
        process.exit(1);
    }

    // Wait for event
    logger.info(`Waiting for order event for order_id: ${orderId}...`);
    const startTime = Date.now();
    while (!completed) {
        if (Date.now() - startTime > 60000) {
            logger.error("Timeout waiting for order event.");
            process.exit(1);
        }
        await new Promise(resolve => setTimeout(resolve, 1000));
    }

    logger.info("Verification successful!");
    process.exit(0);
}

main().catch(e => {
    logger.error(e);
    process.exit(1);
});
