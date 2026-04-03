use crate::bench::{CpuResult, RamResult, DiskResult, NetResult};

pub struct Scores {
    pub cpu: u8,
    pub ram: u8,
    pub disk: u8,
    pub net: u8,
    pub total: u8,
}

pub fn calculate_all(
    cpu: &CpuResult,
    ram: &RamResult,
    disk: &DiskResult,
    net: &NetResult,
) -> Scores {
    let cpu_score = calculate_cpu_score(cpu);
    let ram_score = calculate_ram_score(ram);
    let disk_score = calculate_disk_score(disk);
    let net_score = calculate_net_score(net);
    
    let total = ((cpu_score as f64 * 0.35)
        + (ram_score as f64 * 0.20)
        + (disk_score as f64 * 0.25)
        + (net_score as f64 * 0.20)) as u8;
    
    Scores {
        cpu: cpu_score,
        ram: ram_score,
        disk: disk_score,
        net: net_score,
        total,
    }
}

fn calculate_cpu_score(cpu: &CpuResult) -> u8 {
    let aes_score = (cpu.aes_gb_s / 5.0 * 100.0).min(100.0);
    let sha_score = (cpu.sha_mb_s / 1000.0 * 100.0).min(100.0);
    let gflops_score = (cpu.gflops / 50.0 * 100.0).min(100.0);
    
    let avg = (aes_score + sha_score + gflops_score) / 3.0;
    avg as u8
}

fn calculate_ram_score(ram: &RamResult) -> u8 {
    let latency_score = if ram.latency_ns <= 50.0 {
        100.0
    } else if ram.latency_ns >= 150.0 {
        0.0
    } else {
        (100.0 - (ram.latency_ns - 50.0)).max(0.0)
    };
    
    let throughput_score = if ram.throughput_gb_s >= 50.0 {
        100.0
    } else if ram.throughput_gb_s <= 10.0 {
        0.0
    } else {
        ((ram.throughput_gb_s - 10.0) / 40.0 * 100.0).min(100.0)
    };
    
    ((latency_score + throughput_score) / 2.0) as u8
}

fn calculate_disk_score(disk: &DiskResult) -> u8 {
    let seq_score = if disk.seq_mb_s >= 1000.0 {
        100.0
    } else if disk.seq_mb_s <= 200.0 {
        0.0
    } else {
        ((disk.seq_mb_s - 200.0) / 800.0 * 100.0).min(100.0)
    };
    
    let iops_score = if disk.random_iops >= 100000.0 {
        100.0
    } else if disk.random_iops <= 10000.0 {
        0.0
    } else {
        ((disk.random_iops - 10000.0) / 90000.0 * 100.0).min(100.0)
    };
    
    ((seq_score + iops_score) / 2.0) as u8
}

fn calculate_net_score(net: &NetResult) -> u8 {
    let latency_score = if net.latency_ms <= 5.0 {
        100.0
    } else if net.latency_ms >= 100.0 {
        0.0
    } else {
        (100.0 - (net.latency_ms - 5.0)).max(0.0)
    };
    
    let download_score = if net.download_mbps >= 1000.0 {
        100.0
    } else if net.download_mbps <= 100.0 {
        0.0
    } else {
        ((net.download_mbps - 100.0) / 900.0 * 100.0).min(100.0)
    };
    
    ((latency_score + download_score) / 2.0) as u8
}