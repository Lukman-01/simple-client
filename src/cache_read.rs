// cache_read.rs
use serde_json::Value;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;

pub async fn run_cache_mode(times: u32) -> Result<(), Box<dyn Error>> {
    cache_mode(times).await
}

async fn cache_mode(times: u32) -> Result<(), Box<dyn Error>> {
    let url = url::Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade")?;
    let (mut ws_stream, _) = connect_async(url).await?;

    let mut prices = Vec::new();
    for _ in 0..times {
        if let Some(message) = ws_stream.next().await {
            if let Ok(Message::Text(text)) = message {
                if let Ok(price) = extract_price(&text) {
                    prices.push(price);
                }
            }
        }
    }

    let average_price = if !prices.is_empty() {
        prices.iter().sum::<f64>() / prices.len() as f64
    } else {
        0.0
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.txt")?;
    writeln!(file, "Average: {}, Prices: {:?}", average_price, prices)?;

    println!("Cache complete. The average USD price of BTC is: {}", average_price);

    Ok(())
}

pub fn read_mode() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("data.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn extract_price(data: &str) -> Result<f64, Box<dyn Error>> {
    let parsed: Value = serde_json::from_str(data)?;
    parsed["p"]
        .as_str()
        .and_then(|price| price.parse::<f64>().ok())
        .ok_or_else(|| "Failed to parse price".into())
}
