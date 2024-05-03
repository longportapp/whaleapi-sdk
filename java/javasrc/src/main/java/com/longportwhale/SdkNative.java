package com.longportwhale;

import java.io.IOException;
import java.time.LocalDate;
import org.scijava.nativelib.NativeLoader;

import com.longportwhale.trade.*;

import java.time.OffsetDateTime;
import java.time.LocalDateTime;

/**
 * @hidden
 */
public class SdkNative {
        static native void init();

        public static native long newHttpClient(String httpUrl, String appKey, String appSecret, String accessToken);

        public static native long newHttpClientFromEnv();

        public static native void freeHttpClient(long httpClient);

        public static native void httpClientRequest(long httpClient, String request, AsyncCallback callback);

        public static native long newConfig(String appKey, String appSecret, String accessToken, String httpUrl,
                        String tradeWsUrl, Language language);

        public static native long newConfigFromEnv();

        public static native void configRefreshAccessToken(long config, OffsetDateTime expired_at,
                        AsyncCallback callback);

        public static native void freeConfig(long config);

        public static native void newTradeContext(long config, AsyncCallback callback);

        public static native void freeTradeContext(long config);

        public static native void tradeContextSetOnOrderChanged(long context, OrderChangedHandler handler);

        public static native void tradeContextSubscribe(long context, TopicType[] topics, AsyncCallback callback);

        public static native void tradeContextUnsubscribe(long context, TopicType[] topics, AsyncCallback callback);

        static {
                try {
                        NativeLoader.loadLibrary("longportwhale_java");
                } catch (IOException e) {
                        System.out.println("======================================");
                        System.out.println("Failed to load longportwhale_java");
                        e.printStackTrace();
                        System.out.println("======================================");
                        System.load("longportwhale_java");
                } finally {
                        SdkNative.init();
                }
        }
}
