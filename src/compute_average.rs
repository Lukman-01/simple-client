use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde_json::Value;
use tokio::runtime::Runtime;
use futures_util::stream::StreamExt;

pub fn run_compute_average() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 60 - SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() % 60;

    let rt = Runtime::new()?;
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx_clone = tx.clone();
        let delay = start_time + i * 2;

        rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(delay)).await;
            println!("Starting client at {:?}", Instant::now());
            match simulate_read_and_compute_average().await {
                Ok(average) => {
                    tx_clone.send(average).unwrap_or_else(|e| eprintln!("Failed to send: {}", e));
                }
                Err(e) => {
                    eprintln!("Task failed: {}", e);
                }
            }
        });
    }

    drop(tx);

    let mut results = Vec::new();
    for received in rx {
        results.push(received);
    }

    if !results.is_empty() {
        let final_average = results.iter().sum::<f64>() / results.len() as f64;
        println!("Final average: {}", final_average);
    } else {
        println!("No data received.");
    }

    Ok(())
}

async fn simulate_read_and_compute_average() -> Result<f64, Box<dyn std::error::Error>> {
    let ws_url = "wss://ws-api.binance.com:9443/ws/v3/btcusdt@trade";
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    let mut prices = Vec::new();

    let start_time = Instant::now();
    while Instant::now() - start_time < Duration::from_secs(10) {
        if let Some(message) = ws_stream.next().await {
            match message {
                Ok(WsMessage::Text(text)) => {
                    if let Ok(price) = extract_price(&text) {
                        prices.push(price);
                    }
                }
                _ => {}
            }
        }
    }

    let average_price = if !prices.is_empty() {
        prices.iter().sum::<f64>() / prices.len() as f64
    } else {
        0.0
    };

    Ok(average_price)
}

fn extract_price(data: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let parsed: Value = serde_json::from_str(data)?;
    if let Some(price) = parsed["p"].as_str().and_then(|p| p.parse::<f64>().ok()) {
        Ok(price)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract price",
        )))
    }
}
