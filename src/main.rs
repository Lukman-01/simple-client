 
// use clap::{App, Arg};
// use serde_json::Value;
// use std::error::Error;
// use std::fs::OpenOptions;
// use std::io::{Write, BufReader, BufRead};
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
// use futures_util::stream::StreamExt;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let matches = App::new("simple")
//         .arg(Arg::with_name("mode").long("mode").takes_value(true).required(true))
//         .arg(Arg::with_name("times").long("times").takes_value(true))
//         .get_matches();

//     match matches.value_of("mode").unwrap() {
//         "cache" => {
//             let times = matches.value_of("times").unwrap_or("10").parse::<u32>()?;
//             cache_mode(times).await?;
//         },
//         "read" => read_mode()?,
//         _ => println!("Invalid mode"),
//     }

//     Ok(())
// }

// async fn cache_mode(times: u32) -> Result<(), Box<dyn Error>> {
//     let url = url::Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade")?;
//     let (mut ws_stream, _) = connect_async(url).await?;

//     let mut prices = Vec::new();
//     for _ in 0..times {
//         if let Some(message) = ws_stream.next().await {
//             if let Ok(Message::Text(text)) = message {
//                 if let Ok(price) = extract_price(&text) {
//                     prices.push(price);
//                 }
//             }
//         }
//     }

//     let average_price = prices.iter().sum::<f64>() / prices.len() as f64;

//     // Append data to the file with a timestamp
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("data.txt")?;
//     writeln!(file, "Average: {}, Prices: {:?}", average_price, prices)?;

//     println!("Cache complete. The average USD price of BTC is: {}", average_price);

//     Ok(())
// }

// fn read_mode() -> Result<(), Box<dyn Error>> {
//     let file = OpenOptions::new().read(true).open("data.txt")?;
//     let reader = BufReader::new(file);
//     for line in reader.lines() {
//         println!("{}", line?);
//     }
//     Ok(())
// }

// fn extract_price(data: &str) -> Result<f64, ()> {
//     let parsed: Value = serde_json::from_str(data).map_err(|_| ())?;
//     parsed["p"]
//         .as_str()
//         .and_then(|price| price.parse::<f64>().ok())
//         .ok_or(())
// }


mod cache_read;
mod compute_average;

use clap::{App, Arg};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Rust WebSocket Client")
        .version("1.0")
        .author("Your Name")
        .about("Connects to WebSocket and processes data")
        .arg(
            Arg::with_name("mode")
                .long("mode")
                .takes_value(true)
                .required(true)
                .help("Mode of operation: 'cache' or 'average'"),
        )
        .arg(
            Arg::with_name("times")
                .long("times")
                .takes_value(true)
                .help("Number of times to fetch data in 'cache' mode"),
        )
        .get_matches();

    match matches.value_of("mode").unwrap() {
        "cache" => {
            let times = matches
                .value_of("times")
                .unwrap_or("10")
                .parse::<u32>()
                .unwrap_or(10);
            cache_read::run_cache_mode(times).await?;
        }
        "average" => {
            compute_average::run_compute_average()?;
        }
        _ => println!("Invalid mode. Use 'cache' or 'average'."),
    }

    Ok(())
}
