use longportwhale::{
    Config, TradeContext,
    trade::{TopicType, PushEvent},
    httpclient::{HttpClient, Method, Json},
};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;

#[derive(Debug, Serialize)]
struct SubmitOrderRequest {
    symbol: String,
    order_type: String,
    side: String,
    submitted_quantity: String,
    time_in_force: String,
    account_no: String,
}

#[derive(Debug, Deserialize)]
struct SubmitOrderResponse {
    order_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting verification...");

    // Load config from environment variables
    let config = Arc::new(Config::from_env()?);
    println!("Config loaded successfully.");

    // Initialize TradeContext
    let (ctx, mut receiver) = TradeContext::try_new(config).await?;
    println!("TradeContext initialized.");

    // Subscribe to private events
    ctx.subscribe([TopicType::Private]).await?;
    println!("Subscribed to Private events successfully.");

    // Initialize HttpClient
    let http_cli = HttpClient::from_env()?;
    println!("HttpClient initialized.");

    // Spawn a task to listen for events
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    
    tokio::spawn(async move {
        while let Some(event) = receiver.recv().await {
            println!("Received event: {:?}", event);
            if let PushEvent::OrderChanged(order_changed) = event {
                let _ = event_tx.send(order_changed);
            }
        }
    });

    // Submit order
    let req = SubmitOrderRequest {
        symbol: "700.HK".to_string(),
        order_type: "MO".to_string(),
        side: "Buy".to_string(),
        submitted_quantity: "100".to_string(),
        time_in_force: "Day".to_string(),
        account_no: "L6VQEU00121996".to_string(),
    };

    println!("Submitting order: {:?}", req);
    let resp: SubmitOrderResponse = http_cli.request(Method::POST, "/v1/whaleapi/trade/order")
        .body(Json(req))
        .response::<Json<SubmitOrderResponse>>()
        .send()
        .await?
        .0;
    
    println!("Order submitted successfully. Order ID: {}", resp.order_id);

    // Wait for event
    println!("Waiting for order event...");
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(30));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(order_event) = event_rx.recv() => {
                if order_event.order_id == resp.order_id {
                    println!("Received expected order event: {:?}", order_event);
                    break;
                } else {
                    println!("Received other order event: {:?}", order_event);
                }
            }
            _ = &mut timeout => {
                println!("Timeout waiting for order event.");
                return Err("Timeout waiting for order event".into());
            }
        }
    }

    println!("Verification successful!");

    Ok(())
}
