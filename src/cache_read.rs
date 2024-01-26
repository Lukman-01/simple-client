// Imports necessary crates and modules.
use serde_json::Value;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;

// Public asynchronous function that is the entry point for the cache mode.
// It takes the number of times to fetch data as a parameter.
pub async fn run_cache_mode(times: u32) -> Result<(), Box<dyn Error>> {
    // Calls the cache_mode function with the number of times and awaits its result.
    cache_mode(times).await
}

// Private asynchronous function to handle cache mode logic.
async fn cache_mode(times: u32) -> Result<(), Box<dyn Error>> {
    // Parses the WebSocket URL and establishes a connection.
    let url = url::Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade")?;
    let (mut ws_stream, _) = connect_async(url).await?;

    // Vector to store the fetched prices.
    let mut prices = Vec::new();

    // Fetches data for the number of times specified.
    for _ in 0..times {
        if let Some(message) = ws_stream.next().await {
            // Processes each WebSocket message.
            if let Ok(Message::Text(text)) = message {
                // Attempts to extract price from the message text.
                if let Ok(price) = extract_price(&text) {
                    // Adds the extracted price to the prices vector.
                    prices.push(price);
                }
            }
        }
    }

    // Calculates the average price.
    let average_price = if !prices.is_empty() {
        prices.iter().sum::<f64>() / prices.len() as f64
    } else {
        0.0
    };

    // Opens "data.txt" for writing and appends the average price and all prices.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.txt")?;
    writeln!(file, "Average: {}, Prices: {:?}", average_price, prices)?;

    // Logs the completion of the cache process.
    println!("Cache complete. The average USD price of BTC is: {}", average_price);

    Ok(())
}

// Public function to read and display the content of "data.txt".
pub fn read_mode() -> Result<(), Box<dyn Error>> {
    // Opens "data.txt" in read mode.
    let file = OpenOptions::new().read(true).open("data.txt")?;
    let reader = BufReader::new(file);

    // Reads each line of the file and prints it.
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

// Function to extract the price from a string of data.
fn extract_price(data: &str) -> Result<f64, Box<dyn Error>> {
    // Parses the JSON data.
    let parsed: Value = serde_json::from_str(data)?;

    // Extracts the price value, parses it, and returns it.
    parsed["p"]
        .as_str()
        .and_then(|price| price.parse::<f64>().ok())
        .ok_or_else(|| "Failed to parse price".into())
}
