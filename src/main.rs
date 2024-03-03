use futures_util::{SinkExt, StreamExt};
use serde_json::{json, to_string, Value};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::process::exit;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() {
    // measure how long it takes to get the snapshot
    let mut start = Instant::now();

    // Create a HTTPS client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut total_duration = 0.0; // Variable to accumulate total duration
    let snapshot_url = "https://api.bybit.com/v5/market/time";
    let uri: Uri = snapshot_url.parse().unwrap();

    let response = client.get(Uri::try_from(&uri).unwrap()).await.unwrap();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    println!("{:?}", body);

    let iterations = 50; // Number of iterations
    // also find minimum value
    let mut min_duration = 10e10;
    for _ in 0..iterations{
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_nanos();

        let mut response = client.get(Uri::try_from(&uri).unwrap()).await.unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        // Convert the response body to a string
        let snapshot = String::from_utf8(body.to_vec()).unwrap();

        // Parse the JSON response
        let parsed_snapshot: Value = serde_json::from_str(&snapshot).unwrap();

        // Extract the timestamp and convert it to a u128 integer
        let timestamp = parsed_snapshot["result"]["timeNano"]
            .as_str()
            .unwrap()
            .parse::<u128>()
            .unwrap();

        // Calculate the duration
        let duration = timestamp - start;
        // convert to float
        let duration = duration as f64;
        if duration < min_duration {
            min_duration = duration;
        }

        total_duration += duration; // Accumulate the duration

        println!("Received snapshot: {:?}", timestamp);
        println!("Time elapsed in nano seconds is: {:?}", duration);
        println!("Minimum duration: {:?}", min_duration);
    }

    // Calculate the average duration
    let average_duration  = total_duration / iterations as f64; // Assuming you performed 10 iterations

    println!("Average duration: {:?}", average_duration);
}

