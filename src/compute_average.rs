// Imports necessary libraries and modules.
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde_json::Value;
use tokio::runtime::Runtime;
use futures_util::stream::StreamExt;

// Public function to run the average computation mode.
pub fn run_compute_average() -> Result<(), Box<dyn std::error::Error>> {
    // Calculate a start time based on the current time plus a fixed offset.
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + 60 - SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() % 60;

    // Create a new Tokio runtime for asynchronous operations.
    let rt = Runtime::new()?;
    // Create a multi-producer, single-consumer channel for communication between threads.
    let (tx, rx) = mpsc::channel();

    // Spawn multiple asynchronous tasks to compute averages.
    for i in 0..5 {
        let tx_clone = tx.clone(); // Clone the sender part of the channel for the new task.
        let delay = start_time + i * 2; // Calculate delay for each task.

        // Spawn an asynchronous task.
        rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(delay)).await;
            println!("Starting client at {:?}", Instant::now());
            match simulate_read_and_compute_average().await {
                Ok(average) => {
                    // Send the computed average back to the main thread.
                    tx_clone.send(average).unwrap_or_else(|e| eprintln!("Failed to send: {}", e));
                }
                Err(e) => {
                    // Log any errors that occur in the task.
                    eprintln!("Task failed: {}", e);
                }
            }
        });
    }

    // Drop the original sender to allow the channel to close once all tasks are complete.
    drop(tx);

    // Collect results from all asynchronous tasks.
    let mut results = Vec::new();
    for received in rx {
        results.push(received);
    }

    // Calculate and print the final average.
    if !results.is_empty() {
        let final_average = results.iter().sum::<f64>() / results.len() as f64;
        println!("Final average: {}", final_average);
    } else {
        println!("No data received.");
    }

    Ok(())
}

// Asynchronous function to simulate reading data and computing the average.
async fn simulate_read_and_compute_average() -> Result<f64, Box<dyn std::error::Error>> {
    // Connect to the WebSocket server.
    let ws_url = "wss://ws-api.binance.com:9443/ws/v3/btcusdt@trade";
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    let mut prices = Vec::new();

    // Collect data for a fixed duration.
    let start_time = Instant::now();
    while Instant::now() - start_time < Duration::from_secs(10) {
        // Process each WebSocket message.
        if let Some(message) = ws_stream.next().await {
            match message {
                Ok(WsMessage::Text(text)) => {
                    // Extract and store the price from the message.
                    if let Ok(price) = extract_price(&text) {
                        prices.push(price);
                    }
                }
                _ => {} // Ignore other types of messages.
            }
        }
    }

    // Calculate and return the average price.
    let average_price = if !prices.is_empty() {
        prices.iter().sum::<f64>() / prices.len() as f64
    } else {
        0.0
    };

    Ok(average_price)
}

// Function to extract the price from a JSON string.
fn extract_price(data: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Parse the JSON data.
    let parsed: Value = serde_json::from_str(data)?;
    // Extract the price and convert it to a floating-point number.
    if let Some(price) = parsed["p"].as_str().and_then(|p| p.parse::<f64>().ok()) {
        Ok(price)
    } else {
        // Return an error if the price cannot be extracted.
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract price",
        )))
    }
}
