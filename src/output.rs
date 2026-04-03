use std::time::Duration;
use serde_json::json;
use crate::bench::{CpuResult, RamResult, DiskResult, NetResult};
use crate::hardware::HardwareInfo;
use crate::score::Scores;

pub fn to_json(
    hw: &HardwareInfo,
    cpu: &CpuResult,
    ram: &RamResult,
    disk: &DiskResult,
    net: &NetResult,
    scores: &Scores,
    recommendations: &[String],
    elapsed: Duration,
) -> String {
    let output = json!({
        "version": "0.3.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "host": hw.hostname,
        "elapsed_seconds": elapsed.as_secs_f64(),
        "hardware": {
            "cpu_model": hw.cpu_model,
            "cpu_cores": hw.cpu_cores,
            "ram_gb": hw.ram_total as f64 / 1024.0 / 1024.0 / 1024.0,
            "disk_type": hw.disk_type,
        },
        "benchmarks": {
            "cpu": {
                "aes_gb_s": cpu.aes_gb_s,
                "sha_mb_s": cpu.sha_mb_s,
                "gflops": cpu.gflops,
                "score": scores.cpu,
            },
            "ram": {
                "latency_ns": ram.latency_ns,
                "throughput_gb_s": ram.throughput_gb_s,
                "score": scores.ram,
            },
            "disk": {
                "seq_mb_s": disk.seq_mb_s,
                "random_iops": disk.random_iops,
                "score": scores.disk,
            },
            "network": {
                "latency_ms": net.latency_ms,
                "download_mbps": net.download_mbps,
                "score": scores.net,
            },
        },
        "total_score": scores.total,
        "recommendations": recommendations,
    });
    
    serde_json::to_string_pretty(&output).unwrap()
}

pub fn to_console(
    _hw: &HardwareInfo,
    cpu: &CpuResult,
    ram: &RamResult,
    disk: &DiskResult,
    net: &NetResult,
    scores: &Scores,
    recommendations: &[String],
    elapsed: Duration,
    verbose: bool,
) {
    println!("\x1b[1;35m┌─────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;35m│                   RESULTS                   │\x1b[0m");
    println!("\x1b[1;35m└─────────────────────────────────────────────┘\x1b[0m\n");
    
    let total_color = match scores.total {
        0..=49 => "\x1b[31m",
        50..=69 => "\x1b[33m",
        70..=84 => "\x1b[32m",
        _ => "\x1b[1;32m",
    };
    
    println!("{} TOTAL SCORE: {}/100\x1b[0m\n", total_color, scores.total);
    
    println!("\x1b[1;36mCPU:\x1b[0m");
    println!("  AES-256:        \x1b[32m{:>8.2} GB/s\x1b[0m  Score: {}/100", cpu.aes_gb_s, scores.cpu);
    println!("  SHA-256:        \x1b[32m{:>8.0} MB/s\x1b[0m", cpu.sha_mb_s);
    println!("  Matrix Mul:     \x1b[32m{:>8.1} GFLOPS\x1b[0m\n", cpu.gflops);
    
    println!("\x1b[1;36mRAM:\x1b[0m");
    println!("  Latency:        \x1b[32m{:>8.0} ns\x1b[0m   Score: {}/100", ram.latency_ns, scores.ram);
    if ram.throughput_gb_s > 0.0 {
        println!("  Throughput:     \x1b[32m{:>8.1} GB/s\x1b[0m\n", ram.throughput_gb_s);
    } else {
        println!("  Throughput:     \x1b[33m{:>8}\x1b[0m\n", "N/A");
    }
    
    println!("\x1b[1;36mDISK:\x1b[0m");
    println!("  Sequential:     \x1b[32m{:>8.0} MB/s\x1b[0m  Score: {}/100", disk.seq_mb_s, scores.disk);
    println!("  Random 4K:      \x1b[32m{:>8.0} IOPS\x1b[0m\n", disk.random_iops);
    
    println!("\x1b[1;36mNETWORK:\x1b[0m");
    println!("  Latency (1.1.1.1): \x1b[32m{:>8.0} ms\x1b[0m  Score: {}/100", net.latency_ms, scores.net);
    if net.download_mbps > 0.0 {
        println!("  Download:          \x1b[32m{:>8.0} Mbps\x1b[0m\n", net.download_mbps);
    } else {
        println!("  Download:          \x1b[33m{:>8}\x1b[0m\n", "FAILED");
    }
    
    println!("\x1b[1;35m┌─────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;35m│               RECOMMENDATIONS               │\x1b[0m");
    println!("\x1b[1;35m└─────────────────────────────────────────────┘\x1b[0m\n");
    
    for rec in recommendations {
        println!("  {}", rec);
    }
    
    println!("\n\x1b[1;32m✅ Benchmark completed in {:.1} seconds\x1b[0m\n", elapsed.as_secs_f64());
    
    if verbose {
        println!("\x1b[1;33mVerbose output enabled - additional metrics:\x1b[0m\n");
        println!("  RAM available:  {:.2} GB", ram.available_bytes as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("  Disk used:      {:.2} GB", disk.used_bytes as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("  Network max:    {} Mbps", net.max_mbps);
    }
}