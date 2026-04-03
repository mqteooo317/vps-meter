use crate::bench::{CpuResult, RamResult, DiskResult, NetResult};
use crate::score::Scores;

pub fn generate(
    cpu: &CpuResult,
    ram: &RamResult,
    disk: &DiskResult,
    net: &NetResult,
    scores: &Scores,
) -> Vec<String> {
    let mut recs = Vec::new();
    
    if cpu.aes_gb_s < 0.5 {
        recs.push("⚠️ CPU lacks AES-NI. Consider upgrading VPS for encryption workloads.".to_string());
    } else if cpu.aes_gb_s > 2.0 {
        recs.push("✅ CPU AES performance is excellent.".to_string());
    }
    
    if cpu.sha_mb_s < 200.0 {
        recs.push("⚠️ CPU is underpowered for SHA operations. Consider more cores.".to_string());
    }
    
    if ram.latency_ns > 100.0 {
        recs.push("⚠️ High memory latency. Check NUMA configuration or upgrade RAM.".to_string());
    } else if ram.latency_ns < 60.0 {
        recs.push("✅ RAM latency is excellent.".to_string());
    }
    
    if disk.seq_mb_s < 100.0 {
        recs.push("❌ Disk is very slow (possibly HDD). Upgrade to SSD/NVMe immediately.".to_string());
    } else if disk.seq_mb_s < 300.0 {
        recs.push("⚠️ Disk sequential speed is moderate. Consider NVMe upgrade.".to_string());
    } else if disk.seq_mb_s > 800.0 {
        recs.push("✅ Disk sequential speed is excellent.".to_string());
    }
    
    if disk.random_iops < 5000.0 {
        recs.push("❌ Terrible random I/O. Your disk cannot handle databases. Upgrade to NVMe.".to_string());
    } else if disk.random_iops < 20000.0 {
        recs.push("⚠️ Random I/O is low. Consider NVMe for better performance.".to_string());
    } else if disk.random_iops > 80000.0 {
        recs.push("✅ Random I/O is excellent. Good for databases.".to_string());
    }
    
    if net.latency_ms > 100.0 {
        recs.push("⚠️ High network latency. Consider a provider closer to your users.".to_string());
    } else if net.latency_ms < 20.0 {
        recs.push("✅ Network latency is low. Good for real-time applications.".to_string());
    }
    
    if net.download_mbps < 50.0 && net.download_mbps > 0.0 {
        recs.push("⚠️ Network bandwidth is limited. Check your plan limits.".to_string());
    } else if net.download_mbps > 500.0 {
        recs.push("✅ Network bandwidth is excellent.".to_string());
    }
    
    if net.download_mbps == 0.0 {
        recs.push("⚠️ Network download test failed. Check internet connectivity or firewall.".to_string());
    }
    
    match scores.total {
        0..=49 => recs.push("🔴 VPS underperforming significantly. Consider migrating providers or upgrading plan.".to_string()),
        50..=69 => recs.push("🟡 Acceptable for basic hosting. Optimize for better performance.".to_string()),
        70..=84 => recs.push("🟢 Good performance for most workloads. Minor optimizations available.".to_string()),
        85..=100 => recs.push("🌟 Excellent performance. Your VPS is well optimized for demanding workloads.".to_string()),
        _ => {}
    }
    
    recs
}