use clap::{App, Arg};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::Value;
use futures_util::stream::StreamExt;
use std::fs;



#[tokio::main]
async fn main() {
    let matches = App::new("simple")
        .arg(Arg::with_name("mode").long("mode").takes_value(true))
        .arg(Arg::with_name("times").long("times").takes_value(true))
        .get_matches();

    match matches.value_of("mode").unwrap() {
        "cache" => {
            let times = matches.value_of("times").unwrap_or("10").parse::<u32>().unwrap();
            cache_mode(times).await;
        },
        "read" => read_mode(),
        _ => println!("Invalid mode"),
    }
}

async fn cache_mode(times: u32) {
    let url = url::Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade").unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

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

    let average_price = prices.iter().sum::<f64>() / prices.len() as f64;
    fs::write("data.txt", format!("{}, {:?}", average_price, prices)).expect("Unable to write to file");
    println!("Cache complete. The average USD price of BTC is: {}", average_price);
}

fn read_mode() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    println!("Data: {}", data);
}

fn extract_price(data: &str) -> Result<f64, ()> {
    let parsed: Value = serde_json::from_str(data).map_err(|_| ())?;
    parsed["p"].as_str()
        .and_then(|price| price.parse::<f64>().ok())
        .ok_or(())
}
