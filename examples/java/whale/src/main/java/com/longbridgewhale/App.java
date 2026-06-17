package com.longbridgewhale;


import com.longportwhale.*;
import com.longportwhale.trade.PushOrderChanged;
import com.longportwhale.trade.TopicType;
import com.longportwhale.trade.TradeContext;

import java.util.HashMap;
import java.util.concurrent.CompletableFuture;
import java.io.UnsupportedEncodingException;
import java.net.URLEncoder;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class App 
{
    final static Logger logger = LoggerFactory.getLogger(App.class);

    static String testAccount = "L6VQEU00121996";

    public static void main( String[] args ) throws Exception
    {
        logger.info("key: {}, secret: {}", System.getenv("LONGPORT_APP_KEY"), System.getenv("LONGPORT_APP_SECRET"));


        try (Config conf = Config.fromEnv(); TradeContext tctx = TradeContext.create(conf).get(); HttpClient cli = HttpClient.fromEnv()) {
            testTrade(tctx, cli);
            testAsset(cli);
            // testMember(cli);
        }
    }

    private static void testMember(HttpClient cli) throws Exception 
    {
        HashMap<String, Object> registerReq = new HashMap<String, Object>();
        // change open_id
        registerReq.put("open_id", "test_register");
        CompletableFuture<HashMap> registerF = cli.request(HashMap.class, "POST", "/v1/whaleapi/auth/open_id/register", registerReq);
        HashMap<String, Object> registerRes = registerF.get();
        logger.info("query register response: {}", registerRes);
    }
 
    private static void testAsset(HttpClient cli) throws Exception 
    {
        HashMap<String, Object> queryTotalAssetReq = new HashMap<String, Object>();
        queryTotalAssetReq.put("account_no", testAccount);
        queryTotalAssetReq.put("currency", "HKD");
        CompletableFuture<HashMap> queryTotalAssetF = cli.request(HashMap.class, "POST", "/v1/whaleapi/asset/detail_info", queryTotalAssetReq);
        HashMap<String, Object> queryTotalAssetRes = queryTotalAssetF.get();
        logger.info("query total asset response: {}", queryTotalAssetRes);
    }
 
    private static void testTrade(TradeContext tctx, HttpClient cli) throws Exception 
    {
        // trade context create a websocket connection
        // use trade context to subscribe order change events
        tctx.subscribe(new TopicType[] { TopicType.Private});
        tctx.setOnOrderChange((PushOrderChanged orderEvent) -> {
            logger.info("receive order change event: {}", orderEvent);
        });

        // use http client to invoke whale api
        // do sumit order
        HashMap<String, Object> req = new HashMap<String, Object>();
        req.put("symbol", "700.HK");
        req.put("order_type", "MO"); // Market Order
        req.put("side", "Buy");
        req.put("submitted_quantity", "100");
        req.put("time_in_force", "Day");
        req.put("account_no", testAccount); // Test Account Number

        logger.info("start calling submit order");
        CompletableFuture<HashMap> submitF = cli.request(HashMap.class, "POST", "/v1/whaleapi/trade/order", req);
        HashMap<String, Object> submitRes = submitF.get();
        logger.info("submit order response: {}", submitRes);
        // wait for receive order change events
        Thread.sleep(5000);

        // get order detail
        HashMap<String, String> orderDetailReq = new HashMap<String, String>();
        orderDetailReq.put("order_id", submitRes.get("order_id").toString());
        orderDetailReq.put("account_no", testAccount);
        CompletableFuture<HashMap> orderDetailF = cli.request(HashMap.class, "GET", "/v1/whaleapi/trade/order?" + App.mapToQueryString(orderDetailReq), null);
        HashMap<String, Object> orderDetailRes = orderDetailF.get();
        logger.info("order detail: {}", orderDetailRes);


        // get user today order
        HashMap<String, String> todayOrdersReq = new HashMap<String, String>();
        todayOrdersReq.put("account_no", testAccount);
        logger.info("start calling get today orders");
        CompletableFuture<HashMap> todayOrdersF = cli.request(HashMap.class, "GET", "/v1/whaleapi/trade/order/today?" + App.mapToQueryString(todayOrdersReq), null);
        HashMap<String, Object> todayOrdersRes = todayOrdersF.get();
        logger.info("today order response: {}", todayOrdersRes);
           
        // get user history order
        HashMap<String, String> historyOrderReq = new HashMap<String, String>();
        historyOrderReq.put("account_no", testAccount);
        logger.info("start calling get history orders");
        CompletableFuture<HashMap> historyOrderF = cli.request(HashMap.class, "GET", "/v1/whaleapi/trade/order/history?" + App.mapToQueryString(historyOrderReq), null);
        HashMap<String, Object> historyOrderRes = historyOrderF.get();
        logger.info("history order response: {}", historyOrderRes);
    }

    // convert haspmap to query string 
    private static String mapToQueryString(Map<String, String> params) throws UnsupportedEncodingException 
    {
        StringBuilder queryString = new StringBuilder();

        for (Map.Entry<String, String> entry : params.entrySet()) {
            if (queryString.length() > 0) {
                queryString.append("&");
            }
            queryString.append(URLEncoder.encode(entry.getKey(), "UTF-8"));
            queryString.append("=");
            queryString.append(URLEncoder.encode(entry.getValue(), "UTF-8"));
        }

        return queryString.toString();
    }
}

