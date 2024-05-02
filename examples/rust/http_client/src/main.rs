use std::env;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::Notify;
use tracing_subscriber::EnvFilter;
use longportwhale::{Config, TradeContext};
use longportwhale::trade::TopicType;
use longportwhale::httpclient::{HttpClient, Json, Method};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    tokio::spawn(async move {
        notify_clone.notified().await;
    });

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env().expect("load config from config failed");
    let http_cli = config.create_http_client();

    let (ctx, mut receiver) = TradeContext::try_new(Arc::new(config)).await?;
    ctx.subscribe(vec![TopicType::Private]).await?;

    let test_account = env::var("TEST_ACCOUNT")
        .unwrap_or_else(|_| "L6VQEU00121996".into());

    tokio::spawn(async move {
        while let Some(event) = receiver.recv().await {
            println!("{:?}", event);
        }
    });


    do_trade(&http_cli, &test_account).await;
    query_asset(&http_cli, &test_account).await;

    notify.notified().await;
    Ok(())
}

#[derive(Debug, Serialize, Clone)]
struct Trade {
    symbol: String,
    order_type: String,
    side: String,
    submitted_quantity: String,
    time_in_force: String,
    account_no: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TradeResponse {
    order_id: String,
}

#[derive(Debug, Serialize, Clone)]
struct OrderQuery {
    order_id: String, 
    account_no: String,
}


async fn do_trade(client: &HttpClient, test_account: &str) {
    let req = Trade {
        symbol: "700.HK".to_string(),
        order_type: "MO".to_string(),
        side: "Buy".to_string(),
        submitted_quantity: "100".to_string(),
        time_in_force: "Day".to_string(),
        account_no: test_account.to_string(),
    };

    let res = client
        .request(Method::POST, "/v1/whaleapi/trade/order")
        .body(Json(req))
        .response::<Json<TradeResponse>>()
        .header("accept-language", "zh-CN")
        .send()
        .await
        .expect("Failed to send request");

    println!("submit order response: {:?}", res.0);

    let q = OrderQuery {
        order_id: res.0.order_id.to_string(),
        account_no: test_account.to_string(),
    };
    
    let detail = client
        .request(Method::GET, "/v1/whaleapi/trade/order")
        .query_params(q)
        .response::<String>()
        .send()
        .await
        .expect("failed to query order detail");

    println!("order detail: {}", detail);

}

#[derive(Debug, Serialize, Clone)]
struct AssetQuery {
    account_no: String,
    currency: String,
}

async fn query_asset(client: &HttpClient, test_account: &str) {
    let req = AssetQuery {
        account_no: test_account.to_string(),
        currency: "HKD".to_string(),
    };

    let res = client
        .request(Method::POST, "/v1/whaleapi/asset/detail_info")
        .body(Json(req))
        .response::<String>()
        .send()
        .await
        .expect("Failed to send request");

    println!("query user asset response: {}", res);
}
