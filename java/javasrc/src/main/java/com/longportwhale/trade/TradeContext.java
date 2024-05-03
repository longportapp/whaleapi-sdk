package com.longportwhale.trade;

import java.util.concurrent.CompletableFuture;

import com.longportwhale.*;

/**
 * Trade context
 */
public class TradeContext implements AutoCloseable {
    private long raw;

    /**
     * Create a TradeContext object
     * 
     * @param config Config object
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public static CompletableFuture<TradeContext> create(Config config)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.newTradeContext(config.getRaw(), callback);
        });
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeTradeContext(raw);
    }

    /**
     * Set order changed event callback, after receiving the order changed event, it
     * will call back to this handler.
     * 
     * @param handler A order changed handler
     */
    public void setOnOrderChange(OrderChangedHandler handler) {
        SdkNative.tradeContextSetOnOrderChanged(this.raw, handler);
    }

    /**
     * Subscribe
     * 
     * <pre>
     * {@code
     * import com.longportwhale.*;
     * import com.longportwhale.trade.*;
     * import java.math.BigDecimal;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             ctx.setOnOrderChange((order_changed) -> {
     *                 System.out.println(order_changed);
     *             });
     *             ctx.subscribe(new TopicType[] { TopicType.Private }).get();
     * 
     *             Thread.sleep(3000);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param topics Topics
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> subscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextSubscribe(this.raw, topics, callback);
        });
    }

    /**
     * Unsubscribe
     * 
     * @param topics Topics
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> unsubscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextUnsubscribe(this.raw, topics, callback);
        });
    }
}
