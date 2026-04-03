use std::time::{Duration, Instant};
use reqwest::blocking::Client;
use super::NetResult;

pub fn run() -> NetResult {
    let latency_ms = benchmark_latency();
    let (download_mbps, max_mbps) = benchmark_download();

    NetResult {
        latency_ms,
        download_mbps,
        max_mbps,
    }
}

fn benchmark_latency() -> f64 {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let start = Instant::now();
    let result = client.get("https://1.1.1.1").send();
    
    if result.is_err() {
        return 999.0;
    }
    
    let elapsed = start.elapsed();
    elapsed.as_secs_f64() * 1000.0
}

fn benchmark_download() -> (f64, u64) {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let url = "https://speed.cloudflare.com/__down?bytes=5242880";
    
    let start = Instant::now();
    let response = client.get(url).send();
    
    if response.is_err() {
        return (0.0, 0);
    }
    
    let response = response.unwrap();
    let bytes_result = response.bytes();
    
    if bytes_result.is_err() {
        return (0.0, 0);
    }
    
    let bytes = bytes_result.unwrap().len();
    let elapsed = start.elapsed();

    let mbps = (bytes as f64 * 8.0) / elapsed.as_secs_f64() / 1_000_000.0;
    (mbps, mbps.round() as u64)
}