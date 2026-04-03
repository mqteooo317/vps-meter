use std::fs;
use serde_json::Value;
use crate::bench::{CpuResult, DiskResult, NetResult};
use crate::score::Scores;

pub fn compare_with_baseline(
    cpu: &CpuResult,
    disk: &DiskResult,
    net: &NetResult,
    scores: &Scores,
    baseline_path: &str,
) -> Result<(), String> {
    let content = fs::read_to_string(baseline_path).map_err(|e| format!("Cannot read file: {}", e))?;
    let baseline: Value = serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?;

    println!("\n\x1b[1;35m┌─────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;35m│               COMPARISON MODE               │\x1b[0m");
    println!("\x1b[1;35m└─────────────────────────────────────────────┘\x1b[0m\n");

    let baseline_score = baseline["total_score"].as_u64().unwrap_or(0) as u8;
    let delta = scores.total as i16 - baseline_score as i16;
    let delta_color = if delta > 0 { "\x1b[32m" } else if delta < 0 { "\x1b[31m" } else { "\x1b[33m" };
    println!("  Total score:   {} → {} ({} {:+} points\x1b[0m)",
        baseline_score, scores.total, delta_color, delta);

    let baseline_aes = baseline["benchmarks"]["cpu"]["aes_gb_s"].as_f64().unwrap_or(0.0);
    let delta_aes = cpu.aes_gb_s - baseline_aes;
    let color = if delta_aes > 0.0 { "\x1b[32m" } else if delta_aes < 0.0 { "\x1b[31m" } else { "\x1b[33m" };
    println!("  CPU AES:       {:.2} → {:.2} GB/s ({} {:+} GB/s\x1b[0m)",
        baseline_aes, cpu.aes_gb_s, color, delta_aes);

    let baseline_seq = baseline["benchmarks"]["disk"]["seq_mb_s"].as_f64().unwrap_or(0.0);
    let delta_seq = disk.seq_mb_s - baseline_seq;
    let color = if delta_seq > 0.0 { "\x1b[32m" } else if delta_seq < 0.0 { "\x1b[31m" } else { "\x1b[33m" };
    println!("  Disk seq:      {:.0} → {:.0} MB/s ({} {:+} MB/s\x1b[0m)",
        baseline_seq, disk.seq_mb_s, color, delta_seq);

    let baseline_lat = baseline["benchmarks"]["network"]["latency_ms"].as_f64().unwrap_or(0.0);
    let delta_lat = net.latency_ms - baseline_lat;
    let color = if delta_lat < 0.0 { "\x1b[32m" } else if delta_lat > 0.0 { "\x1b[31m" } else { "\x1b[33m" };
    println!("  Net latency:   {:.0} → {:.0} ms ({} {:+} ms\x1b[0m)",
        baseline_lat, net.latency_ms, color, delta_lat);

    println!();
    Ok(())
}