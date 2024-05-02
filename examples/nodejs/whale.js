import {EventEmitter} from 'node:events';
import qs from 'node:querystring';

import {  Config, TradeContext, HttpClient, TopicType } from 'longportwhale';
import pino from 'pino';

const logger = pino();
const eventEmitter = new EventEmitter();
const conf = Config.fromEnv();
const httpcli = HttpClient.fromEnv();


const test_account = "L6VQEU00121996";

async function main() {
    const tctx = await TradeContext.new(conf);
    tctx.setOnOrderChanged(onOrder);
    await tctx.subscribe([TopicType.Private]);
    // createMember();
    await doTrade();
    await queryAsset();

    eventEmitter.on('shutdown', () => {
        process.exit();
    });
}

function onOrder(err, ev) {
    if (!err) {
        logger.info(`get order status changed event: ${ev}`);
    } else {
        logger.error(err)
    }
}

async function createMember() {
    const req = {
        "open_id": `whale_node_demo_${Date.now()}`
    };
    const headers = {
        // set response language by accept-language header
        'accept-language': "zh-CN" // zh-CN 中文简体，zh-HK - 繁体，en - english
    }
    const res = await httpcli.request('POST', '/v1/whaleapi/auth/open_id/register', headers, req);
    logger.info(`create member response: ${JSON.stringify(res)}`);
}

async function queryAsset() {
    const req = {
        "account_no": test_account,
        "currency": "HKD"
    };
    
    const res = await httpcli.request('POST', '/v1/whaleapi/asset/detail_info', {}, req);
    logger.info(`query user asset response: ${JSON.stringify(res)}`);
}

async function doTrade() {
    const req = {
        "symbol": "700.HK",
        "order_type": "MO", // Market Order
        "side": "Buy",
        "submitted_quantity": "100",
        "time_in_force": "Day",
        "account_no": test_account // Test Account Number
    };

    logger.info("start calling submit order");
    const submitRes = await httpcli.request('POST', '/v1/whaleapi/trade/order', null, req);
    logger.info(`submit order response: ${JSON.stringify(submitRes)}`);

    // get order detail
    const orderDetailReq = {
        "order_id": submitRes.order_id,
        "account_no": test_account
    };
    const orderDetailRes = await httpcli.request('GET', `/v1/whaleapi/trade/order?${qs.stringify(orderDetailReq)}`);
    logger.info(`order detail: ${JSON.stringify(orderDetailRes)}`);
}

await main();
